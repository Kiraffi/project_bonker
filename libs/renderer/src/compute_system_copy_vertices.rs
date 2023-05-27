use std::borrow::Cow;
use std::num::NonZeroU32;

use wgpu::*;

pub struct TriangleSystem
{
    _shader: ShaderModule,
    _bind_group_layout: BindGroupLayout,
    _pipeline_layout: PipelineLayout,
    compute_pipeline: ComputePipeline,
    bind_group: BindGroup,

    _input_view: TextureView,
    _output_view: TextureView,

    texture_width: u32,
    texture_height: u32,

    index_buffer: Buffer,
    vertex_buffer: Buffer,
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
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../../data/shaders/compute.wgsl"))),
        });

        let _bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute bindings"),
            entries: &[
                // Input
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Output
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        view_dimension: wgpu::TextureViewDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        access: wgpu::StorageTextureAccess::WriteOnly,
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
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&_pipeline_layout),
            module: &_shader,
            entry_point: "main",
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


        let (bind_group, _input_view, _output_view) =
            Self::create_bind_group(
                &device,
                &_bind_group_layout,
                &input_texture,
                &output_texture,
            );



        Self {
            _shader,
            _bind_group_layout,
            _pipeline_layout,
            compute_pipeline,
            bind_group,

            _input_view,
            _output_view,

            texture_width: output_texture.width(),
            texture_height: output_texture.height(),

            index_buffer,
            vertex_buffer,
        }
    }
    pub fn rebind_textures(&mut self,
       device: &Device,
       input_texture: &wgpu::Texture,
       output_texture: &wgpu::Texture
    )
    {
        let (bind_group, _input_view, _output_view) =
            Self::create_bind_group(
                &device,
                &self._bind_group_layout,
                &input_texture,
                &output_texture,
            );
        self.bind_group = bind_group;
        self._input_view = _input_view;
        self._output_view = _output_view;
        self.texture_width = input_texture.width();
        self.texture_height = input_texture.height();
    }
    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView)
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.insert_debug_marker("Compute testing");
        compute_pass.dispatch_workgroups(
            (self.texture_width + 7) / 8,
            (self.texture_height + 7) / 8,
            1
        );
    }

    fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        input_texture: &wgpu::Texture,
        output_texture: &wgpu::Texture
    ) -> (BindGroup, TextureView, TextureView)
    {
        let _input_view = input_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("My compute texture view")),
            format: Some(input_texture.format()),
            base_mip_level: 0,
            mip_level_count: Some(NonZeroU32::try_from(1u32).unwrap().into()),
            ..Default::default()
        });

        let _output_view = output_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("My compute texture view")),
            format: Some(output_texture.format()),
            base_mip_level: 0,
            mip_level_count: Some(NonZeroU32::try_from(1u32).unwrap().into()),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("My compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                // Input
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&_input_view),
                },
                // Output
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&_output_view),
                },
            ],
        });
        return (bind_group, _input_view, _output_view);
    }
}