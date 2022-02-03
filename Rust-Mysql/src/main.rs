use core::time;
use mysql::Pool;
use std::{env, thread::sleep};

fn main() {
    println!("Rust-Mysql docker image boilerplate!");

    let database_url: String = env::var("DATABASE_URL").unwrap();

    let mut pool = None;

    // try to connect 5 times with 5s delay to compensate for mysql startup time
    for i in 0..5 {
        match Pool::new(database_url.clone()) {
            Ok(p) => {
                pool = Some(p);
                break;
            }
            _ => {
                println!("({}) Failed to connect, retrying...", i);
                sleep(time::Duration::from_secs(5));
            }
        }
    }

    match pool {
        Some(pool) => match pool.get_conn() {
            Ok(_) => println!("Successfully connected to DB!"),
            Err(e) => println!("{:?}", e),
        },

        None => println!("Couldn't connect to DB!"),
    }
}
