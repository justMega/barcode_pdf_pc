use image::{
    imageops::{contrast, filter3x3, grayscale, FilterType},
    ImageFormat,
};
use pdf2image::{RenderOptionsBuilder, PDF};
use rxing;
use std::{fs, path::PathBuf};

fn convert_first_page_to_jpg(mut file_name: PathBuf) -> PathBuf {
    // 1. get the pdf file
    let pdf = PDF::from_file(file_name.clone()).unwrap();
    // 2. convert the first page to image
    let pages = pdf
        .render(
            pdf2image::Pages::Range(1..=1),
            RenderOptionsBuilder::default().build().unwrap(),
        )
        .unwrap();
    // 3. Resize, crop, grayscale, increse contrast and sharpen image
    let upscaled_img = pages[0].resize(3072, 3072, FilterType::Lanczos3);
    let page_cropped = upscaled_img.crop_imm(0, 0, 3072, 615);
    let page_gray = grayscale(&page_cropped);
    let page_contrasted = contrast(&page_gray, 50.0);
    // Define a sharpening kernel
    let sharpening_kernel: [f32; 9] = [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
    // Apply the kernel to the image
    let sharpen_img = filter3x3(&page_contrasted, &sharpening_kernel);
    // 4. save image
    file_name.set_extension("jpg");
    let _ = sharpen_img.save_with_format(file_name.clone(), ImageFormat::Jpeg);
    return file_name.clone();
}

fn decode_barcode(file_name: PathBuf) -> Option<String> {
    // 1. try to read barcode from image
    let results = rxing::helpers::detect_in_file(file_name.to_str().unwrap(), None);
    // 2. handel case that reading barcode fails
    let decode = match results {
        Err(e) => {
            println!("there was an error when deocding barcode: {}", e);
            return None;
        }
        Ok(value) => value,
    };
    // 3. return value if read sucsesful
    println!("conversion sucesfull");
    println!("{} -> {}", decode.getBarcodeFormat(), decode.getText());
    return Some(decode.getText().to_string());
}

pub fn scane_directory(input_folder: &str, output_folder: &str) {
    println!("{}", input_folder);
    println!("{}", output_folder);
    let paths = fs::read_dir(input_folder).unwrap();

    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.extension().unwrap() != "pdf" {
            continue;
        }
        let image_path = convert_first_page_to_jpg(file_path.clone());
        let barcode_value = decode_barcode(image_path.clone());
        match barcode_value {
            Some(bv) => {
                let mut output_path = PathBuf::from(output_folder);
                output_path.push(bv);
                output_path.set_extension("pdf");
                println!(
                    "{} -> {}",
                    file_path.clone().display(),
                    output_path.clone().display()
                );
                fs::rename(file_path, output_path).unwrap();
            }
            None => println!(
                "could not read barcode from {}",
                image_path.clone().display()
            ),
        }

        let res = fs::remove_file(image_path);
        match res {
            Ok(()) => println!("deleted temporary file"),
            Err(e) => println!("there was an error deleting file {}", e),
        }
    }
}
