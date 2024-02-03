# Pyxelium [![Rust](https://github.com/Rikatemu/pyxelium/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Rikatemu/pyxelium/actions/workflows/rust.yml)

![Screenshot](encoded_image.png)

Pixelithos is a lightweight pixel-based encryption tool that allows you to encode and decode messages within PNG images. It provides a command-line interface (CLI) for encoding and decoding messages using pixel-based encryption.

## Features

- Encode text messages into images.
- Decode hidden messages from images.
- Secure and robust pixel-based encryption.
- Simple and easy-to-use CLI interface.

## Usage

### Encoding a Message

To encode a message into an image, use the following command:

```shell
$ pyxelium encode "Your message goes here"
```

This command will encode your message into an image named "encoded_image.png."

### Decoding a Message

To decode a message from an image, use the following command:

```shell
$ pyxelium decode image.png
```

Replace "image.png" with the path to the image you want to decode. The decoded message will be displayed in the terminal.

## Getting Started

1. Clone the repository

```shell
$ git clone https://github.com/Rikatemu/pyxelium.git
```

2. Navigate to the project directory:

```shell
$ cd pixelithos
```

3. Run the CLI tool to encode or decode messages.

## Contributing

Contributions to Pixelithos are welcome!

## License

This project is open-source and available under the MIT License. You are free to use, modify, and distribute this software.