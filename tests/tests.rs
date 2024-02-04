#[cfg(test)]
mod tests {
    use image::Rgba;
    use pyxelium::{decode_pixels_to_string, string_to_pixels, count_overflow_pixels};

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

    #[test]
    fn test_count_overflow_pixels() {
        // Case 1 - no overflow pixels, because the base color is 0,0,0
        let case1_chunks = vec![vec![0, 0, 0],
                                vec![0, 0, 0],
                                vec![0, 0, 0]];
        let case1_base = Rgba([0, 0, 0, 255]);
        assert_eq!(count_overflow_pixels(&case1_chunks, case1_base), 0);

        // Case 2 - 9 overflow pixels, because the base color is on the limit and all of the pixels will overflow
        let case2_chunks = vec![vec![255, 255, 255],
                                vec![255, 255, 255],
                                vec![255, 255, 255]];
        let case2_base = Rgba([255, 255, 255, 255]);
        assert_eq!(count_overflow_pixels(&case2_chunks, case2_base), 9);

        // Case 3 - 6 overflow pixels, because the base color is fairly high, so some of them will overflow
        let case3_chunks = vec![vec![100, 200, 200],
                                vec![100, 100, 100],
                                vec![200, 100, 200]];
        let case3_base = Rgba([200, 100, 100, 255]);
        assert_eq!(count_overflow_pixels(&case3_chunks, case3_base), 6);
    }
}
