/**
 * EKSPLEIN (/ɛkˈspleɪn/) is a simple and stupid glossary-like blog
 * in which things are explained 
 * Copyright (C) 2020  Tom Bazarnik and the contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::env;
use std::process;
use ezcli::{named_flag, named_option, name::Name};

#[derive(Clone)]
pub struct Context {
    pub dist: String,
    pub port: String,
    pub redis_uri: String,
}

// Parse any flag or option provided by the user
pub fn parse() -> Context {
    // Retrieve version from Cargo.toml
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Create a context with default values
    let mut context = Context {
        dist: "dist/".to_string(),
        port: 9494.to_string(),
        redis_uri: "redis://127.0.0.1/".to_string()
    };

    named_flag!(help, Name::new("help", "h"));
    named_flag!(version, Name::new("version", "v"));
    named_option!(dist, Name::new("dist", "d"));
    named_option!(port, Name::new("port", "p"));
    named_option!(redis_uri, Name::new("redis_uri", "u"));

    // Set up help usage guide
    let mut help_guide = "EKSPLEIN v".to_string();
    help_guide.push_str(VERSION);
    help_guide.push_str("\nusage: eksplein [-h] [-v] [-d PATH] [-p PORT] [-u URI]");
    help_guide.push_str("\n\n  --help, -h     —  Show CLI usage guide");
    help_guide.push_str("\n  --version, -v  —  Show CLI version");
    help_guide.push_str("\n  --dist, -d     —  Set frontend dist folder (default: dist/)");
    help_guide.push_str("\n  --port, -p     —  Set server port (default: 9494)");
    help_guide.push_str("\n  --uri, -u      —  Set Redis URI (default: redis://127.0.0.1/)");

    // If help requested, show the help usage guide and exit
    if help {
        println!("{}", help_guide);
        process::exit(0);
    }

    // If version requested, show the app version and exit
    if version {
        println!("EKSPLEIN v{}", VERSION);
        process::exit(0);
    }

    // If dist provided, set 'context.dist' value
    match dist {
        Some(x) => context.dist = x,
        None => {},
    }

    // If port provided, set 'context.port' value
    match port {
        Some(x) => context.port = x,
        None => {},
    }

    // If uri provided, set 'context.redis_uri' value
    match redis_uri {
        Some(x) => context.redis_uri = x,
        None => {},
    }

    // Return the computed context
    context
}