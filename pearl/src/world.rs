use amethyst::core::nalgebra::Vector3;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

pub struct Block {
    pub air: bool,
}

pub const CHUNK_SIZE: usize = 32;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkPos(pub Vector3<isize>);

pub type ChunkMap = HashMap<ChunkPos, Chunk>;

impl Hash for ChunkPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[0].hash(state);
        self.0[1].hash(state);
        self.0[2].hash(state);
    }
}

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
