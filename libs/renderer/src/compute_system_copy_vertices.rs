use std::borrow::Cow;
use std::num::NonZeroU32;

use wgpu::*;
use wgpu::BindingResource::Buffer;

pub struct TriangleSystem
{
    _shader: ShaderModule,
    _bind_group_layout: BindGroupLayout,
    _pipeline_layout: PipelineLayout,
    compute_pipeline: ComputePipeline,
    compute_pipeline_reset: ComputePipeline,
    bind_group: BindGroup,

    texture_width: u32,
    texture_height: u32,

    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MeshGpu
{
    vertices_start_index: u32,
    vertices_count: u32,
    indices_start_index: u32,
    indices_count: u32,
}

impl TriangleSystem
{
    pub fn new(
        device: &Device,
        input_texture: &wgpu::Texture,
        output_texture: &wgpu::Texture
    ) -> Self
    {
        // Load the shaders from disk
        let _shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::ShaderSource::Wgsl(
                Cow::Borrowed(include_str!("../../../data/shaders/compute_copy_vertices.wgsl"))),
        });

        let index_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Index Buffer"),
                size: (64u32 * 1024u32 * 1024u32) as BufferAddress,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }
        );

        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                size: (64u32 * 1024u32 * 1024u32) as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }
        );

        let _bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute bindings"),
            entries: &[
                // Input
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Output
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let _pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[&_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Instantiates the pipeline.
        let compute_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&_pipeline_layout),
                module: &_shader,
                entry_point: "main",
            }
        );
        // Instantiates the pipeline.
        let compute_pipeline_reset = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&_pipeline_layout),
                module: &_shader,
                entry_point: "main_reset",
            }
        );





        let bind_group =
            Self::create_bind_group(
                &device,
                &_bind_group_layout,
                &index_buffer,
                &vertex_buffer,
            );



        Self {
            _shader,
            _bind_group_layout,
            _pipeline_layout,
            compute_pipeline,
            compute_pipeline_reset,
            bind_group,

            texture_width: output_texture.width(),
            texture_height: output_texture.height(),

            index_buffer,
            vertex_buffer,
        }
    }
    pub fn rebind_textures(&mut self,
       device: &Device,
    )
    {
        let bind_group =
            Self::create_bind_group(
                &device,
                &self._bind_group_layout,
                &self.index_buffer,
                &self.vertex_buffer,
            );
        self.bind_group = bind_group;
    }
    pub fn render(&mut self, encoder: &mut CommandEncoder)//, view: &TextureView)
    {
        {
            let mut compute_pass1 = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            compute_pass1.set_bind_group(0, &self.bind_group, &[]);
            compute_pass1.insert_debug_marker("Compute reset");
            compute_pass1.set_pipeline(&self.compute_pipeline_reset);
            compute_pass1.dispatch_workgroups(1, 1, 1);
        }
        {
            let mut compute_pass2 = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            compute_pass2.set_pipeline(&self.compute_pipeline);
            compute_pass2.set_bind_group(0, &self.bind_group, &[]);
            compute_pass2.insert_debug_marker("Compute testing");
            compute_pass2.dispatch_workgroups(
                (self.texture_width * self.texture_height + 63) / 64,
//                (self.texture_width + 7) / 8,
//                (self.texture_height + 7) / 8,
                1,
                1
            );
        }

    }

    fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        input_buffer: &wgpu::Buffer,
        output_buffer: &wgpu::Buffer
    ) -> BindGroup
    {
        let _input_view = input_buffer.as_entire_binding();
        let _output_view = output_buffer.as_entire_binding();

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("My compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                // Input
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: _input_view,
                },
                // Output
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: _output_view,
                },
            ],
        });
        return bind_group;
    }
}