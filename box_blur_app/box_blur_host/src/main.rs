use std::path::{Path}; // Rust path module
use std::env; // Rust command line argument module
use image::{ImageBuffer, Rgb, RgbImage}; // Rust image processing modules

// Same dimensions as the one in box_blur.h
const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = 480;

// FFI Declaration for the C++ box blur function from HLS
unsafe extern "C" {
    fn box_blur_filter(input_pixels: *mut u8, output_pixels: *mut u8);
}

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Box Blur Filter Host Script (Rust FFI) ---");

    //Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let input_image_path_str: &str;
    if args.len() > 1 {
        // If an argument is provided, use it as the input image path
        input_image_path_str = &args[1];
        println!("Using image from command line: {}", input_image_path_str);
    } else {
        // If no argument, create a dummy image as before
        input_image_path_str = "sample_input_image.png";
        if !Path::new(input_image_path_str).exists() {
            println!("No input image path provided. Creating a dummy input image: {}", input_image_path_str);
            //Create an image exactly like the one in the testbench
            let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
            for y in 0..IMAGE_HEIGHT {
                for x in 0..IMAGE_WIDTH {
                    if x % 20 < 10 {
                        img.put_pixel(x, y, image::Rgb([255, 0, 0]));
                    } else if y % 20 < 10 {
                        img.put_pixel(x, y, image::Rgb([255, 0, 0])); 
                    } else {
                        img.put_pixel(x, y, image::Rgb([0, 255, 0]));
                    }
                }
            }
            img.save(input_image_path_str)?;
            println!("Dummy image created.");
        } else {
            println!("No input image path provided. Using existing dummy image: {}", input_image_path_str);
        }
    }
    
    // Catch any errors arising from the path and name the output image
    let input_path = Path::new(input_image_path_str);
    let filename_str = input_path.file_name()
        .ok_or("Invalid input path: missing filename")?
        .to_str()
        .ok_or("Invalid filename: not valid UTF-8")?;
    let output_image_path = format!("output_{}", filename_str);


    // Load the input image
    let input_image = image::open(input_image_path_str)?.to_rgb8();
    
    // Resize if dimensions don't match 
    let input_image = if input_image.width() != IMAGE_WIDTH || input_image.height() != IMAGE_HEIGHT {
        println!(
            "Resizing input image from {}x{} to {}x{}",
            input_image.width(), input_image.height(), IMAGE_WIDTH, IMAGE_HEIGHT
        );
        image::imageops::resize(&input_image, IMAGE_WIDTH, IMAGE_HEIGHT, image::imageops::FilterType::Lanczos3)
    } else {
        input_image // dimensions match
    };

    // Get pixel data as a flat Vec<u8>
    let mut input_pixels_flat_rgb = input_image.into_raw();
    println!(
        "Input image loaded: {}x{} pixels. Data size: {} bytes.",
        IMAGE_WIDTH, IMAGE_HEIGHT, input_pixels_flat_rgb.len()
    );

    // Prepare output buffer
    let mut output_pixels_flat_rgb = vec![0u8; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];
    println!("Output buffer created: {} bytes.", output_pixels_flat_rgb.len());


    // ** Call the C/C++ box_blur_filter function via FFI **
    println!("\n--- Calling C/C++ Filter via FFI ---");
    unsafe {
        let input_ptr = input_pixels_flat_rgb.as_mut_ptr();
        let output_ptr = output_pixels_flat_rgb.as_mut_ptr();
        box_blur_filter(input_ptr, output_ptr); // Call the compiled C executable
    }
    println!("C/C++ box_blur_filter call complete.");

    // Convert the raw output data back to an ImageBuffer
    let output_image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, output_pixels_flat_rgb)
            .ok_or("Failed to create ImageBuffer from raw data")?;

    // Save the output image
    output_image_buffer.save(&output_image_path)?;
    println!("Blurred output image saved to: {}", output_image_path);
    println!("\n--- End of Box Blur Host Script ---");
    println!(
        "Compare '{}' with '{}' to see the blur effect.",
        input_image_path_str, output_image_path
    );
    println!("Remember to use 'LD_LIBRARY_PATH=. cargo run -- <path_to_your_image.png>'");
    Ok(())
}
