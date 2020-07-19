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

use std::io;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{web, App, Error, error::ErrorNotFound, HttpRequest, HttpServer};
use redis::Commands;

use crate::schema::create_schema;
use crate::cli::Context;
use crate::juniper;

// Try to serve static files from frontend
async fn serve(req: HttpRequest, data: web::Data<Context>) -> Result<NamedFile, Error> {
    // Get frontend dist folder path from CLI context
    let static_dir: PathBuf = PathBuf::from(data.dist.clone());
    
    // Serve file if exists...
    let requested_filename: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut path = PathBuf::new();
    path.push(static_dir);
    path.push(requested_filename);
    if path.exists() && path.is_file() {
        return Ok(NamedFile::open(path)?)
    }

    // If not, try instead to look for {path}/index.html
    // This is meant for matching any 'index.html' file in a Sapper project
    let mut path_with_index_html = PathBuf::from(path);
    let index_html: PathBuf = PathBuf::from("index.html".to_string());
    path_with_index_html.push(index_html);
    if path_with_index_html.exists() {
        return Ok(NamedFile::open(path_with_index_html)?)
    }

    // If nothing worked, just return 404
    return Err(ErrorNotFound("not found"))
}

// Serve GraphQL Playground for any 'GET /graphql' request
async fn graphql_playground(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = PathBuf::from("static/playground.html".to_string());
    Ok(NamedFile::open(path)?)
}

// Just a sample Redis test in which we set 'my_key' value to 42 and try to retrieve it 
async fn fetch_an_integer_from_redis(context: Context) -> redis::RedisResult<isize> {
    let client = redis::Client::open(context.redis_uri.clone())?;
    let mut con = client.get_connection()?;
    let _ : () = con.set("my_key", 42)?;
    con.get("my_key")
}

#[actix_rt::main]
pub async fn run(context: Context) -> io::Result<()> {
    // Setup Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Test Redis
    let value = fetch_an_integer_from_redis(context.clone()).await;
    match value {
        Ok(value) => println!("{:#?}", value),
        Err(e) => panic!("Couldn\'t connect to Redis database : {:?}", e),
    }

    // Set up server port mapping
    let mut url = "localhost:".to_string();
    url.push_str(&context.port.clone());
    println!("Server is now running in : http://{}", url);

    // Set up the actual Actix server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(context.clone())
            .service(web::resource("/graphql")
                .route(web::post().to(juniper::handler))
                .route(web::get().to(graphql_playground)))
            .route("/", web::get().to(serve))
            .route("/{filename:.*}", web::get().to(serve))
    })
    .bind(url)?
    .run()
    .await
}