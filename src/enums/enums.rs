#[derive(Debug)]
pub enum Command {
    DoSomething(String),
    Stop,
}

#[derive(Debug)]
pub enum Status {
    Done(String),
    Error(String),
}