use image::{ImageBuffer, Rgba, RgbaImage};

/// Encodes a string message into an image without saving it to a file.
///
/// # Arguments
///
/// * `input` - The input string to encode.
/// * `pixel_size` - The size of each data pixel in the output image. Used for aesthetic purposes.
/// * `base_color` - The base color used for encoding. Acts as a secret key.
///
/// # Returns
///
/// The encoded image as an `RgbaImage`.
pub fn string_to_pixels(
    input: &str,
    pixel_size: u32,
    base_color: Rgba<u8>,
) -> RgbaImage {
    let bytes = input.as_bytes();
    let num_pixels = bytes.len() as u32;
    let img_size = ((num_pixels as f32).sqrt().ceil() as u32) * pixel_size;
    let mut imgbuf: RgbaImage = ImageBuffer::new(img_size, img_size);

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

/// Decodes an image into a string message.
///
/// # Arguments
///
/// * `image` - The image to decode as an `RgbaImage`.
/// * `pixel_size` - The size of each data pixel in the input image. Used for aesthetic purposes. Must be the same as the one used for encoding.
/// * `base_color` - The base color used for encoding. Acts as a secret key. Must be the same as the one used for encoding.
///
/// # Returns
///
/// A `Result` containing the decoded string or an error.
pub fn decode_pixels_to_string(
    image: &RgbaImage,
    pixel_size: u32,
    base_color: Rgba<u8>,
) -> Result<String, std::string::FromUtf8Error> {
    let img_width = image.width();

    let mut bytes = Vec::new();

    for y in (0..image.height()).step_by(pixel_size as usize) {
        for x in (0..img_width).step_by(pixel_size as usize) {
            let pixel = image.get_pixel(x, y);

            let r_diff = if pixel[0] > base_color[0] {
                pixel[0] - base_color[0]
            } else {
                0
            };
            let g_diff = if pixel[1] > base_color[1] {
                pixel[1] - base_color[1]
            } else {
                0
            };
            let b_diff = if pixel[2] > base_color[2] {
                pixel[2] - base_color[2]
            } else {
                0
            };

            let max_diff = r_diff.max(g_diff).max(b_diff);
            bytes.push(max_diff);
        }
    }

    // Create a string from the bytes and trim null characters
    let decoded_string = String::from_utf8(bytes)?;
    let trimmed_string = decoded_string.trim_matches('\0').to_string();

    Ok(trimmed_string)
}