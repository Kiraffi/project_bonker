use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};


async fn run<T>(event_loop: EventLoop<()>, window: Window, update_func: T)
    where T: Fn(&input::Input, f64) + 'static
{
    let size = window.inner_size();
    let mut renderer = renderer::Renderer::new(&window, size.width, size.height).await;
    let mut input = input::Input::new();

    let mut now = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {

        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = &renderer;

        *control_flow = ControlFlow::Poll;
        match event
        {
            Event::MainEventsCleared => { window.request_redraw(); },
            Event::WindowEvent
            {
                ref event,
                window_id,
            } if window.id() == window_id =>
            {
                input.update(event);
                match event
                {
                    WindowEvent::Resized(size) =>
                    {
                        renderer.resize(size.width, size.height);
                        // On macos the window needs to be redrawn manually after resizing
                        window.request_redraw();
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            },
            Event::RedrawRequested(_) =>
            {
                let new_now = std::time::Instant::now();
                let dur = new_now.duration_since(now);
                let dt = dur.as_micros() as f64 / 1000_000.0;
                now = new_now;

                update_func(&input, dt);

                input.reset();

                renderer.update(dt);
                renderer.render();
                //std::thread::sleep(std::time::Duration::from_millis(1));
            },
            _ => {}
        }
    });
}
pub fn run_window<T>(update_func: T) where T: Fn(&input::Input, f64) + 'static
{
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    {
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run(event_loop, window, update_func));
    }

}
