#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Auxiliary {
    Up(bool), 
    Down(bool), 
    Left(bool), 
    Right(bool)
}