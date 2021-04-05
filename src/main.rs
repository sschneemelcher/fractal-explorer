extern crate sdl2; 

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use sdl2::EventPump;

use std::time::Duration;
use std::process::exit;
 
pub fn main() {
    let sdl_context     = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_height   = 600i32;
    let window_width    = 800i32;

    let window = video_subsystem.window("fractal-explorer", window_width as u32, window_height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();


    let mut canvas = window.into_canvas()
        .present_vsync()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    exit(run(&mut canvas, &mut event_pump, window_width, window_height));
} 


fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            println!("found opengl driver");
            return Some(index as u32);
        }
    }
    println!("did not find opengl driver");
    None
}


fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump, window_width: i32, window_height: i32) -> i32 {

    let mut max_iteration = 1000;
    let mut scale = 1f64;
    let mut x_temp = 0.0f64;
    let mut y_temp = 0.0f64;
    let mut iteration = 0u32;
    let mut tmp = 0f64;
    let mut x_scaled = 0f64;
    let mut y_scaled = 0f64;
    let mut x_offset = -2f64;
    let mut y_offset = -1f64;
    let mut c = 0u32;

    loop {
        canvas.set_draw_color(Color::RGB(0, 0 , 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return 0;
                },
                Event::KeyDown { keycode: Some(Keycode::Plus), .. } => scale+=0.1,
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => scale-=0.1,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => y_offset-=0.1,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => y_offset+=0.1,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => x_offset-=0.1,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => x_offset+=0.1,
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => max_iteration+=100,
                Event::KeyDown { keycode: Some(Keycode::E), .. } => max_iteration-=100,
                _ => {}
            }
        }
        

            for x in 0..window_width {
                x_scaled = (((x as f64 - x_offset) / window_width as f64) + x_offset) / scale;

                for y in 0..window_height as i32 {
                    y_scaled = (((y as f64 - y_offset) / window_height as f64) + y_offset) / scale;

                    x_temp = 0.0f64;
                    y_temp = 0.0f64;
                    iteration = 0u32;

                    while (x_temp * x_temp + y_temp * y_temp <= 4f64) && (iteration < max_iteration) {
                        tmp = x_temp * x_temp - y_temp * y_temp + x_scaled;
                        y_temp = 2f64 * x_temp * y_temp + y_scaled;
                        x_temp = tmp;
                        iteration += 1;
                    }
                    c = iteration % 255;
                    canvas.set_draw_color(Color::RGB(c as u8, (c * c) as u8, (c * c * c ) as u8));
                    canvas.draw_point(Point::new(x, y));

                }
            }

            canvas.present();

        //        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

