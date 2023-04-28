use crate::block::BlockKind;

pub const FIELD_WIDTH: usize = 13;
pub const FIELD_HEIGHT: usize = 21;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct Game {
    pub field: Field,
    pub pos: Pos,
    pub block: BlockKind,
}
