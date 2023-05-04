use image::DynamicImage;

enum PixelColor {
    Red,
    Green,
    Blue,
}

struct pixel {
    color: PixelColor,
}

pub fn separate_colors(starting_image: DynamicImage) -> Result<(DynamicImage, DynamicImage, DynamicImage), String> {
    let opened = starting_image.as_rgb8().unwrap();
    let img = opened.as_raw();
    let mut scratch_img1 = DynamicImage::new_rgb8(starting_image.width(), starting_image.height());
    let mut scratch_img2 = DynamicImage::new_rgb8(starting_image.width(), starting_image.height());
    let mut scratch_img3 = DynamicImage::new_rgb8(starting_image.width(), starting_image.height());
    let red_image = scratch_img1.as_mut_rgb8().unwrap().as_mut();
    let green_image = scratch_img2.as_mut_rgb8().unwrap().as_mut();
    let blue_image = scratch_img3.as_mut_rgb8().unwrap().as_mut();
    for (i, ch) in img.iter().enumerate() {
        red_image[i] = 0;
        green_image[i] = 0;
        blue_image[i] = 0;
        match i%3 {
            0 => {red_image[i] = *ch},
            1 => {green_image[i] = *ch},
            _ => {blue_image[i] = *ch}
        }
    }
    Ok((scratch_img1, scratch_img2, scratch_img3))
}