use std::borrow::Cow;
use wgpu::util::DeviceExt;

use wgpu::{ShaderModule, PipelineLayout, RenderPipeline, TextureFormat, Device, CommandEncoder, TextureView, Texture};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform
{
    view_proj: [f32; 16],
}

impl CameraUniform
{
    fn new() -> Self
    {
        Self
        {
            view_proj: glam::Mat4::IDENTITY.to_cols_array(),
        }
    }

    fn update_view_proj(&mut self, camera: &common::Camera)
    {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array();
    }
}





pub struct TriangleSystem
{
    pub shader: ShaderModule,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex
{
    position: [f32; 4],
    normal: [f32; 4],
    color: [f32; 4],
}
// lib.rs
impl Vertex
{
    fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}
const WHITE_COLOR: [f32; 4] = [1.0f32, 1.0f32, 1.0f32, 1.0f32];

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5,  0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [ 0.5,  0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [ 0.5, -0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },

    Vertex { position: [ 0.5, -0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [ 0.5,  0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5,  0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5, -0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },

    Vertex { position: [-0.5, -0.5, -0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5,  0.5, -0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5,  0.5,  0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5, -0.5,  0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },

    Vertex { position: [0.5, -0.5,  0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [0.5,  0.5,  0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [0.5,  0.5, -0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [0.5, -0.5, -0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },

    Vertex { position: [-0.5,  0.5,  0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5,  0.5, -0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [ 0.5,  0.5, -0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [ 0.5,  0.5,  0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },


    Vertex { position: [ 0.5, -0.5, -0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [ 0.5, -0.5, 0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5, -0.5, 0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
    Vertex { position: [-0.5, -0.5, -0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
];

const INDICES: &[u16] = &[
    0, 1, 2,
    2, 0, 3,

    4, 5, 6,
    6, 4, 7,

    8, 9, 10,
    10, 8, 11,

    12, 13, 14,
    14, 12, 15,

    16, 17, 18,
    18, 16, 19,

    20, 21, 22,
    22, 20, 23,
//    3, 2, 5,
//    5, 4, 3,
//
//    7, 6, 1,
//    1, 0, 7,
//
//    1, 6, 5,
//    5, 2, 1,
//
//    7, 0, 3,
//    3, 4, 7,
];



impl TriangleSystem
{
    pub fn new(device: &Device, textureformat: TextureFormat, depth_texture_format: TextureFormat) -> Self
    {

        let camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
            label: Some("camera_bind_group_layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../data/shaders/triangle_shader_camera_vertices.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[
                &camera_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
        {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState
            {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc()
                ],
            },
            fragment: Some(wgpu::FragmentState
            {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(textureformat.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some( wgpu::DepthStencilState{
                format: depth_texture_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );





        Self {
            shader,
            pipeline_layout,
            render_pipeline,

            vertex_buffer,
            index_buffer,

            camera_uniform,
            camera_buffer,
            camera_bind_group,
        }
    }
    pub fn update(&mut self, camera: &common::Camera, queue: &wgpu::Queue)
    {
        self.camera_uniform.update_view_proj(camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]));

    }
    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView, depth_view: &TextureView)
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor
        {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment
            {
                view,
                resolve_target: None,
                ops: wgpu::Operations
                {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.camera_bind_group, &[]);

        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);

        // Draw vertices without index buffer
        //rpass.draw(0..VERTICES.len() as u32, 0..1);
    }
}