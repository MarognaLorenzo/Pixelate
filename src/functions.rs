use std::cmp::min;
use image::{DynamicImage, GenericImageView};

enum PixelColor {
    Red,
    Green,
    Blue,
}

struct pixel {
    color: PixelColor,
}

pub fn separate_colors(starting_image: &DynamicImage) -> Result<(DynamicImage, DynamicImage, DynamicImage), String> {
    let img = starting_image.as_rgb8().unwrap();
    let mut scratch_img1 = DynamicImage::new_rgb8(starting_image.width(), starting_image.height());
    let mut scratch_img2 = DynamicImage::new_rgb8(starting_image.width(), starting_image.height());
    let mut scratch_img3 = DynamicImage::new_rgb8(starting_image.width(), starting_image.height());
    let mut red_image = scratch_img1.as_mut_rgb8().unwrap().pixels_mut();
    let mut green_image = scratch_img2.as_mut_rgb8().unwrap().pixels_mut();
    let mut blue_image = scratch_img3.as_mut_rgb8().unwrap().pixels_mut();
    img.pixels().for_each(|pi|{
        red_image.next().unwrap().0[0] = pi.0[0];
        green_image.next().unwrap().0[1] = pi.0[1];
        blue_image.next().unwrap().0[2] = pi.0[2];
    });
    Ok((scratch_img1, scratch_img2, scratch_img3))
}

pub fn sum_images(img1: &DynamicImage, img2: &DynamicImage) -> Result<DynamicImage,()> {
    if img1.dimensions() != img2.dimensions() {return Err(())}
    let mut scratch_img = DynamicImage::new_rgb8(img1.width(), img1.height());
    let mut result = scratch_img.as_mut_rgb8().unwrap().pixels_mut();
    let mut img1 = img1.as_rgb8().unwrap().pixels();
    let mut img2 = img2.as_rgb8().unwrap().pixels();
    result.for_each(|pi| {
        let extern_pixel1 = img1.next().unwrap().0;
        let extern_pixel2 = img2.next().unwrap().0;
        for (i, third_pi) in pi.0.iter_mut().enumerate() {
            *third_pi = min(255 as u16, (extern_pixel1[i] as u16) + (extern_pixel2[i] as u16 )) as u8;
        }
    });
    Ok(scratch_img)
}

pub fn avarage(img1: &DynamicImage, img2: &DynamicImage) -> Result<DynamicImage,()> {
    if img1.dimensions() != img2.dimensions() {return Err(())}
    let mut scratch_img = DynamicImage::new_rgb8(img1.width(), img1.height());
    let mut result = scratch_img.as_mut_rgb8().unwrap().pixels_mut();
    let mut img1 = img1.as_rgb8().unwrap().pixels();
    let mut img2 = img2.as_rgb8().unwrap().pixels();
    result.for_each(|pi| {
        let extern_pixel1 = img1.next().unwrap().0;
        let extern_pixel2 = img2.next().unwrap().0;
        for (i, third_pi) in pi.0.iter_mut().enumerate() {
            *third_pi = extern_pixel1[i] + extern_pixel2[i] / 2;
        }
    });
    Ok(scratch_img)
}