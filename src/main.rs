use image::GenericImageView;
use easy_paths::get_absolute_path;


fn main() {
    let path = get_absolute_path(&"./images/samples/tmp.jpeg");
    println!("{:?}", path);
    if easy_paths::is_existing_path(&path) {
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let mut img = image::open(path).unwrap();
        let c = img.as_mut_rgb8();
        let c = c.unwrap();
        for ch in c.iter_mut() {
            let f = 128;
            *ch = if *ch >= f { *ch - f } else { *ch + f };
        }

        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        img.save("images/output/test.png").unwrap();
        println!("a");
    }

}