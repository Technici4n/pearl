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

    pub fn generate_cube(offset: Vector3<f32>) -> Vec<PosNormTex> {
        let mut output = Vec::new(); // TODO: with_capacity
        for f in 0..6 {
            for v in 0..6 {
                output.push(PosNormTex {
                    position: Vector3::from(VERTICES[FACES[f][v]]) + &offset,
                    normal: NORMALS[f].into(),
                    tex_coord: TEXTURE_COORDINATES[v].into(),
                });
            }
        }
        output
    }
}

/// Chunk meshing
pub mod chunk {
    use crate::{
        registry::Registry,
        world::{Block, Chunk, CHUNK_SIZE},
    };
    use amethyst::renderer::PosNormTex;

    const ADJACENCY_PLUS_ONE: [[usize; 3]; 6] = [
        [2, 1, 1],
        [0, 1, 1],
        [1, 2, 1],
        [1, 0, 1],
        [1, 1, 2],
        [1, 1, 0],
    ];

    pub fn generate_chunk(chunk: Chunk, block_registry: &Registry<Block>) -> Vec<PosNormTex> {
        let mut output = Vec::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut should_render = true;
                    if 0 < x
                        && x < CHUNK_SIZE - 1
                        && 0 < y
                        && y < CHUNK_SIZE - 1
                        && 0 < z
                        && z < CHUNK_SIZE - 1
                    {
                        should_render = false;
                        for side in 0..6 {
                            let nx = x + ADJACENCY_PLUS_ONE[side][0] - 1;
                            let ny = y + ADJACENCY_PLUS_ONE[side][1] - 1;
                            let nz = z + ADJACENCY_PLUS_ONE[side][2] - 1;
                            // TODO: if this is slow use unsafe unchecked getters
                            let block_id = chunk.blocks[nx][ny][nz];
                            let block = block_registry.get_item(block_id);
                            should_render = should_render | block.air;
                        }
                    }
                    if should_render {
                        output.append(&mut super::cube::generate_cube(
                            [x as f32, y as f32, z as f32].into(),
                        ));
                    }
                }
            }
        }
        output
    }
}
