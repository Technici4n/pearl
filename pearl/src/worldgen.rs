use crate::{
    registry::Registry,
    world::{Block, Chunk, CHUNK_SIZE},
};
use amethyst::core::nalgebra::Vector3;

const SEA_LEVEL: isize = 0;

/// Default Chunk generator
pub struct ChunkGenerator {
    air_block: usize,
    dirt_block: usize,
}

impl ChunkGenerator {
    /// TODO: proper error handling
    pub fn new(block_registry: &Registry<Block>) -> Self {
        Self {
            air_block: block_registry
                .get_item_id("default:air")
                .unwrap(),
            dirt_block: block_registry
                .get_item_id("default:dirt")
                .unwrap(),
        }
    }

    /// Generate a chunk at the given position
    pub fn generate_chunk(&mut self, pos: &Vector3<isize>) -> Chunk {
        let mut chunk = Chunk::filled(self.air_block);
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let height = j as isize + pos[1] * CHUNK_SIZE as isize;
                let generated_block = if height < SEA_LEVEL {
                    self.dirt_block
                } else {
                    self.air_block
                };
                for k in 0..CHUNK_SIZE {
                    chunk.blocks[i][j][k] = generated_block;
                }
            }
        }
        chunk
    }
}
