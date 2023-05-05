use std::cmp::min;
use easy_paths::get_absolute_path;
use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;

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

pub fn manupulation() {
    return;
    let path = get_absolute_path(&"./images/samples/retina.jpg");
    println!("{:?}", path);
    if easy_paths::is_existing_path(&path) {
        let mut img = image::open(path).unwrap();
        img.crop(100, 0, 500, 500).save("images/output/cropped.png").unwrap();
        img.adjust_contrast(30.0).save("images/output/contrast.png").unwrap();
        img.blur(-40.0).save("images/output/blurred.png").unwrap();
        img.brighten(-100).save("images/output/brightened.png").unwrap();
        img.grayscale().save("images/output/grayscale.png").unwrap();
        img.filter3x3(&[0.5, 1., 0.5, 1., 2., 1., 0.5, 1., 0.5]).save("images/output/filtered.png").unwrap();
        img.thumbnail(1400, 1000).save("images/output/thimbnail.png").unwrap();
        img.resize(300, 455, FilterType::Triangle).save("images/output/resized.png").unwrap();
        let (r, g, b) = separate_colors(&img).unwrap();
        r.save("images/output/decomposition_test/red.png").unwrap();
        g.save("images/output/decomposition_test/green.png").unwrap();
        b.save("images/output/decomposition_test/blue.png").unwrap();
        let summed = sum_images(&b, &g).unwrap();
        summed.save("images/output/sum_test/summed.png").unwrap();
        let average = avarage(&b, &g).unwrap();
        average.save("images/output/average_test/average.png").unwrap();
    }
}