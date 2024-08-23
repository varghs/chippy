use sdl2::render::WindowCanvas;
use sdl2::Sdl;

struct Platform {
    canvas: WindowCanvas,
}

impl Platform {
    pub fn new(title: String, window_width: u32, window_height: u32, texture_width: u32, texture_height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window("rust-sdl2 demo", window_width, window_height).position_centered().build().unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Platform {
            canvas
        }
    }
}