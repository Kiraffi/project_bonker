pub trait System
{
    fn update(&mut self, _dt: f64, _game_state: &mut GameState) {}
    fn post_update(&mut self, _dt: f64, _game_state: &mut GameState) {}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshModelLocation
{
    pub vertices_start_index: u32,
    pub vertices_count: u32,
    pub indices_start_index: u32,
    pub indices_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex
{
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuOutInstanceMatrices
{
    pub v0: [f32; 4],
    pub v1: [f32; 4],
    pub v2: [f32; 4],
}














pub struct MeshData
{
    pub models: Vec<MeshModelLocation>,
    pub vertices: Vec<MeshVertex>,
    pub indices: Vec<u32>,

    pub gpu_out_instance_matrices: Vec<GpuOutInstanceMatrices>,
    pub gpu_out_instance_mesh_model_locations: Vec<MeshModelLocation>,
}

impl MeshData
{
    fn new() -> Self
    {
        Self
        {
            models: Vec::with_capacity(1024),
            vertices: Vec::with_capacity(1024 * 1024),
            indices: Vec::with_capacity(1024 * 1024),

            gpu_out_instance_matrices: Vec::with_capacity(1024 * 1024),
            gpu_out_instance_mesh_model_locations: Vec::with_capacity(1024 * 1024),
        }
    }
}



pub struct GameState
{
    pub input: input::Input,
    pub scene: Scene,

    pub mesh_data: MeshData,
}

impl GameState
{
    pub fn new(width: f32, height: f32) -> Self
    {
        Self {
            input: input::Input::new(),
            scene: Scene::new(width, height),
            mesh_data: MeshData::new(),
        }
    }
}

pub struct Transform
{
    pub pos: glam::Vec3A,
    pub rot: glam::Quat,
    pub scale: glam::Vec3A
}

pub struct Entity
{
    pub transform: Transform,
}

pub struct Scene
{
    pub entities: Vec<Entity>,
    pub cameras: Vec<Camera>,
    pub current_cam_index: usize,
}

impl Scene
{
    pub fn new(width: f32, height: f32) -> Self
    {
        let mut cameras = Vec::new();
        cameras.push(Camera::new(width, height));
        Self {
            entities: Vec::new(),
            cameras,
            current_cam_index: 0,
        }
    }

    pub fn resize_canvas(&mut self, width: f32, height: f32)
    {
        self.cameras[0].resize(width, height);
    }

    pub fn get_current_camera(&self) -> &Camera
    {
        return &self.cameras[self.current_cam_index];
    }

    pub fn get_current_camera_mut(&mut self) -> &mut Camera
    {
        return &mut self.cameras[self.current_cam_index];
    }
}


pub struct Camera
{
    pub eye: glam::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    pub heading: f32,
    pub pitch: f32,
}


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::mat4(
    glam::Vec4::new(1.0, 0.0, 0.0, 0.0),
    glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
    glam::Vec4::new(0.0, 0.0, 0.5, 0.0),
    glam::Vec4::new(0.0, 0.0, 0.5, 1.0),
);




impl Camera
{
    pub fn new(width: f32, height: f32) -> Self
    {
        Self
        {
            eye: glam::vec3(0.0, 1.0, 2.0),
            aspect: width / height,
            fovy: 55.0,
            znear: 0.1,
            zfar: 100.0,

            heading: std::f32::consts::PI,
            pitch: 0.0,
        }
    }
    pub fn resize(&mut self, width: f32, height: f32)
    {
        self.aspect = width / height;
    }

    pub fn build_view_projection_matrix(&self) -> glam::Mat4
    {
        let view = glam::Mat4::look_at_rh(
            self.eye,
            self.eye + self.get_forward(),
            glam::Vec3::Y);

        let proj = glam::Mat4::perspective_rh(
            self.fovy.to_radians(),
            self.aspect,
            self.znear,
            self.zfar);

        return proj * view;
    }

    pub fn get_forward(&self) -> glam::Vec3
    {
        let sinx = self.heading.sin();
        let cosx = self.heading.cos();

        let siny = self.pitch.sin();
        let cosy = self.pitch.cos();

        let forward = glam::vec3(sinx * cosy,  siny, cosx * cosy);
        return forward;
    }
}



