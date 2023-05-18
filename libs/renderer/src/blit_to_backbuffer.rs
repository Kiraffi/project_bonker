use std::borrow::Cow;
use std::num::NonZeroU32;

use wgpu::{ShaderModule, PipelineLayout, RenderPipeline, TextureFormat, Device, CommandEncoder, TextureView, Texture, BindGroupLayout, BindGroup, Sampler};

pub struct TriangleSystem
{
    _shader: ShaderModule,
    _pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,

    _input_texture_view: TextureView,
    _texture_sampler: Sampler,
    _bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
}

impl TriangleSystem
{
    pub fn new(device: &Device, texture_format: TextureFormat, input_texture: &Texture) -> Self
    {
        // Load the shaders from disk
        let _shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                include_str!("../../../data/shaders/blit.wgsl"))),
        });





        let _bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Back buffer blit bind group"),
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

        let (bind_group, _input_texture_view, _texture_sampler) =
            Self::create_bind_group(
                &device,
                &_bind_group_layout,
                &input_texture
            );


        let _pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[&_bind_group_layout],
            push_constant_ranges: &[],
        });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
        {
            label: None,
            layout: Some(&_pipeline_layout),
            vertex: wgpu::VertexState
            {
                module: &_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState
            {
                module: &_shader,
                entry_point: "fs_main",
                targets: &[Some(texture_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });


        Self {
            _shader,
            _pipeline_layout,
            render_pipeline,

            _input_texture_view,
            _texture_sampler,

            _bind_group_layout,
            bind_group,
        }
    }
    pub fn rebind_texture(&mut self, device: &Device, texture: &Texture)
    {
        let (bind_group, texture_view, sampler) =
            Self::create_bind_group(
                &device,
                &self._bind_group_layout,
                &texture
            );
        self.bind_group = bind_group;
        self._input_texture_view = texture_view;
        self._texture_sampler = sampler;
    }
    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView)
    {
        let mut render_pass= encoder.begin_render_pass(&wgpu::RenderPassDescriptor
        {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment
            {
                view,
                resolve_target: None,
                ops: wgpu::Operations
                {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        input_texture: &Texture
    ) -> (BindGroup, TextureView, Sampler)
    {
        let _input_texture_view = input_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("Input texture")),
            format: Some(input_texture.format()),
            base_mip_level: 0,
            mip_level_count: Some(NonZeroU32::try_from(1u32).unwrap().into()),
            ..Default::default()
        });

        let _texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&_input_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&_texture_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        return (bind_group, _input_texture_view, _texture_sampler);
    }

}