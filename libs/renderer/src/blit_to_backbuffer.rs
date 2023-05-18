use std::borrow::Cow;

use wgpu::{ShaderModule, PipelineLayout, RenderPipeline, TextureFormat, Device, CommandEncoder, TextureView, Texture, BindGroupLayout, BindGroup, Sampler};

pub struct TriangleSystem
{
    shader: ShaderModule,
    pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,

    input_texture_view: TextureView,
    texture_sampler: Sampler,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
}

impl TriangleSystem
{
    pub fn new(device: &Device, textureformat: TextureFormat, input_texture: &Texture) -> Self
    {
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                include_str!("../../../data/shaders/blit.wgsl"))),
        });

        let input_texture_view= input_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("Input texture")),
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });




        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Back buffer blitter bind group"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
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
                buffers: &[],
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

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&input_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        Self {
            shader,
            pipeline_layout,
            render_pipeline,

            input_texture_view,
            texture_sampler,

            bind_group_layout,
            bind_group,
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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}