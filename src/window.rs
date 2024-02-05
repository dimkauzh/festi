extern crate glutin;

use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;

pub struct Window {
    event_loop,
    video_subsystem: sdl2::VideoSubsystem,
    window: Option<sdl2::video::Window>,
}

pub fn create_window() -> Window {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    Window {
        sdl_context,
        video_subsystem,
        window: Some(window),
    }
}
