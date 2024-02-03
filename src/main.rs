use image::{ImageBuffer, Rgba, io::Reader as ImageReader};
use std::env;

const BASE_COLOR : Rgba<u8> = Rgba([153, 51, 255, 255]);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: \n  To encode: {} encode \"Example string\"\n  To decode: {} decode image.png", args[0], args[0]);
        return;
    }

    match args[1].as_str() {
        "encode" => {
            if args.len() != 3 {
                println!("Usage: {} encode \"Example string\"", args[0]);
                return;
            }
            let input = &args[2];
            let pixel_size = 10;
            let imgbuf = string_to_pixels(input, pixel_size, BASE_COLOR);
            let output_path = "encoded_image.png";
            imgbuf.save(output_path).unwrap();
            println!("Message encoded into {}", output_path);
        }
        "decode" => {
            if args.len() != 3 {
                println!("Usage: {} decode image.png", args[0]);
                return;
            }
            let img_path = &args[2];
            let pixel_size = 10;
            match decode_pixels_to_string(img_path, pixel_size, BASE_COLOR) {
                Ok(message) => println!("Decoded message: {}", message),
                Err(e) => println!("Error decoding message: {}", e),
            }
        }
        _ => println!("Invalid command. Use 'encode' or 'decode'."),
    }
}

fn string_to_pixels(input: &str, pixel_size: u32, base_color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let bytes = input.as_bytes();
    let num_pixels = bytes.len() as u32;
    let img_size = ((num_pixels as f32).sqrt().ceil() as u32) * pixel_size;
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(img_size, img_size);

    for (i, &byte) in bytes.iter().enumerate() {
        let x = (i as u32 % (img_size / pixel_size)) * pixel_size;
        let y = (i as u32 / (img_size / pixel_size)) * pixel_size;

        let transformed_color = Rgba([
            (base_color[0] as u16 + byte as u16).min(255) as u8,
            (base_color[1] as u16 + byte as u16).min(255) as u8,
            (base_color[2] as u16 + byte as u16).min(255) as u8,
            255,
        ]);

        for dx in 0..pixel_size {
            for dy in 0..pixel_size {
                imgbuf.put_pixel(x + dx, y + dy, transformed_color);
            }
        }
    }

    imgbuf
}

fn decode_pixels_to_string(img_path: &str, pixel_size: u32, base_color: Rgba<u8>) -> Result<String, std::string::FromUtf8Error> {
    let img = ImageReader::open(img_path).unwrap().decode().unwrap().to_rgba8();
    let img_width = img.width();

    let mut bytes = Vec::new();

    for y in (0..img.height()).step_by(pixel_size as usize) {
        for x in (0..img_width).step_by(pixel_size as usize) {
            let pixel = img.get_pixel(x, y);

            let r_diff = if pixel[0] > base_color[0] { pixel[0] - base_color[0] } else { 0 };
            let g_diff = if pixel[1] > base_color[1] { pixel[1] - base_color[1] } else { 0 };
            let b_diff = if pixel[2] > base_color[2] { pixel[2] - base_color[2] } else { 0 };

            let max_diff = r_diff.max(g_diff).max(b_diff);
            bytes.push(max_diff);
        }
    }

    String::from_utf8(bytes)
}