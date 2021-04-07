extern crate sdl2; 
extern crate num_complex;


use sdl2::pixels::Color;
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
    let window_height   = 200i32;
    let window_width    = 200i32;

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


fn scale(x: f64, a: f64, b: f64, max: f64) -> f64 {

    return ((b-a) * x)/max + a;

} 

fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump, window_width: i32, window_height: i32) -> i32 {
    
    let mut max_iteration = 200;
    let mut x_temp        = 0.0f64;
    let mut y_temp        = 0.0f64;
    let mut iteration     = 0i32;
    let mut tmp           = 0f64;
    let mut x_scaled      = 0f64;
    let mut y_scaled      = 0f64;
    let mut x_offset      = 0f64;
    let mut y_offset      = 0f64;
    let mut min           = -2f64;
    let mut max           = 2f64;

    loop {
        canvas.set_draw_color(Color::RGB(0, 0 , 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return 0,
                Event::KeyDown { keycode: Some(Keycode::Plus), .. } => {min /= 1.5; max /= 1.5;}, 
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {min *= 1.5; max *= 1.5;},
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => y_offset -= 0.1 * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => y_offset += 0.1 * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => x_offset -= 0.1 * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => x_offset += 0.1 * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => max_iteration=std::cmp::min(100000000,max_iteration+100),
                Event::KeyDown { keycode: Some(Keycode::E), .. } => max_iteration=std::cmp::max(0, max_iteration-100),
                _ => continue
            }
        }
            for x in 0..window_width {
                x_scaled = scale(x as f64, min + x_offset, max + x_offset, window_width as f64);

                for y in 0..window_height {
                    y_scaled = scale(y as f64, min + y_offset, max + y_offset, window_height as f64);
                    let c = num_complex::Complex::new(x_scaled, y_scaled);
                    let mut z = num_complex::Complex::new(0.0, 0.0);    
                    iteration = 0;

                    while z.norm() <= 2f64 && (iteration < max_iteration) {
                        z = z.powu(2) + c;
                        iteration += 1;
                    }
                    iteration %= 255;
                    canvas.set_draw_color(Color::RGB(iteration as u8, (2 * iteration) as u8, (3 * iteration) as u8));
                    canvas.draw_point(Point::new(x, y)).ok();
                }
            }
            canvas.present();
    }
}

