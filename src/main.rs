use image::{ImageBuffer, Rgb};
use sha2::{Digest, Sha256};

fn main() {
    let input = b"Hello, world!";

    let mut hasher = Sha256::new();

    hasher.update(input);
    let result = hasher.finalize();

    let result_hex = result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    let result_bytes = result.as_slice();
    let mut result_bits = Vec::new();

    for &byte in result_bytes {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            result_bits.push(bit);
        }
    }

    let width = ((result_bits.len() as f32).sqrt()).floor() as u32;
    let height = ((result_bits.len() as f32).sqrt()).floor() as u32;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let colors;

            let bit_index = (y * width + x) as usize;
            let bit = result_bits[bit_index];

            if bit == 1 {
                colors = [255, 255, 255];
            } else {
                colors = [0, 0, 0];
            }
            img.put_pixel(x, y, Rgb(colors));
        }
    }

    let filename = result_hex[..6].to_owned() + ".png";
    img.save(&filename).unwrap();

    println!("Image saved as {} !", filename);
}
