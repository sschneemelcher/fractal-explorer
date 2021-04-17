extern crate minifb;
extern crate num_complex;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 500;
const HEIGHT: usize = 400;

fn scale(x: usize, a: f64, b: f64, max: usize) -> f64 {
    return ((b-a) * x as f64)/max as f64 + a;}


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
//    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut max_iteration = 200u32;
    let mut iteration: u32;
    let mut x_scaled: f64;
    let mut y_scaled: f64;
    let mut x_offset      = 0f64;
    let mut y_offset      = 0f64;
    let mut min           = -2f64;
    let mut max           = 2f64;       
    let mut changed: bool = true;



    while window.is_open() && !window.is_key_down(Key::Escape) {
        if changed {
            let mut count: usize = 0;
            for i in buffer.iter_mut() {
                x_scaled = scale(count % WIDTH, min + x_offset, max + x_offset, WIDTH);
                y_scaled = scale(count / HEIGHT, min + y_offset, max + y_offset, HEIGHT);
                let c = num_complex::Complex::new(x_scaled, y_scaled);
                let mut z = num_complex::Complex::new(0.0, 0.0);
                iteration = 0_u32;

                while z.norm() <= 2f64 && (iteration < max_iteration) {
                    z = z.powu(2) + c;
                    iteration += 1;
                }

                *i = iteration.pow(5);
                count += 1;
            }
        }
        changed = false;
        window.get_keys_pressed(minifb::KeyRepeat::Yes).map(|keys| {
            for t in keys {
                changed = true;
                match t {
                    Key::Up => y_offset -= 0.1 * (max-min),
                    Key::Down => y_offset += 0.1 * (max-min),
                    Key::Left => x_offset -= 0.1 * (max-min),
                    Key::Right => x_offset += 0.1 * (max-min),
                    Key::Q => max_iteration=std::cmp::min(100000000,max_iteration+100),
                    Key::E => max_iteration=std::cmp::max(0, max_iteration-100),
                    Key::W => {min /= 1.5; max /= 1.5;},
                    Key::S => {min *= 1.5; max *= 1.5;},
                    _ => (),
                }
            }
        });
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
