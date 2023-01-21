
use wgpu::*;


mod triangle_system;
mod triangle_system_vertices;

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


    triangle_system: triangle_system::TriangleSystem,
    triangle_system_vertices: triangle_system_vertices::TriangleSystem,
}

impl Renderer
{
    pub async fn new<W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle>
        (window: &W, width: u32, height: u32) -> Self
    {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
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
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_format = surface.get_supported_formats(&adapter)[0];

        let config = wgpu::SurfaceConfiguration
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: width,
            height: height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface.get_supported_alpha_modes(&adapter)[0],
        };

        surface.configure(&device, &config);

        let triangle_system =
            triangle_system::TriangleSystem::new(&device, swapchain_format);
            let triangle_system_vertices =
            triangle_system_vertices::TriangleSystem::new(&device, swapchain_format);

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

            triangle_system,
            triangle_system_vertices,
        }
    }

    pub fn update(&mut self, _dt: f64)
    {

    }

    pub fn render(&mut self)
    {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.triangle_system.render(&mut encoder, &view);
        self.triangle_system_vertices.render(&mut encoder, &view);
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32)
    {
        // Reconfigure the surface with the new size
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
}

impl Drop for Renderer
{
    fn drop(&mut self)
    {
        println!("Renderer dropped");
    }
}

