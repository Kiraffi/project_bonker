use std::mem::size_of;
use common::GameState;
use wgpu::*;
use wgpu::util::DeviceExt;

mod blit_to_backbuffer;
mod compute_system;
mod compute_system_copy_vertices;
mod triangle_system;
mod triangle_system_vertices;
mod triangle_system_camera_vertices;

pub struct PhysicalSize<P> {
    pub width: P,
    pub height: P,
}

impl<P> PhysicalSize<P> {
    #[inline]
    pub const fn new(width: P, height: P) -> Self {
        PhysicalSize { width, height }
    }
}

pub struct Renderer
{
    width: u32,
    height: u32,
    _instance: Instance,
    surface: Surface,
    _adapter: Adapter,

    device: Device,
    queue: Queue,

    _swapchain_format: TextureFormat,
    config: SurfaceConfiguration,

    render_target_texture: Texture,
    render_target_texture2: Texture,

    render_target_texture_view: TextureView,

    render_target_depth_texture: Texture,
    render_target_depth_texture_view: TextureView,
    render_target_depth_texture_sampler: Sampler,

    compute_system: compute_system::TriangleSystem,
    compute_system_copy_vertices: compute_system_copy_vertices::TriangleSystem,

    triangle_system: triangle_system::TriangleSystem,
    triangle_system_vertices: triangle_system_vertices::TriangleSystem,
    triangle_system_camera_vertices: triangle_system_camera_vertices::TriangleSystem,


    blit_to_backbuffer: blit_to_backbuffer::TriangleSystem,



    model_mesh_vertices: Buffer,
    model_mesh_indices: Buffer,

    frame_instance_model_data: Buffer,
    frame_instance_model_transforms: Buffer,

    gpu_frame_vertices: Buffer,
    gpu_frame_indices: Buffer,
    gpu_frame_instance_data: Buffer,
}

impl Renderer
{ 
    fn create_depth_texture(device: &Device, config: &wgpu::SurfaceConfiguration) -> 
        (Texture, TextureView, Sampler)
    {

        let texture = Self::create_rendertarget_texture(
            &device,
            config.width,
            config.height,
            TextureFormat::Depth32Float,
            TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
        );


        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler( &wgpu::SamplerDescriptor{
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear ,
            
            mipmap_filter: wgpu::FilterMode::Nearest,

            compare: Some(wgpu::CompareFunction::LessEqual),

            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,

            ..Default::default()
        });

        return (texture, view, sampler)

    }


