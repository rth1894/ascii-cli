mod args;

pub use args::{Config, check};

use image::{ImageReader, DynamicImage};
use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use image::imageops::FilterType;

pub fn get_info(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let terminal = config.terminal;
    let input_path = config.input_file;
    let output_path = config.output_file;

    let mut file = fs::File::open(&input_path).map_err(|e| format!("Failed to open input file '{}': {}", input_path, e))?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| format!("Failed to read input file '{}': {}", input_path, e))?;

    let img = ImageReader::new(Cursor::new(&bytes))
        .with_guessed_format()?
        .decode()?
        .grayscale();

    if let Some(ref output) = output_path {
        let mut out_file = File::create(output)?;
        img.write_to(&mut out_file, image::ImageFormat::Png)?;
    }

    let mut grayscale_output = File::create("grayscale.png")?;
    img.write_to(&mut grayscale_output, image::ImageFormat::Png)?;

    convert(img, terminal, output_path)?;

    Ok(())
}

pub fn convert(image: DynamicImage, terminal: bool, output_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let target_width = 80;
    let scale_factor = 0.5;

    let original_width = image.width();
    let original_height = image.height();

    let new_height = ((original_height as f32) * (target_width as f32 / original_width as f32) * scale_factor) as u32;
    let resized_image = image.resize_exact(target_width, new_height, FilterType::Lanczos3);

    let gray_image = resized_image.to_luma8();

    if terminal {
        for y in 0..gray_image.height() {
            for x in 0..gray_image.width() {
                let pixel = gray_image.get_pixel(x, y);
                let value = pixel[0];
                let character = grayscale_to_ascii(value);
                print!("{}", character);
            }
            println!();
        }
    } else {
        let mut output = String::new();
        for y in 0..gray_image.height() {
            for x in 0..gray_image.width() {
                let pixel = gray_image.get_pixel(x, y);
                let value = pixel[0];
                let character = grayscale_to_ascii(value);
                output.push(character);
            }
            output.push('\n');
        }

        if let Some(path) = output_path {
            let mut file = File::create(path)?;
            file.write_all(output.as_bytes())?;
        } else {
            eprintln!("Error: output_path is required when terminal mode is off.");
        }
    }

    Ok(())
}

fn grayscale_to_ascii(value: u8) -> char {
    const ASCII_CHARS: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let index = (value as usize * (ASCII_CHARS.len() - 1)) / 255;
    ASCII_CHARS[index]
}

/*
1. read the image, send err if none
2. grayscale the image
3. finalize the ascii characters
4. normalize values to pixel range
5. find ascii characters for range, append to file OR print in terminal
6. PLEASE ALSO DO THIS
        - color ascii art (woww)
        - multithreading (damnnn)
        - experiment with .md file
        - package the cli with docker or --install
        - ADD UNIT TESTS
*/

