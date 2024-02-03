# Pyxelium [![Rust](https://github.com/Rikatemu/pyxelium/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Rikatemu/pyxelium/actions/workflows/rust.yml)

![Screenshot](hello_world.png)
![Screenshot](how_are_you.png)
![Screenshot](lorem_ipsum.png)

Pyxelium is a lightweight pixel-based encryption library that allows you to encode and decode messages within PNG images. It provides Rust functions for encoding and decoding messages using pixel-based encryption.

Why? I have no idea, but if you find any good reason to use this project, let me know!

## Features

- Encode text messages into images.
- Decode hidden messages from images.
- Secure and robust pixel-based encryption.

## Usage

### Encoding a Message

To encode a message into an image using Pyxelium, use the following Rust function:

```rust
use image::{ImageBuffer, Rgba, RgbaImage};
use pyxelium::string_to_pixels;

fn main() {
    let message = "Your message goes here";
    let pixel_size = 5;
    let base_color = Rgba([128, 128, 128, 255]);
    
    let encoded_image: RgbaImage = string_to_pixels(message, pixel_size, base_color);
    
    // You can now use `encoded_image` for further processing or display.
}
```

This function will encode your message into an RgbaImage object.

### Decoding a Message

To decode a message from an image using Pyxelium, use the following Rust function:

```rust
use image::{RgbaImage, io::Reader as ImageReader};
use pyxelium::decode_pixels_to_string;

fn main() {
    let image_path = "image.png";
    let pixel_size = 5;
    let base_color = Rgba([128, 128, 128, 255]);
    
    let encoded_image = ImageReader::open(image_path).unwrap().decode().unwrap().to_rgba8();
    
    let decoded_message_result = decode_pixels_to_string(&encoded_image, pixel_size, base_color);
    
    match decoded_message_result {
        Ok(decoded_message) => {
            println!("Decoded Message: {}", decoded_message);
        }
        Err(err) => {
            eprintln!("Error decoding message: {}", err);
        }
    }
}
```

Replace "image.png" with the path to the image you want to decode. The decoded message will be available as a Result<String, std::string::FromUtf8Error>.

### Getting Started

1. Add Pyxelium as a dependency to your Cargo.toml:
```toml
[dependencies]
pyxelium = "0.1.0"
```

2. Import the necessary modules and use the functions as shown in the usage examples above.

### Contributing

Contributions to Pyxelium are welcome! Feel free to open issues or submit pull requests.

### License

This project is open-source and available under the MIT License. You are free to use, modify, and distribute this software.
