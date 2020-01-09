use image::{ImageBuffer, RgbImage};
mod budder;

use budder::{buddah_brot, mandel_brot};

pub fn run(width: u32, height: u32, iters: usize, mandel: bool, buddah: bool) {
    if mandel {
        let mut mandel: RgbImage = ImageBuffer::new(width, height);
        mandel_brot(&mut mandel, iters);
        let name = format!("mandel({}x{})_{}.png", width, height, iters);
        mandel.save(name).unwrap();
    }

    if buddah {
        let mut buddah: RgbImage = ImageBuffer::new(width, height);
        buddah_brot(&mut buddah, iters);
        let name = format!("buddah({}x{})_{}.png", width, height, iters);
        buddah.save(name).unwrap();
    }
}
