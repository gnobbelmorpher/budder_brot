use ndarray::{
    azip, par_azip, s, Array, ArrayBase, ArrayViewMut, Axis, Data, Dimension, Ix2, Ix3, RawData,
};
use num::complex::Complex;
use sdl2::event::Event;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::time::{Duration, Instant};

use sdl2::mouse::MouseWheelDirection::Unknown;
use sdl2::keyboard::Keycode;
use butterbrot::{util, Config, Image, Inputs};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn mandelbrot() -> impl FnMut(Image, &Inputs, (u32, u32)) {
    struct RunningPixel {
        c: Complex<f64>,
        z: Complex<f64>,
    }

    enum Iteration {
        Running(RunningPixel),
        StoppedAtIter(u64),
    }

    let center_r_0 = -0.5;
    let mut center_r = center_r_0;
    let center_i_0 = 0.0;
    let mut center_i = center_i_0;
    let mut real_width = 3.0;
    let mut imag_height = 2.0;
    let mut tick = 0;
    let mut width = WIDTH; //Auflösung in Pixeln in der Breite
    let mut height = HEIGHT; //Auflösung in Pixeln in der Höhe
    let mut scale = real_width / WIDTH as f64; //Skalierung in "Real/Pixel"
    let scale_0 = scale;
    let mut coordinates = (center_i, center_r, scale);
    let mut maus_alt = (0, 0);
    let mut maus_neu = (0, 0);
    let mut schieb = false;

    let initialize = |width, height, (center_i, center_r, scale)| {
        let HALF_WIDTH: f64 = width as f64 / 2.0;
        let HALF_HEIGHT: f64 = height as f64 / 2.0;
        Array::from_shape_fn((height, width), |(y, x)| {
            let r = (x as f64 - HALF_WIDTH) * scale + center_r;
            let i = (y as f64 - HALF_HEIGHT) * scale + center_i;
            let c = Complex::new(r, i);
            let z = Complex::new(0.0, 0.0);
            Iteration::Running(RunningPixel { c, z })
        })
    };
    //let mut koordinaten = ((0, 0), (WIDTH, HEIGHT));
    let mut values = initialize(width, height, coordinates);
    move |mut image: Image, inputs, (a, b)| {
        if image.arr.dim().0 != height || image.arr.dim().1 != width {
            height = image.arr.dim().0;
            width = image.arr.dim().1;
            //koordinaten = ((0, 0), (width, height));
            values = initialize(width, height, coordinates);
        }
        for event in inputs.events.iter() {
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    println! {"herunter bei x = {}, y = {}\n hoehe = {}, breite = {}", x, y, a, b};
                    //(koordinaten.0).0 = usize::try_from(*x).expect("negative x-coordinate");
                    //(koordinaten.0).1 = usize::try_from(*y).expect("negative x-coordinate");
                    maus_alt = (*x, *y);
                    //schieb = true;
                    center_i = coordinates.0 + (*y as f64 - height as f64/2.0) as f64 * coordinates.2;
                    center_r = coordinates.1 + (*x as f64 - width as f64/2.0) as f64 * coordinates.2;
                    coordinates = (center_i, center_r, scale);
                    tick = 0;
                    values = initialize(width, height, coordinates);
                } //Speichert den Landeort der Maus/kann den mittelpunkt verschieben
                /*Event::MouseButtonUp { x, y, .. } => {
                    println! {"hoch bei x = {}, y = {}\n hoehe = {}, breite = {}", x, y, a, b};
                    //(koordinaten.1).0 = usize::try_from(*x).expect("negative x-coordinate");
                    //(koordinaten.1).1 = usize::try_from(*y).expect("negative y-coordinate");
                    maus_neu = (*x, *y);
                    center_i = coordinates.0 - (maus_neu.1 - maus_alt.1) as f64 * coordinates.2;
                    center_r = coordinates.1 - (maus_neu.0 - maus_alt.0) as f64 * coordinates.2;
                    //scale = (maus_neu.0-maus_alt.0)as f64/width as f64;
                    coordinates = (center_i, center_r, scale);
                    values = initialize(width, height, coordinates);
                    schieb = false;
                }*/ //Verschiebt das Bild
                /*Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    /*println!{"x={}, y={}", x, y}*/
                    if schieb {
                        center_i = coordinates.0 - (*yrel) as f64 * coordinates.2;
                        center_r = coordinates.1 - (*xrel) as f64 * coordinates.2;
                        //scale = (maus_neu.0-maus_alt.0)as f64/width as f64;
                        coordinates = (center_i, center_r, scale);
                        values = initialize(width, height, coordinates);
                    }
                }*/
                /*Event::MouseWheel { direction, .. } => {
                    match direction {
                        Normal => {println!{"Mousewheel Normal"}; scale = coordinates.2 * 0.8;}
                        Flipped => {println!{"Mousewheel flipped"}; scale = coordinates.2 * 1.2;}
                        Unknown(x) => println!{"unknown: {}", x}
                    };
                    coordinates = (center_i, center_r, scale);
                    values = initialize(width, height, coordinates);
                }*/
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Num0) => {
                            tick = 0;
                            center_i = center_i_0;
                            center_r = center_r_0;
                            scale = scale_0;
                            coordinates = (center_i_0, center_r_0, scale_0);
                            values = initialize(width, height, coordinates);
                        }
                        Some(Keycode::Plus) | Some(Keycode::E) | Some(Keycode::PageDown) => {
                            scale = coordinates.2 * 0.8;
                            coordinates = (center_i, center_r, scale);
                            tick = 0;
                            values = initialize(width, height, coordinates);
                        }
                        Some(Keycode::Minus) | Some(Keycode::Q) | Some(Keycode::PageUp) => {
                            scale = coordinates.2 * 1.2;
                            coordinates = (center_i, center_r, scale);
                            tick = 0;
                            values = initialize(width, height, coordinates);
                        }
                        Some(Keycode::S) | Some(Keycode::Down) => {
                            center_i = coordinates.0 + width as f64 * scale * 0.1;
                            coordinates = (center_i, center_r, scale);
                            tick = 0;
                            values = initialize(width, height, coordinates);
                        }
                        Some(Keycode::W) | Some(Keycode::Up) => {
                            center_i = coordinates.0 - width as f64 * scale * 0.1;
                            coordinates = (center_i, center_r, scale);
                            tick = 0;
                            values = initialize(width, height, coordinates);
                        }
                        Some(Keycode::D) | Some(Keycode::Right) => {
                            center_r = coordinates.1 + width as f64 * scale * 0.1;
                            coordinates = (center_i, center_r, scale);
                            tick = 0;
                            values = initialize(width, height, coordinates);
                        }
                        Some(Keycode::A) | Some(Keycode::Left) => {
                            center_r = coordinates.1 - width as f64 * scale * 0.1;
                            coordinates = (center_i, center_r, scale);
                            tick = 0;
                            values = initialize(width, height, coordinates);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        azip!((mut pixel in image.arr.genrows_mut(), it in &mut values) {
            *it = match it {
                Iteration::Running(RunningPixel { ref c, ref z }) => {
                    if z.norm_sqr() > 16.0 {
                        pixel[0] = 0 + (tick * 10) as u8;
                        pixel[1] = 255 - (tick * 20) as u8;
                        pixel[2] = 255 - (tick * 10) as u8;
                        Iteration::StoppedAtIter(tick)
                    } else {
                        let z_new = z * z + c;
                        pixel[0] = 0;
                        pixel[1] = 0;
                        pixel[2] = 0;
                        Iteration::Running(RunningPixel { c: *c, z: z_new })
                    }
                }
                Iteration::StoppedAtIter(s) => Iteration::StoppedAtIter(*s),
            };
        });
        tick += 1;
    }
}

fn main() {
    let mut x = std::usize::MAX;
    println!("{}", x);
    let closure = mandelbrot();
    let config = Config {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        print_framerate: true,
    };
    match butterbrot::run(config, closure) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
