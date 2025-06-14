use std::{
    env,
    fs::File,
    io::{self, Read},
};

use image::{ImageBuffer, Rgb};
use sha2::{Digest, Sha256};

fn main() {
    const HELP: &str = "
Creates a unique visual representation of a text, or file.

USAGE: visuhash [TEXT]

Options :
    -f, --file      set input file path
    -o --output     set output image file path/name

Examples :
    visuhash Hello World
    visuhash --file hello.zip --output hi.png
    ";

    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("{}", HELP);
    }

    let mut output_file: &str = "output.png";

    let mut input = String::new();

    let mut file_specified = false;
    let mut output_specified = false;

    let mut use_file_input = false;
    let mut file_input_path = String::new();

    for i in 1..args.len() {
        if file_specified {
            use_file_input = true;
            file_input_path = args[i].clone();
        } else if output_specified {
            output_file = &args[i];
        } else {
            input = format!("{} {}", input, args[i]);
        }

        file_specified = args[i] == "-f" || args[i] == "--file";

        output_specified = args[i] == "-o" || args[i] == "--output";
    }

    println!("{}", input);

    let result_bits: Vec<u8>;

    if use_file_input {
        result_bits = match file_to_bits(file_input_path) {
            Ok(r) => r,
            Err(e) => {
                panic!("Error with file : {}", e);
            }
        };
    } else {
        result_bits = match text_to_bits(input) {
            Ok(r) => r,
            Err(e) => {
                panic!("Error with text : {}", e);
            }
        };
    }

    let width = ((result_bits.len() as f32).sqrt()).floor() as u32;
    let height = (result_bits.len() / width as usize) as u32;

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

    img.save(&output_file).unwrap();

    println!("Image saved as {} !", output_file);
}

fn text_to_bits(text: String) -> Result<Vec<u8>, io::Error> {
    let mut hasher = Sha256::new();

    hasher.update(text);
    let result = hasher.finalize();

    let bytes: &[u8] = result.as_slice();
    let bits: Vec<u8> = bytes_to_bits(bytes);

    Ok(bits)
}

fn file_to_bits(file_path: String) -> Result<Vec<u8>, io::Error> {
    let mut hasher = Sha256::new();
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    let bits = bytes_to_bits(result.as_slice());
    Ok(bits)
}

fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits = Vec::new();

    for &byte in bytes {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            bits.push(bit);
        }
    }

    return bits;
}
