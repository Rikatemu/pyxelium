use image::{ImageBuffer, Rgba, RgbaImage};

/// Encodes a string message into an image. The function checks the length
/// of the input string to ensure it does not exceed the maximum allowed length of 765 bytes, which
/// is determined by the worst-case scenario where all pixels overflow and pixel indexes in overflows
/// have a limit of 255. If the input string exceeds this length, the function will panic.
///
/// # Arguments
///
/// * `input` - The input string to encode. Must not exceed 765 bytes in length.
/// * `pixel_size` - The size of each data pixel in the output image. Used for aesthetic purposes.
/// * `base_color` - The base color used for encoding. Acts as a secret key.
///
/// # Returns
///
/// The encoded image as an `RgbaImage`. If the input string exceeds the maximum allowed length,
/// the function will terminate the program with a panic.
///
/// # Panics
///
/// Panics if the input string exceeds 765 bytes in length.
pub fn string_to_pixels(
    input: &str,
    pixel_size: u32,
    base_color: Rgba<u8>,
) -> RgbaImage {
    let bytes = input.as_bytes();

    // Check if the input string exceeds the maximum allowed length
    if bytes.len() > 765 {
        panic!("Input string exceeds the maximum allowed length of 765 bytes.");
    }

    let mut chunks = Vec::new();
    let mut overflow_data = Vec::new();

    // Prepare chunks of bytes
    for chunk in bytes.chunks(3) {
        let mut padded_chunk = chunk.to_vec();
        while padded_chunk.len() < 3 {
            padded_chunk.push(0); // Padding to ensure 3 bytes
        }
        chunks.push(padded_chunk);
    }

    let num_pixels = chunks.len() as u32;

    // Estimate additional space for overflow pixels. Assuming worst case: every pixel overflows.
    // Plus one more pixel for the separator and a rough estimate for overflow pixels.
    let estimated_overflow_pixels = count_overflow_pixels(&chunks, base_color);
    let total_pixels = num_pixels + estimated_overflow_pixels + 1; // +1 for the 0,0,0,0 separator pixel
    let img_size = ((total_pixels as f32).sqrt().ceil() as u32) * pixel_size;
    let mut imgbuf: RgbaImage = ImageBuffer::new(img_size, img_size);

    for (i, chunk) in chunks.iter().enumerate() {
        let x = (i as u32 % (img_size / pixel_size)) * pixel_size;
        let y = (i as u32 / (img_size / pixel_size)) * pixel_size;

        let (r, r_overflow) = add_with_overflow(base_color[0], chunk[0]);
        let (g, g_overflow) = add_with_overflow(base_color[1], chunk[1]);
        let (b, b_overflow) = add_with_overflow(base_color[2], chunk[2]);

        // Record overflow information
        if r_overflow > 0 {
            overflow_data.push((i as u32, 1, r_overflow));
        }
        if g_overflow > 0 {
            overflow_data.push((i as u32, 2, g_overflow));
        }
        if b_overflow > 0 {
            overflow_data.push((i as u32, 3, b_overflow));
        }

        // Set the pixel
        for y_offset in 0..pixel_size {
            for x_offset in 0..pixel_size {
                imgbuf.put_pixel(x + x_offset, y + y_offset, Rgba([r, g, b, 255]));
            }
        }
    }

    // Append a 0,0,0,0 pixel to signal the start of overflow data
    let overflow_start_index = num_pixels;
    let overflow_marker_x = (overflow_start_index % (img_size / pixel_size)) * pixel_size;
    let overflow_marker_y = (overflow_start_index / (img_size / pixel_size)) * pixel_size;
    for y_offset in 0..pixel_size {
        for x_offset in 0..pixel_size {
            imgbuf.put_pixel(overflow_marker_x + x_offset, overflow_marker_y + y_offset, Rgba([0, 0, 0, 0]));
        }
    }

    // Append overflow data
    for (i, &(index, channel, value)) in overflow_data.iter().enumerate() {
        let pixel_index = overflow_start_index + 1 + i as u32; // +1 to account for the 0,0,0,0 separator pixel
        // Calculate the position of the pixel in the image, ensuring it's within bounds
        let x = ((pixel_index % (img_size / pixel_size)) * pixel_size) % img_size;
        let y = ((pixel_index / (img_size / pixel_size)) * pixel_size) % img_size;
        for y_offset in 0..pixel_size {
            for x_offset in 0..pixel_size {
                imgbuf.put_pixel(x + x_offset, y + y_offset, Rgba([index as u8, channel, value, 255]));
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
    let mut overflow_pixels = Vec::new();
    let mut encountered_separator = false;

    for y in (0..image.height()).step_by(pixel_size as usize) {
        for x in (0..img_width).step_by(pixel_size as usize) {
            let pixel = image.get_pixel(x, y);
            if encountered_separator {
                // After encountering the separator, all pixels are part of the overflow information
                if pixel[3] > 0 { // Using alpha channel to ensure it's not the separator pixel
                    overflow_pixels.push((pixel[0] as usize, pixel[1], pixel[2]));
                }
            } else {
                // Before encountering the separator, pixels are part of the main data
                if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 && pixel[3] == 0 {
                    encountered_separator = true; // Found the separator pixel
                } else {
                    let r_diff = pixel[0].saturating_sub(base_color[0]);
                    let g_diff = pixel[1].saturating_sub(base_color[1]);
                    let b_diff = pixel[2].saturating_sub(base_color[2]);
                    bytes.push(r_diff);
                    bytes.push(g_diff);
                    bytes.push(b_diff);
                }
            }
        }
    }

    // Process overflow information after collecting all pixels
    for (index, channel, amount) in overflow_pixels {
        let byte_index = index * 3; // Convert pixel index to byte index
        match channel {
            1 => if byte_index < bytes.len() { bytes[byte_index] = bytes[byte_index].saturating_add(amount); },
            2 => if byte_index + 1 < bytes.len() { bytes[byte_index + 1] = bytes[byte_index + 1].saturating_add(amount); },
            3 => if byte_index + 2 < bytes.len() { bytes[byte_index + 2] = bytes[byte_index + 2].saturating_add(amount); },
            _ => {},
        }
    }

    // Remove the padding zeros that were added during encoding
    while let Some(&0) = bytes.last() {
        bytes.pop();
    }

    String::from_utf8(bytes)
}

/// Helper function for adding two u8s and handling overflow.
fn add_with_overflow(base: u8, add: u8) -> (u8, u8) {
    let sum = base as u16 + add as u16;
    if sum > 255 {
        (255, (sum - 255) as u8)
    } else {
        (sum as u8, 0)
    }
}

/// Helper function that calculates the required overflow pixles.
pub fn count_overflow_pixels(chunks: &Vec<Vec<u8>>, base_color: Rgba<u8>) -> u32 {
    let mut count: u32 = 0;

    for chunk in chunks {
        for i in 0..3 {
            if chunk[i] as u16 + base_color[i] as u16 > 255 {
                count += 1;
            }
        }
    }

    count
}
