use std::env;
use tokio::task;
mod config_processor;
use config_processor::{file_op,config_model};
mod master;
use master::master_process;
mod  worker;

#[tokio::main]
async fn main() {
    println!("Welcome to IMproxyble !!!");

    let mut args: Vec<String> = Vec::new();
    for (_,arg) in env::args().enumerate().skip(1) {
        args.push(arg);
    }

    if args.is_empty() {
        println!("the args are zero");
        return;
    }

    let copy_file_task = task::spawn(file_op::copy_config_file(args[0].clone(), "static".to_string()));
    let config = match file_op::read_file(&args[0]) {
        
        Ok(contents) => {
            match  config_model::Config::new(&contents){
                
                Ok(config) => {
                    println!("Config loaded: {:?}", config);
                    config
                },
                Err(err) => {
                    println!("Failed to parse config: {}", err);
                    return;
                },
            }
        },
        Err(err) => {
            println!("Failed to read file: {}", err);
            return;
        },
    };

    let brain = match master_process::Brain::new(&config) {
        Ok(brain) => {
            println!("master process created ");
            brain
        },
        Err(err) => {
            println!("failed to create master process {}", err);
            return;
        }
    };

    match copy_file_task.await {
        Ok(Ok(())) => println!("The contents copied successfully."),
        Ok(Err(e)) => println!("Error copying file: {}", e),
        Err(e) => println!("Task panicked: {:?}", e), // Handles cases where the task itself failed
    }

    
}
