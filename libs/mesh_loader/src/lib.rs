use common::GameState;

mod cube;

pub struct MeshLoader
{}

impl MeshLoader
{
    pub fn new(_game_state: &mut GameState) -> Self
    {

        let mut mesh_model = common::MeshModelLocation {
            vertices_start_index: _game_state.mesh_data.vertices.len() as u32,
            vertices_count: cube::VERTICES.len() as u32,
            indices_start_index: _game_state.mesh_data.indices.len() as u32,
            indices_count: cube::INDICES.len() as u32,
        };

        _game_state.mesh_data.models.push(mesh_model);

        _game_state.mesh_data.vertices.extend(cube::VERTICES);
        _game_state.mesh_data.indices.extend(cube::INDICES);

        return Self {};
    }
}

