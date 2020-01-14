extern crate crossbeam;
extern crate image;
extern crate pbr;

use image::{Rgb, RgbImage};
use ndarray::{azip, Array, Array2, Ix1};
use num::complex::Complex;
use pbr::{MultiBar, ProgressBar};
use std::convert::TryInto;
use std::f64;

struct RunningPixel {
    c: Complex<f64>,
    z: Complex<f64>,
}

enum Iteration {
    Running(RunningPixel),
    Stopped(Complex<f64>),
}

pub fn mandel_brot(mandel: &mut RgbImage, iters: usize) {
    println!("Mandel:");
    let width = mandel.width();
    let height = mandel.height();
    let width_size: usize = width.try_into().unwrap();
    let height_size: usize = height.try_into().unwrap();

    let locwidth = 3f64;
    let center = Complex::new(-0.5, 0.0);
    let scale = locwidth / width as f64;

    let half_width: f64 = width as f64 / 2.0;
    let half_height: f64 = height as f64 / 2.0;
    let mut values = Array::from_shape_fn((height_size, width_size), |(y, x)| {
        let r = (x as f64 - half_width) * scale + center.re;
        let i = (y as f64 - half_height) * scale + center.im;
        let c = Complex::new(r, i);
        let z = Complex::new(0.0, 0.0);
        Iteration::Running(RunningPixel { c, z })
    });

    for i in 0..iters {
        let mut pixels2d = mandel
            .pixels_mut()
            .collect::<Array<&mut Rgb<u8>, Ix1>>()
            .into_shape((height_size, width_size))
            .unwrap();
        azip!((p in &mut pixels2d, it in &mut values) {
            *it = match it {
                Iteration::Running(RunningPixel { ref c, ref z }) => {
                    if z.norm_sqr() > 128.0 {
                        let r = 0 + (i * 10) as u8;
                        let g = 0 + (i * 15) as u8;
                        let b = 255 - (i * 10) as u8;
                        **p = Rgb([r, g, b]);
                        Iteration::Stopped(*c)
                    } else {
                        let z_new = z * z + c;
                        Iteration::Running(RunningPixel { c: *c, z: z_new })
                    }
                }
                Iteration::Stopped(c) => Iteration::Stopped(*c),
            };
        });
    }
}

pub fn buddah_brot(img: &mut RgbImage, iters: usize, inverted: bool) {
    if inverted {
        print!("inverted ");
    }
    println!("Buddah:");

    let width = img.width();
    let height = img.height();
    let width_size: usize = width.try_into().unwrap();
    let height_size: usize = height.try_into().unwrap();

    let locwidth = 3f64;
    let center = Complex::new(-0.5, 0.0);
    let scale = locwidth / width as f64;

    let half_width: f64 = width as f64 / 2.0;
    let half_height: f64 = height as f64 / 2.0;

    let mut values = vec![];
    for x in 0..width {
        for y in 0..height {
            let r = (x as f64 - half_width) * scale + center.re;
            let i = (y as f64 - half_height) * scale + center.im;
            let c = Complex::new(r, i);
            let z = Complex::new(0.0, 0.0);
            values.push(Iteration::Running(RunningPixel { c, z }));
        }
    }

    let count = width_size * height_size;
    let num_threads = 4;
    let chunksize = count / num_threads;
    //let iterations diverge
    println!("Pre-Iterations:");

    let _ = crossbeam::scope(|scope| {
        let mut children = vec![];

        let mut mb = MultiBar::new();

        for chunk in values.chunks_mut(chunksize) {
            let mut p = mb.create_bar(iters as u64);

            children.push(scope.spawn(move |_| {
                for _ in 0..iters {
                    for it in chunk.iter_mut() {
                        *it = match it {
                            Iteration::Running(RunningPixel { ref c, ref z }) => {
                                if z.norm_sqr() > 128.0 {
                                    Iteration::Stopped(*c)
                                } else {
                                    let z_new = z * z + c;
                                    Iteration::Running(RunningPixel { c: *c, z: z_new })
                                }
                            }
                            Iteration::Stopped(c) => Iteration::Stopped(*c),
                        };
                    }
                    p.inc();
                }
                p.finish();
            }));
        }
        mb.listen();
        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }
    });

    //reset z
    if !inverted {
        for it in values.iter_mut() {
            *it = match it {
                Iteration::Running(RunningPixel { c, z: _ }) => Iteration::Stopped(*c),
                Iteration::Stopped(c) => Iteration::Running(RunningPixel {
                    c: *c,
                    z: Complex::new(0.0, 0.0),
                }),
            };
        }
    } else {
        for it in values.iter_mut() {
            *it = match it {
                Iteration::Running(RunningPixel { c, z: _ }) => Iteration::Running(RunningPixel {
                    c: *c,
                    z: Complex::new(0.0, 0.0),
                }),
                Iteration::Stopped(c) => Iteration::Stopped(*c),
            };
        }
    }

    //track remaining iterations
    let mut visits = Array2::<f64>::zeros((height_size, width_size));
    println!("\nMain-Iterations:");
    let mut pb = ProgressBar::new(iters as u64);

    for _ in 0..iters {
        let mut found_running = false;
        for it in values.iter_mut() {
            *it = match it {
                Iteration::Running(RunningPixel { ref c, ref z }) => {
                    found_running = true;
                    if z.norm_sqr() > 128.0 {
                        Iteration::Stopped(*c)
                    } else {
                        let z_new = z * z + c;
                        let x = (((z_new - center).re / scale).floor() + half_width) as usize;
                        let y = (-((z_new - center).im / scale).floor() + half_height) as usize;
                        // let x : usize = (x as i64).try_into().unwrap();
                        // let y : usize = (y as i64).try_into().unwrap();
                        if x < width_size && y < height_size {
                            visits[(y, x)] += 1f64;
                        }
                        Iteration::Running(RunningPixel { c: *c, z: z_new })
                    }
                }
                Iteration::Stopped(c) => Iteration::Stopped(*c),
            };
        }

        if !found_running {
            break;
        }
        pb.inc();
    }
    pb.finish();

    let maxvisit = visits.fold(0f64, |b, a| b.max(*a));
    println!("maxvisit: {}", maxvisit);
    visits = visits.map(|a| a / maxvisit);

    let mut pixels2d = img
        .pixels_mut()
        .collect::<Array<&mut Rgb<u8>, Ix1>>()
        .into_shape((height_size, width_size))
        .unwrap();
    azip!((p in &mut pixels2d, vis in &mut visits){
        let r = (255f64 * (*vis).ln() *1.0) as u8;
        let g = (255f64 * (*vis).ln() *1.0) as u8;
        let b = (255f64 * (*vis).ln() *1.0) as u8;

        **p = Rgb([r, g, b]);
    });
}
