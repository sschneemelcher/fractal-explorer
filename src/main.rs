extern crate sdl2; 
extern crate num_complex;
extern crate rust_decimal;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use sdl2::EventPump;

use rust_decimal::prelude::Decimal;

use std::time::Duration;
use std::process::exit;
 
pub fn main() {
    let sdl_context     = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_height   = 150i32;
    let window_width    = 150i32;

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


fn scale(x: Decimal, a: Decimal, b: Decimal, max: Decimal) -> Decimal {

    return ((b-a) * x)/max + a;

} 

fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump, window_width: i32, window_height: i32) -> i32 {
    
    let mut max_iteration = 200;
    let mut x_temp        = 0.0f64;
    let mut y_temp        = 0.0f64;
    let mut iteration     = 0i32;
    let mut tmp           = 0f64;
    let mut x_offset      = Decimal::new(0, 0);
    let mut y_offset      = Decimal::new(0, 0);
    let mut min           = Decimal::new(-2, 0);
    let mut max           = Decimal::new(2, 0);
    let mut x_scaled      = Decimal::new(0, 0);
    let mut y_scaled      = Decimal::new(0, 0);
    let width             = Decimal::new(window_width as i64, 0);
    let height            = Decimal::new(window_height as i64, 0);
    let move_speed        = Decimal::new(1, 1);
    let zoom_speed        = Decimal::new(15, 1);
    let threshold         = Decimal::new(4, 0);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0 , 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return 0,
                Event::KeyDown { keycode: Some(Keycode::Plus), .. }   => {min         /= zoom_speed; max /= zoom_speed;}, 
                Event::KeyDown { keycode: Some(Keycode::Minus), .. }  => {min         *= zoom_speed; max *= zoom_speed;},
                Event::KeyDown { keycode: Some(Keycode::Up), .. }     => y_offset     -= move_speed * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Down), .. }   => y_offset     += move_speed * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Left), .. }   => x_offset     -= move_speed * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Right), .. }  => x_offset     += move_speed * (max-min),
                Event::KeyDown { keycode: Some(Keycode::Q), .. }      => max_iteration = std::cmp::min(100000000,max_iteration+100),
                Event::KeyDown { keycode: Some(Keycode::E), .. }      => max_iteration = std::cmp::max(0, max_iteration-100),
                _                                                     => continue 'running
            }
        }
            for x in 0..window_width {
                x_scaled = scale(Decimal::new(x as i64, 0), min + x_offset, max + x_offset, width);

                for y in 0..window_height {
                    println!("x: {}, y: {}", x, y);
                    y_scaled = scale(Decimal::new(y as i64, 0), min + y_offset, max + y_offset, height);
                    let c = num_complex::Complex::new(x_scaled, y_scaled);
                    let mut z = num_complex::Complex::new(Decimal::new(0, 0), Decimal::new(0, 0));    
                    iteration = 0;

                    while ((z.re * z.re) + (z.im * z.im)) <= threshold && (iteration < max_iteration) {
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

