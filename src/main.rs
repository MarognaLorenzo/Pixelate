mod functions;

use image::{ColorType, DynamicImage, GenericImageView};
use easy_paths::get_absolute_path;
use image::imageops::FilterType;
use crate::functions::{avarage, separate_colors, sum_images};


fn main() {
    let path = get_absolute_path(&"./images/samples/retina.jpg");
    println!("{:?}", path);
    if easy_paths::is_existing_path(&path) {

        let mut img = image::open(path).unwrap();
        img.crop(100,0,500,500).save("images/output/cropped.png").unwrap();
        img.adjust_contrast(30.0).save("images/output/contrast.png").unwrap();
        img.blur(-40.0).save("images/output/blurred.png").unwrap();
        img.brighten(-100).save("images/output/brightened.png").unwrap();
        img.grayscale().save("images/output/grayscale.png").unwrap();
        img.filter3x3(&[0.5 , 1. , 0.5 , 1. , 2. , 1. , 0.5 , 1. , 0.5]).save("images/output/filtered.png").unwrap();
        img.thumbnail(1400,1000).save("images/output/thimbnail.png").unwrap();
        img.resize(300,455, FilterType::Triangle).save("images/output/resized.png").unwrap();
        let (r,g,b) = separate_colors(&img).unwrap();
        r.save("images/output/decomposition_test/red.png").unwrap();
        g.save("images/output/decomposition_test/green.png").unwrap();
        b.save("images/output/decomposition_test/blue.png").unwrap();
        let summed = sum_images(&b, &g).unwrap();
        summed.save("images/output/sum_test/summed.png").unwrap();
        let average = avarage(&b, &g).unwrap();
        average.save("images/output/average_test/average.png").unwrap();
    }
}