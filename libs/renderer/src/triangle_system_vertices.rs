use std::borrow::Cow;
use wgpu::util::DeviceExt;

use wgpu::{ShaderModule, PipelineLayout, RenderPipeline, TextureFormat, Device, CommandEncoder, TextureView};

pub struct TriangleSystem
{
    pub shader: ShaderModule,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]struct Vertex
{
    position: [f32; 4],
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
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0, 1.0], color: [0.5, 0.0, 0.5, 1.0] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0, 1.0], color: [0.5, 0.0, 0.5, 1.0] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0, 1.0], color: [0.5, 0.0, 0.5, 1.0] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0, 1.0], color: [0.5, 0.0, 0.5, 1.0] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0, 1.0], color: [0.5, 0.0, 0.5, 1.0] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];



impl TriangleSystem
{
    pub fn new(device: &Device, textureformat: TextureFormat) -> Self
    {
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../../data/shaders/triangle_shader_vertices.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[],
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
            depth_stencil: None,
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
        }
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView)
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor
        {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment
            {
                view: view,
                resolve_target: None,
                ops: wgpu::Operations
                {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);

        // Draw vertices without index buffer
        //rpass.draw(0..VERTICES.len() as u32, 0..1);
    }
}