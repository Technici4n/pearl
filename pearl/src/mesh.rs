pub mod cube {
    use amethyst::renderer::PosNormTex;

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

    pub fn generate_cube() -> Vec<PosNormTex> {
        let mut output = Vec::new();
        for f in 0..6 {
            for v in 0..6 {
                output.push(PosNormTex {
                    position: VERTICES[FACES[f][v]].into(),
                    normal: NORMALS[f].into(),
                    tex_coord: TEXTURE_COORDINATES[v].into(),
                });
            }
        }
        output
    }
}
