use tokio::task::Id;
use std::io;

#[derive(Debug)]
pub struct Muscle {
    id: Id
}

impl Muscle {
    
    pub  fn new(id: Id) -> Muscle {
        let new_musle = Muscle{id: id};
        new_musle
    }

    pub fn start(&self) -> io::Result<()> {
        println!("worker process of id {} has started...",self.id);
        
        Ok(())
    }
}