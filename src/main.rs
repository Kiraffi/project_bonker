use std::f32::consts::PI;
use winit::event::VirtualKeyCode;



use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};


struct TestA {}
impl common::System for TestA
{
    fn update(&mut self, dt: f64, game_state: &mut common::GameState)
    {
        let v2 = glam::Vec2::new(1.5f32, 2.5f32);

        if game_state.input.is_down(&VirtualKeyCode::Space)
        {
            println!("space is down");
            println!("timestep: {}, f1: {}, f2: {}", dt, v2.x, v2.y);
            println!("timestep: {}, f1: {}, f2: {}", dt, v2.x, v2.y);
        }
    }
}


struct CameraSystem {}
impl common::System for CameraSystem
{

    fn update(&mut self, dt: f64, game_state: &mut common::GameState)
    {
        let camera = game_state.scene.get_current_camera_mut();
        let input = &game_state.input;


        let mut movement = glam::Vec3::ZERO;

        let forward = camera.get_forward();

        //let forward = (self.target - self.eye).normalize();
        let right = forward.cross(glam::Vec3::Y).normalize();
        let up = -forward.cross(right);
        let multiplier = if input.is_down(&VirtualKeyCode::LShift)
            || input.is_down(&VirtualKeyCode::RShift) { 5.0 } else { 1.0 };
        let rotation_speed = (dt * 1.0 * multiplier) as f32;
        let movement_speed = (dt * multiplier) as f32;

        if input.is_down(&VirtualKeyCode::W)
        {
            movement += forward * movement_speed;
        }
        if input.is_down(&VirtualKeyCode::S)
        {
            movement -= forward * movement_speed;
        }
        if input.is_down(&VirtualKeyCode::A)
        {
            movement -= right * movement_speed;
        }
        if input.is_down(&VirtualKeyCode::D)
        {
            movement += right * movement_speed;
        }
        if input.is_down(&VirtualKeyCode::Q)
        {
            movement -= up * movement_speed;
        }
        if input.is_down(&VirtualKeyCode::E)
        {
            movement += up * movement_speed;
        }


        if input.is_down(&VirtualKeyCode::I)
        {
            camera.pitch += rotation_speed;
        }
        if input.is_down(&VirtualKeyCode::K)
        {
            camera.pitch -= rotation_speed;
        }
        if input.is_down(&VirtualKeyCode::J)
        {
            camera.heading += rotation_speed;
        }
        if input.is_down(&VirtualKeyCode::L)
        {
            camera.heading -= rotation_speed;
        }
        camera.pitch = camera.pitch.max(-PI * 0.499f32).min(PI * 0.499f32);
        camera.eye += movement;
    }
}





async fn run()
{
    let size = winit::dpi::PhysicalSize::new(1024, 768);
    let mut game_state =
        common::GameState::new(size.width as f32, size.height as f32);

    // Init only:
    let _mesh_loader = mesh_loader::MeshLoader::new(&mut game_state);


    // Updateable systems.
    let mut systems: Vec<Box<dyn common::System>> = Vec::new();

    systems.push(Box::new(CameraSystem{}));
    systems.push(Box::new(TestA{}));


    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)

        .build(&event_loop).unwrap();

    let size = window.inner_size();
    println!("window size: {}, {}", size.width, size.height);
    let mut renderer =
        renderer::Renderer::new(&window, size.width, size.height, &game_state).await;
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
                    game_state.input.update(event);
                    match event
                    {
                        WindowEvent::Resized(size) =>
                            {
                                renderer.resize(size.width, size.height);
                                game_state.scene.resize_canvas(
                                    size.width as f32,
                                    size.height as f32
                                );
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

                    //update_func(&mut game_state, &input, dt);

                    game_state.input.reset();

                    for system in &mut systems
                    {
                        system.as_mut().update(dt, &mut game_state);
                    }
                    for system in &mut systems
                    {
                        system.as_mut().post_update(dt, &mut game_state);
                    }

                    renderer.update(dt, &game_state);
                    renderer.render();
                    //std::thread::sleep(std::time::Duration::from_millis(1));
                },
            _ => {}
        }
    });
}


fn main()
{
    env_logger::init();
    // Temporarily avoid srgb formats for the swapchain on the web
    pollster::block_on(run());
}

