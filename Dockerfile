# Layer for cloning the Sapper frontend from Github
FROM node as sapper-clone
USER root
RUN sudo apt-get update
RUN sudo apt-get install -y git
USER node
RUN mkdir -p /home/node/app
WORKDIR /home/node/app
RUN git clone https://github.com/eksplein/website.git .

# Layer for building the Sapper frontend
FROM node as sapper-export

# Prepare some dummy Node project, just to compile and cache dependencies
USER node
RUN mkdir -p /home/node/app
WORKDIR /home/node/app
COPY --from=sapper-clone --chown=node:node /home/node/app/package.json .
RUN yarn

# Transpile the actual frontend source code, using cached dependencies
COPY --from=sapper-clone --chown=node:node /home/node/app .
RUN yarn export

# Layer for Rust server build
FROM ekidd/rust-musl-builder:latest as rust-build

# Prepare some dummy Rust project, just to compile and cache dependencies
USER root
RUN sudo mkdir -p /home/rust/src/src/
ADD --chown=rust:rust Cargo.toml /home/rust/src/
ADD --chown=rust:rust Cargo.lock /home/rust/src/
RUN sudo echo "fn main() {}" > /home/rust/src/src/main.rs

# Download and compile Cargo dependencies only
USER rust
RUN cargo build --release

# Compile the actual Rust server source code, using cached dependencies
ADD --chown=rust:rust . .
RUN cargo build --release

# The actual Docker image we want to release
FROM alpine:latest

# Copy artifacts from previous builds
COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/eksplein ./eksplein
COPY --from=rust-build /home/rust/src/static ./static
COPY --from=sapper-export /home/node/app/__sapper__/export ./dist

# Run the statically linked binary
ENTRYPOINT ["./eksplein"]