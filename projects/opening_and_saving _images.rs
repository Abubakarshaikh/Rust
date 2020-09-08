// >>>>>>>>>>>>>>>>>>>>>>>>>>>>> Opening and Saving Images <<<<<<<<<<<<<<<<<<<<<<<<<<<<<
extern crate  image;
use image::GenericImageView;

fn main(){
    // Use the opening funtion to load an image from path
    // open return a 'DynamicImage' on success
    let image = image::open("./test.png").unwrap();

    // The Dimention function return image width and height
    println!("Dimention {:?}",image.dimensions());

    // The color method return's the image's colorType
    println!("Image color {:?}",image.color());

    // Write the contents of this image to the writer in PNG formate
    image.save("test1.png").unwrap();
}
