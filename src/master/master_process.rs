use std::{error,io};

use crate::worker::worker_process;
use crate::config_processor::config_model;

#[derive(Debug)]
pub struct Brain<'a> {

    configurations: &'a config_model::Config,

}

impl<'a> Brain<'a> {
    pub fn new(configurations: &'a config_model::Config) -> Result<Brain<'a>,Box<dyn error::Error>> {
        let new_brain = Brain { configurations: configurations };
        Ok(new_brain)
    }

    pub fn start(&self) -> io::Result<()> {
        println!("master process has started...");

        Ok(())
    }
}
