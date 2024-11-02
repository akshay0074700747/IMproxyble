use std::env;
use tokio::task;
mod config_processor;
use config_processor::{file_op,config_model};

#[tokio::main]
async fn main() {
    println!("Welcome to IMproxyble !!!");

    let mut args: Vec<String> = Vec::new();
    for (_,arg) in env::args().enumerate().skip(1) {
        args.push(arg);
    }

    let copy_file_task = task::spawn(file_op::copy_config_file(&args[0], "improxyble/static/"));

    match file_op::read_file(&args[0]) {
        
        Ok(contents) => {
            match  config_model::Config::new(&contents){
                
                Ok(config) => println!("Config loaded: {:?}", config),
                Err(err) => println!("Failed to parse config: {}", err),
            }
        },
        Err(err) => println!("Failed to read file: {}", err),
    }

    let copy_file_result = copy_file_task.await;
}
