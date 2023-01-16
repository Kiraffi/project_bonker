use winit::event::VirtualKeyCode;

fn main()
{
    env_logger::init();
    window::run_window(update);
}

fn update(timestep: f64)
{
    let v2 = glam::Vec2::new(1.5f32, 2.5f32);

    if input::is_down(&VirtualKeyCode::Space)
    {
        println!("space is down");
        println!("timestep: {}, f1: {}, f2: {}", timestep, v2.x, v2.y);
}
}
