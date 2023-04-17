use shader::Vertex;

pub mod shader;

impl From<easy_gltf::model::Vertex> for Vertex {
    fn from(value: easy_gltf::model::Vertex) -> Self {
        let easy_gltf::model::Vertex {
            position,
            normal,
            tangent,
            tex_coords,
        } = value;
        Self {
            position: position.into(),
            normal: normal.into(),
            tangent: tangent.into(),
            tex_coords: tex_coords.into(),
        }
    }
}
