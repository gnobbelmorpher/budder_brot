use image::{ImageBuffer, RgbImage};
mod budder;

use budder::{inverted_buddah_brot, mandel_brot};

pub fn run(width: u32, height: u32, iters: usize, mandel: bool, ibuddah: bool) {
    if mandel {
        let mut mandel: RgbImage = ImageBuffer::new(width, height);
        mandel_brot(&mut mandel, iters);
        let name = format!("mandel({}x{})_{}.png", width, height, iters);
        mandel.save(name).unwrap();
    }

    if ibuddah {
        let mut ibuddah: RgbImage = ImageBuffer::new(width, height);
        inverted_buddah_brot(&mut ibuddah, iters);
        let name = format!("ibuddah({}x{})_{}.png", width, height, iters);
        ibuddah.save(name).unwrap();
    }
}
