use std::os::linux::raw::time_t;
use std::time::Instant;
use image::GenericImageView;
use easy_paths::get_absolute_path;


fn main() {
    let path = get_absolute_path(&"./images/samples/retina.jpg");
    println!("{:?}", path);
    if easy_paths::is_existing_path(&path) {
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let start = Instant::now();
        let mut img = image::open(path).unwrap();
        let opened = img.as_mut_rgb8().unwrap();
        let mut fs = opened.as_mut();
        let mut i: u64 = 0;
        for mut ch in fs.iter_mut() {
            if i%3 == 0 {//red
            }
            if i%3 == 1 {//green
                *ch = 0;
            }
            if i%3 ==2 {//blue
                *ch = 0;
            }
            i += 1;
        }
        println!("{:?}", fs.len());

        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        println!("end: {:?}", start.elapsed().as_millis());
        img.save("images/output/test.png").unwrap();
    }

}