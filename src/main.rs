mod functions;

use image::{DynamicImage, GenericImageView};
use easy_paths::get_absolute_path;
use crate::functions::separate_colors;


fn main() {
    let path = get_absolute_path(&"./images/samples/retina.jpg");
    println!("{:?}", path);
    if easy_paths::is_existing_path(&path) {
        let mut img = image::open(path).unwrap();
        let (r,g,b) = separate_colors(img).unwrap();
        r.save("./images/output/decomposition_test/red.png").unwrap();
        g.save("./images/output/decomposition_test/green.png").unwrap();
        b.save("./images/output/decomposition_test/blue.png").unwrap();
    }
}