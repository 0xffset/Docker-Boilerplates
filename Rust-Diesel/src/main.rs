#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use core::time;
use diesel::mysql::*;
use diesel::prelude::*;
use std::{env, thread::sleep};

pub mod models;
pub mod schema;

embed_migrations!("./migrations");

fn main() {
    println!("Rust-Diesel docker image boilerplate!");

    let database_url = env::var("DATABASE_URL").unwrap();

    let mut diesel_connection = None;

    // try to connect 5 times with 5s delay to compensate for mysql startup time
    for i in 0..5 {
        match MysqlConnection::establish(&database_url) {
            Ok(conn) => {
                diesel_connection = Some(conn);
                break;
            }
            _ => {
                println!("({}) Failed to connect, retrying...", i);
                sleep(time::Duration::from_secs(5));
            }
        };
    }

    match diesel_connection {
        Some(conn) => {
            println!("Successfully connected to the DB!");

            //embedded_migrations::run(&conn);
            embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
            println!("Successfully ran the migrations!")
        }
        None => println!("Couldn't connect to DB!"),
    }
}
