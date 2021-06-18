use core::time;
use mysql::Pool;
use std::{env, thread::sleep};

fn main() {
    println!("Rust-Mysql docker image boilerplate!");

    let USER: String = env::var("MYSQL_USER").unwrap();
    let PASS: String = env::var("MYSQL_PASSWORD").unwrap();
    let HOST: String = env::var("MYSQL_HOST").unwrap();
    let DB: String = env::var("MYSQL_DB").unwrap();

    let url = format!("mysql://{}:{}@{}:3306/{}", USER, PASS, HOST, DB);
    let mut pool = None;

    // try to connect 5 times with 5s delay to compensate for mysql startup time
    for i in 0..5 {
        match Pool::new(url.clone()) {
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
