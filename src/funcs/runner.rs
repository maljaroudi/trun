pub trait Runner {
    fn run(&mut self) -> Result<(), std::io::Error>;
}
