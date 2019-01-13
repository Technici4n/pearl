pub struct Block {
    pub air: bool,
}

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    pub blocks: Box<[[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl Chunk {
    pub fn filled(block_id: usize) -> Self {
        Self {
            blocks: Box::new([[[block_id; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        }
    }
}
