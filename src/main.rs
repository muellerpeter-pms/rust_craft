use log::*;

fn main() {    
    log4rs::init_file("log4rs.yml", Default::default()).expect("log system could not get started!");

    info!("log works");
    
}
