use std::borrow::Cow;
use std::collections::HashMap;
use std::{error,io};

use tokio::select;
use tokio::task::Id;

use crate::worker::worker_process::{self, Muscle};
use crate::config_processor::config_model;

#[derive(Debug)]
pub struct Brain<'a> {

    configurations: &'a config_model::Config,
    muscles: HashMap<Id,Muscle>

}

impl<'a> Brain<'a> {
    pub fn new(configurations: &'a config_model::Config) -> Result<Brain<'a>,Box<dyn error::Error>> {
        let new_brain = Brain { configurations: configurations, muscles: HashMap::new() };
        Ok(new_brain)
    }

    pub async fn start(mut self) -> io::Result<()> {
        println!("master process has started...");
        match self.configurations.get_setting(&"workers".to_string()) {
            Some(value)    => {
                match value.as_i64() {
                    Some(no_of_workers) => {
                        for i in 0..no_of_workers {
                            let muscle = Muscle::spawn().await?;
                            self.muscles.insert(muscle.get_id(), muscle);
                        }
                    },
                    None => println!("enter a valid no of workers")
                }
            },
            None => println!("key not found")
        }

        //process for listening to the status from worker processes
        //here maybe i can use a nested macro
        tokio::spawn(async move {
            loop {
                select! {
                    
                }
            }
        });
        Ok(())
    }
}
