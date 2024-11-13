use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::sync::Mutex;
use tokio::task::{self, Id};

use std::io;

use crate::enums::enums;

#[derive(Debug)]
pub struct Muscle {
    id: Id,
    command_tx: Sender<enums::Command>,
    status_chl: Mutex<Receiver<enums::Status>>
}

impl Muscle {
    

    pub async fn spawn() -> io::Result<Self> {

        //in here teh command_tx is immutable because its a sender channel and it doesnt affect the internal state of the channel
        //command_rx is mutable becuase recieving messages changes teh internal state
        let (command_tx, mut command_rx) = mpsc::channel::<enums::Command>(32);
        let (status_tx, status_rx) = mpsc::channel::<enums::Status>(32);

        //move keyword forces the ownership of teh captured variables to move into the closure because the async process may outlive the scope that its called from
        //or in other words it simply allowes us to bring the variables into the scope of this async fn
        let join_handle = task::spawn(async move {

            while let Some(cmd) = command_rx.recv().await {
                match cmd {
                    enums::Command::DoSomething(msg) => {
                        println!("Processing: {}", msg);
                        // sending back the status of the task
                        let _ = status_tx.send(enums::Status::Done(format!("Processed {}", msg))).await;
                    },
                    enums::Command::Stop => {
                        println!("Stopping task");
                        break;
                    }
                }
            }
        });
        let id = join_handle.id();
        println!("worker process of id {} has started...",id);
        let status_chl = Mutex::new(status_rx);
        Ok(Muscle{id,command_tx,status_chl})
    }

    pub async fn send_command(&self, cmd: enums::Command) -> Result<(), tokio::sync::mpsc::error::SendError<enums::Command>> {
        self.command_tx.send(cmd).await //this will wait if the channel is full
    }

    pub fn get_id(&self) -> Id {
        self.id
    }
}