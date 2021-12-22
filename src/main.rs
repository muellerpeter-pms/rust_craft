mod client;

use client::Client;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).expect("log system could not get started!");

    let client = Client::new();
    client.run();
}
