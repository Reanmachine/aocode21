pub trait Challenge {
    fn run(&self) -> Result<(), String>;
}

#[derive(Debug)]
pub enum ArgumentError {
    InvalidSubcommand,
    MissingInput,
}