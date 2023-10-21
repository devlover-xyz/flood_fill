extern crate image;

use image::{ImageBuffer, Rgb};

fn flood_fill(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    target_color: Rgb<u8>,
    replacement_color: Rgb<u8>,
) {
    let mut stack = vec![(x, y)];
    let (width, height) = img.dimensions();

    while let Some((px, py)) = stack.pop() {
        if px >= width || py >= height {
            continue;
        }

        let current_color = *img.get_pixel(px, py);

        if current_color != target_color {
            continue;
        }

        img.put_pixel(px, py, replacement_color);

        for &(nx, ny) in &[(px - 1, py), (px + 1, py), (px, py - 1), (px, py + 1)] {
            if nx < width && ny < height {
                stack.push((nx, ny));
            }
        }
    }
}

fn main() {
    //load an image
    let img = image::open("original.png").unwrap().to_rgb8();

    // Define coordinates and colors
    let start_x = 100;
    let start_y = 100;
    let target_color: Rgb<u8> = Rgb([255, 255, 0]);
    let replacement_color = Rgb([0, 255, 0]);

    // Clone the image and apply flood fill
    let mut modified_img = img.clone();
    flood_fill(&mut modified_img, start_x, start_y, target_color, replacement_color);

    // Save the modified image
    modified_img.save("output_image.png").unwrap();
}
