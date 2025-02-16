use image::{DynamicImage, GenericImageView, ImageEncoder, ExtendedColorType::Rgba8};
use image::codecs::png;
use base64::{Engine as _, engine::general_purpose};
fn merge_sort(input: &Vec<(u32, u32, u32, image::Rgba<u8>)>) -> Vec<(u32, u32, u32, image::Rgba<u8>)> {
    // base case
    if input.len() < 2 {
        return input.to_vec();  // Ensure that we return the vector here
    } else {
        // splits the vector into two equal-ish halves
        let size = input.len() / 2;
        let left = merge_sort(&input[0..size].to_vec());
        let right = merge_sort(&input[size..].to_vec());

        // sorts and merges the two halves, then returns
        let merged = merge(&left, &right);
        merged
    }
}

fn merge(left: &Vec<(u32, u32, u32, image::Rgba<u8>)>, right: &Vec<(u32, u32, u32, image::Rgba<u8>)>) -> Vec<(u32, u32, u32, image::Rgba<u8>)> {
    let mut i = 0;
    let mut j = 0;
    let mut output: Vec<(u32, u32, u32, image::Rgba<u8>)> = Vec::new();

    // performing the actual sort, and appending values to the buffer
    while i < left.len() && j < right.len() {
        if left[i].0 < right[j].0 {
            output.push(left[i]);
            i += 1;
        } else {
            output.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        output.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        output.push(right[j]);
        j += 1;
    }

    output
}

fn sort_deviance_load(red: u8, green: u8, blue: u8, img: DynamicImage) -> String {
    // loading original photo and creating the buffer
    let (imgx, imgy) = img.dimensions();
    let mut pixels = Vec::new();
    let mut buffer = image::ImageBuffer::new(imgx, imgy);
    //looping through and creating a 1-dimensional array of each pixel and it's deviance
    for (x, y, pixel) in img.pixels() {
        let mut deviance: u32 = 0;
        if red > pixel[0] {
            deviance += red as u32 - pixel[0] as u32 ;
        } else {
            deviance += pixel[0] as u32 - red as u32 ;
        }

        if green > pixel[1] {
            deviance += green as u32  - pixel[1] as u32 ;
        } else {
            deviance +=  pixel[1] as u32  - green as u32 ;
        }

        if blue > pixel[2] {
            deviance += blue as u32  - pixel[2] as u32 ;
        } else {
            deviance +=  pixel[2] as u32  - blue as u32;
        }
        pixels.push((deviance, x, y, pixel));
    }
    // merge sorts the vector based on the total value of their RGB channels 
    let sorted = merge_sort(&pixels);
    // copies the sorted pixel data back to the buffer in a gradient-fashion
    let mut i = 0;
    for d in 0..(imgy + imgx - 1) {
        for y in (0..=d).rev() {
            let x = d - y;
            if y < imgy && x < imgx {
                buffer.put_pixel(x, y, sorted[sorted.len() - i - 1].3);
                i += 1; 
            }
        }
    }
        
    // finally, saves the image
    //let location = format!("sorted.png", output_path);
    let mut output_bytes = Vec::new();
    let encoder = png::PngEncoder::new(&mut output_bytes);
    encoder.write_image(&buffer, imgx, imgy, Rgba8).unwrap();
    let output_b64 = general_purpose::STANDARD.encode(output_bytes);
    output_b64

}

#[tauri::command]
fn process(input: &str, red: u8, green: u8, blue: u8) -> String {
    // decode the image from front-end
    let decoded_image_bytes: Vec<u8> = general_purpose::STANDARD.decode(input).unwrap();
    // writing to temporary location
    let img = image::load_from_memory(&decoded_image_bytes);
    // file writes to location, but app crashes right after
    let output = sort_deviance_load(red, green, blue, img.expect("Whoah there buddy image mightve loaded inproperly"));
    output
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