    fn create_buffers(device: &Device, game_state: &common::GameState) -> (
        Buffer, Buffer, Buffer, Buffer, Buffer, Buffer, Buffer
    )
    {
        let gpu_frame_vertices = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex Buffer gpu copy"),
                size: (size_of::<common::MeshVertex>() * 1_000_000) as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }
        );

        let gpu_frame_indices = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Index Buffer gpu copy"),
                size: (size_of::<u32>() * 1_000_000) as BufferAddress,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }
        );

        let gpu_frame_instance_data = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Instance Buffer gpu copy"),
                size: (size_of::<u32>() * 8 * 256) as BufferAddress,
                usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }
        );

        let model_mesh_vertices = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer all"),
                contents: unsafe { game_state.mesh_data.vertices.align_to::<u8>().1 },
                usage: wgpu::BufferUsages::STORAGE,
            }
        );

        let model_mesh_indices = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer all"),
                contents: unsafe { game_state.mesh_data.indices.align_to::<u8>().1 },
                usage: wgpu::BufferUsages::STORAGE,
            }
        );

        let frame_instance_model_data = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Instance model data"),
                size: (size_of::<common::MeshModelLocation>() * 1024 * 1024) as BufferAddress,
                usage: wgpu::BufferUsages::MAP_WRITE, // | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: true,
            }
        );
        let frame_instance_model_transforms = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Frame instance model transforms"),
                size: (size_of::<common::GpuOutInstanceMatrices>() * 1024 * 1024) as BufferAddress,
                usage: wgpu::BufferUsages::MAP_WRITE, // | wgpu::BufferUsages::STORAGE,
                mapped_at_creation: true,
            }
        );

        (
            model_mesh_vertices,
            model_mesh_indices,
        
            frame_instance_model_data,
            frame_instance_model_transforms,
        
            gpu_frame_vertices,
            gpu_frame_indices,
            gpu_frame_instance_data,
        
        )

    }

    fn create_rendertarget_texture(
        device: &Device,
        w: u32,
        h: u32,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
    ) -> wgpu::Texture
    {
        //if w % 64 != 0
        {
            //return Err("Render target texture needs to be multiple of 64 pixels.")
        }

        let rt_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            label: None,
            view_formats: &[], //TextureFormat::Rgba8UnormSrgb, TextureFormat::Rgba8Unorm],
        };
        return device.create_texture(&rt_desc);
    }
    fn create_render_target_textures(
        device: &Device,
        width: u32,
        height: u32
    ) -> (Texture, Texture, TextureView)
    {
        let render_target_texture = Self::create_rendertarget_texture(
            &device,
            width,
            height,
            TextureFormat::Rgba8UnormSrgb,
            TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
        );
        let render_target_texture2 = Self::create_rendertarget_texture(
            &device,
            width,
            height,
            TextureFormat::Rgba8Unorm,
            TextureUsages::TEXTURE_BINDING
                | TextureUsages::STORAGE_BINDING
        );

        let render_target_texture_view = render_target_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        return (render_target_texture, render_target_texture2,render_target_texture_view);
    }














    
    pub async fn new<W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle>
        (window: &W, width: u32, height: u32, game_state: &GameState) -> Self
    {

        let instance = wgpu::Instance::default(); //new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) }
            .unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions
            {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor
                {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_defaults() // downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let config = wgpu::SurfaceConfiguration
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![]
        };

        //format: wgpu::TextureFormat::Rgba8UnormSrgb,

        surface.configure(&device, &config);

        let (render_target_texture, render_target_texture2, render_target_texture_view) =
            Self::create_render_target_textures(&device, width, height);


        let (render_target_depth_texture, render_target_depth_texture_view, render_target_depth_texture_sampler) =
            Self::create_depth_texture(&device, &config);

        let (
                model_mesh_vertices,
                model_mesh_indices,
            
                frame_instance_model_data,
                frame_instance_model_transforms,
            
                gpu_frame_vertices,
                gpu_frame_indices,
                gpu_frame_instance_data,
            ) = Self::create_buffers(&device, &game_state);


        let triangle_system =
            triangle_system::TriangleSystem::new(
                &device,
                render_target_texture.format());
        let triangle_system_vertices =
            triangle_system_vertices::TriangleSystem::new(
                &device,
                render_target_texture.format());

        let triangle_system_camera_vertices =
        triangle_system_camera_vertices::TriangleSystem::new(
            &device,
            render_target_texture.format(),
            render_target_depth_texture.format());


        let compute_system = compute_system::TriangleSystem::new(
            &device,
            &render_target_texture,
            &render_target_texture2,
        );

        let compute_system_copy_vertices = compute_system_copy_vertices::TriangleSystem::new(
            &device,
            &render_target_texture,
            &render_target_texture2,
        );

        let blit_to_backbuffer = blit_to_backbuffer::TriangleSystem::new(
            &device,
            swapchain_format,
            &render_target_texture2
        );






        Self {
            width,
            height,

            _instance: instance,
            surface,
            _adapter: adapter,

            device,
            queue,

            _swapchain_format: swapchain_format,
            config,

            render_target_texture,
            render_target_texture2,

            render_target_texture_view,

            render_target_depth_texture,
            render_target_depth_texture_view,
            render_target_depth_texture_sampler,

            compute_system,
            compute_system_copy_vertices,
            
            triangle_system,
            triangle_system_vertices,
            triangle_system_camera_vertices,

            blit_to_backbuffer,



            model_mesh_vertices,
            model_mesh_indices,
        
            frame_instance_model_data,
            frame_instance_model_transforms,
        
            gpu_frame_vertices,
            gpu_frame_indices,
            gpu_frame_instance_data,
        }
    }

    pub fn update(&mut self, _dt: f64, game_state: &common::GameState)
    {
        self.triangle_system_camera_vertices.update(game_state.scene.get_current_camera(), &self.queue);
    }

    pub fn render(&mut self)
    {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let back_buffer_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.triangle_system.render(&mut encoder, &self.render_target_texture_view);
        self.triangle_system_vertices.render(&mut encoder, &self.render_target_texture_view);
        self.triangle_system_camera_vertices.render(
            &mut encoder,
            &self.render_target_texture_view,
            &self.render_target_depth_texture_view);
        self.compute_system.render(&mut encoder, &self.render_target_texture_view);
        self.compute_system_copy_vertices.render(&mut encoder);
        self.blit_to_backbuffer.render(&mut encoder, &back_buffer_view);
        /*
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &self.render_target_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            wgpu::ImageCopyTexture {
                texture: &frame.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            self.render_target_texture.size()
        );
        */
        self.queue.submit(Some(encoder.finish()));




        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32)
    {
        if width == self.width && height == self.height
        {
            return;
        }
        let width = std::cmp::max(4u32, width);
        let height = std::cmp::max(4u32, height);
        // Reconfigure the surface with the new size
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        //self.render_target_texture.destroy();
        //self.render_target_texture2.destroy();

        let (render_target_texture, render_target_texture2, render_target_texture_view) =
            Self::create_render_target_textures(&self.device, width, height);

        let (render_target_depth_texture, render_target_depth_texture_view, render_target_depth_texture_sampler) =
            Self::create_depth_texture(&self.device, &self.config);


        self.compute_system.rebind_textures(
            &self.device,
            &render_target_texture,
            &render_target_texture2
        );
        self.blit_to_backbuffer.rebind_texture(
            &self.device,
            &render_target_texture2,
        );
        self.render_target_texture = render_target_texture;
        self.render_target_texture2 = render_target_texture2;
        self.render_target_texture_view = render_target_texture_view;

        self.render_target_depth_texture = render_target_depth_texture;
        self.render_target_depth_texture_view = render_target_depth_texture_view;
        self.render_target_depth_texture_sampler = render_target_depth_texture_sampler;

    }
}

impl Drop for Renderer
{
    fn drop(&mut self)
    {
        println!("Renderer dropped");
    }
}

