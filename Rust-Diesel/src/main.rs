#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use core::time;
use std::{env, thread::sleep};

fn main() {
    println!("Rust-Diesel docker image boilerplate!");

    let database_url = env::var("DATABASE_URL").unwrap();

    let mut mysql_connection = None;

    for i in 0..5 {
        match MysqlConnection::establish(&database_url) {
            Ok(conn) => {
                mysql_connection = Some(conn);
                break;
            },
            _ => {
                println!("({}) Failed to connect, retrying...", i);
                sleep(time::Duration::from_secs(5));
            }
        };
    }

    match mysql_connection {
        Some(_) => println!("Successfully connected to DB!"),
        None => println!("Couldn't connect to DB!")
    }
}