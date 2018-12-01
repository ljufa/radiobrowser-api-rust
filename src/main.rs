#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate url;
extern crate handlebars;

#[macro_use]
extern crate mysql;

use clap::{App, Arg};

use std::{thread, time};

mod api;
mod db;

fn main() {
    let matches = App::new("stream-check")
        .version(crate_version!())
        .author("segler_alex@web.de")
        .about("HTTP Rest API for radiobrowser")
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("DATABASE_URL")
                .help("Database connection url")
                .env("DATABASE_URL")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("listen_host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("listening host ip")
                .env("HOST")
                .default_value("127.0.0.1")
                .takes_value(true),
        ).arg(
            Arg::with_name("server_url")
                .short("s")
                .long("server_url")
                .value_name("SERVER_URL")
                .help("full server url that should be used in docs")
                .env("SERVER_URL")
                .default_value("localhost:8080")
                .takes_value(true),
        ).arg(
            Arg::with_name("listen_port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("listening port")
                .env("PORT")
                .default_value("8080")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("THREADS")
                .help("concurrent threads used by socket")
                .env("THREADS")
                .default_value("1")
                .takes_value(true),
        ).arg(
            Arg::with_name("update-caches-interval")
                .short("u")
                .long("update-caches-interval")
                .value_name("UPDATE_CACHES_INTERVAL")
                .help("update caches at an interval in seconds")
                .env("UPDATE_CACHES_INTERVAL")
                .default_value("0")
                .takes_value(true),
        ).get_matches();

    let connection_string: String = matches.value_of("database").unwrap().to_string();
    let listen_host: String = matches.value_of("listen_host").unwrap().parse().expect("listen_host is not string");
    let listen_port: i32 = matches.value_of("listen_port").unwrap().parse().expect("listen_port is not u32");
    let server_url: &str = matches.value_of("server_url").unwrap();
    let threads: usize = matches.value_of("threads").unwrap().parse().expect("threads is not usize");
    let update_caches_interval: u64 = matches.value_of("update-caches-interval").unwrap().parse().expect("update-caches-interval is not u64");

    loop {
        let connection = db::new(&connection_string, update_caches_interval);
        match connection {
            Ok(v) => {
                api::run(v, listen_host, listen_port, threads, server_url);
                break;
            }
            Err(e) => {
                println!("{}", e);
                thread::sleep(time::Duration::from_millis(1000));
            }
        }
    }
}
