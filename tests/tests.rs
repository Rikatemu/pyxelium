#[cfg(test)]
mod tests {
    use image::Rgba;
    use pyxelium::{decode_pixels_to_string, string_to_pixels};

    #[test]
    fn test_encode_decode() {
        // Test encoding a simple string into an image
        let input = "Hello, World!";
        let pixel_size = 5;
        let base_color = Rgba([0, 0, 0, 255]);

        let encoded_image = string_to_pixels(input, pixel_size, base_color);

        // Verify that the encoded image is not empty
        assert!(!encoded_image.is_empty());

        // Verify that the encoded image has the correct dimensions
        let expected_width = 15;
        let expected_height = 15;
        assert_eq!(encoded_image.width(), expected_width);
        assert_eq!(encoded_image.height(), expected_height);

        // Decode the image and verify the result
        let decoded_string = decode_pixels_to_string(&encoded_image, pixel_size, base_color).unwrap();
        assert_eq!(decoded_string, "Hello, World!");

        // Decoding with the wrong base color should fail
        let wrong_base_color = Rgba([255, 255, 255, 255]);
        let result = decode_pixels_to_string(&encoded_image, pixel_size, wrong_base_color).unwrap();
        assert_ne!(result, "Hello, World!")
    }
}