#[derive(Clone, Copy)]
pub enum Block {
    Plain,
    Empty,
}

impl Block {
    pub fn is_blocked(&self) -> bool {
        match self {
            Block::Empty => false,
            Block::Plain => true,
        }
    }
}