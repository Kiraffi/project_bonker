use winit::event::VirtualKeyCode;

fn main()
{
    env_logger::init();

    window::run_window(update);
    println!("Haaa haaeyaaa");
}

fn update(timestep: f64)
{
    let v2 = cgmath::Vector2::new(1.5f32, 2.5f32);
    println!("timestep: {}, f1: {}, f2: {}", timestep, v2.x, v2.y);

    if input::is_down(&VirtualKeyCode::Space)
    {
        println!("space is down");
    }
}
