#[typetag::deserialize(tag = "module")]
pub trait Runner {
    fn run(&mut self) -> Result<(), std::io::Error>;
}
