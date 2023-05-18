use std::borrow::Cow;

use wgpu::*;

pub struct TriangleSystem
{
    shader: ShaderModule,
    bind_group_layout: BindGroupLayout,
    pipeline_layout: PipelineLayout,
    compute_pipeline: ComputePipeline,
    bind_group: BindGroup,

    input_view: TextureView,
    output_view: TextureView,
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
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../../data/shaders/compute.wgsl"))),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Instantiates the pipeline.
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });



        let input_view = input_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("My compute texture view")),
            format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });

        let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("My compute texture view")),
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });


        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("My compute Bind Group"),
            layout: &compute_pipeline.get_bind_group_layout(0),
            entries: &[
                // Input
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&input_view),
                },
                // Output
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&output_view),
                },
            ],
        });





        Self {
            shader,
            bind_group_layout,
            pipeline_layout,
            compute_pipeline,
            bind_group,

            input_view,
            output_view,

        }
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView)
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.insert_debug_marker("compute collatz iterations");
        compute_pass.dispatch_workgroups(1024 / 8, 768 / 8, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
}