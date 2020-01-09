use image::{ImageBuffer, RgbImage};
mod budder;

use budder::{buddah_brot, mandel_brot};

pub fn run(width: u32, height: u32, iters: u32, mandel: bool, buddah: bool) {
    if mandel {
        let mut mandel: RgbImage = ImageBuffer::new(width, height);
        mandel_brot(&mut mandel, iters);
        mandel.save("mandel.png").unwrap();
    }

    if buddah {
        let mut buddah: RgbImage = ImageBuffer::new(width, height);
        buddah_brot(&mut buddah, iters);
        buddah.save("buddah.png").unwrap();
    }
}
