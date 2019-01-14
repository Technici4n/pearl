/// Cube meshing
pub mod cube {
    use amethyst::{core::nalgebra::Vector3, renderer::PosNormTex};

    /// Cube vertices. Bottom face then top face. Counterclockwise starting from (0.0, _, 0.0).
    const VERTICES: [[f32; 3]; 8] = [
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 0.0],
    ];

    /// Cube faces. Two triangles per face, split at the bottom-left to top-right diagonal.
    /// First top right and then bottom left part. First edge is the diagonal.
    /// Order: x = 0, x = 1, y = 0, y = 1, z = 0, z = 1
    const FACES: [[usize; 6]; 6] = [
        // +x
        [2, 7, 6, 7, 2, 3],
        // -x
        [0, 5, 4, 5, 0, 1],
        // +y
        [4, 6, 7, 6, 4, 5],
        // -y
        [1, 3, 2, 3, 1, 0],
        // +z
        [1, 6, 5, 6, 1, 2],
        // -z
        [3, 4, 7, 4, 3, 0],
    ];

    /// Texture coordinates for one face.
    const TEXTURE_COORDINATES: [[f32; 2]; 6] = [
        [0.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];

    /// Normal vector for every face. Same order as the `FACES` variable.
    const NORMALS: [[f32; 3]; 6] = [
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
    ];

    const TOTAL_VERTEX_COUNT: usize = 36;

    pub fn generate_cube(offset: Vector3<f32>, dest: &mut Vec<PosNormTex>) {
        dest.reserve(TOTAL_VERTEX_COUNT);
        for f in 0..6 {
            for v in 0..6 {
                dest.push(PosNormTex {
                    position: Vector3::from(VERTICES[FACES[f][v]]) + &offset,
                    normal: NORMALS[f].into(),
                    tex_coord: TEXTURE_COORDINATES[v].into(),
                });
            }
        }
    }
}

/// Chunk meshing
pub mod chunk {
    use crate::{
        registry::Registry,
        world::{Block, Chunk},
    };
    use amethyst::{core::nalgebra::Vector3, renderer::PosNormTex};

    pub const ADJACENCY: [[isize; 3]; 6] = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ];

    const CHUNK_SIZE: isize = crate::world::CHUNK_SIZE as isize;

    pub fn generate_chunk(
        chunk: &Chunk,
        adjacent_chunks: &[&Chunk],
        block_registry: &Registry<Block>,
    ) -> Vec<PosNormTex> {
        assert!(adjacent_chunks.len() == 6);

        let mut output = Vec::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut should_render = false;
                    for side in 0..6 {
                        let mut chunk = chunk;
                        let mut nx = x + ADJACENCY[side][0];
                        let mut ny = y + ADJACENCY[side][1];
                        let mut nz = z + ADJACENCY[side][2];
                        // TODO: Is there a cleaner/more efficient way to do this?
                        // TODO: Maybe separating chunk sides and chunk interior could work?
                        if nx == CHUNK_SIZE {
                            nx -= CHUNK_SIZE;
                            chunk = &adjacent_chunks[0];
                        }
                        if nx == -1 {
                            nx += CHUNK_SIZE;
                            chunk = &adjacent_chunks[1];
                        }
                        if ny == CHUNK_SIZE {
                            ny -= CHUNK_SIZE;
                            chunk = &adjacent_chunks[2];
                        }
                        if ny == -1 {
                            ny += CHUNK_SIZE;
                            chunk = &adjacent_chunks[3];
                        }
                        if nz == CHUNK_SIZE {
                            nz -= CHUNK_SIZE;
                            chunk = &adjacent_chunks[4];
                        }
                        if nz == -1 {
                            nz += CHUNK_SIZE;
                            chunk = &adjacent_chunks[5];
                        }
                        // TODO: if the cast makes the build slow then use unsafe
                        let block_id = chunk.blocks[nx as usize][ny as usize][nz as usize];
                        let block = block_registry.get_item(block_id);
                        should_render = should_render | block.air;
                    }
                    let block_id = chunk.blocks[x as usize][y as usize][z as usize];
                    let block = block_registry.get_item(block_id);
                    if !block.air && should_render {
                        super::cube::generate_cube(
                            Vector3::new(x as f32, y as f32, z as f32),
                            &mut output,
                        );
                    }
                }
            }
        }
        output
    }
}
