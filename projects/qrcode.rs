// cargo.toml
// [dependencies]
// qrcode = "0.12.0"
// image = "0.23.9"

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< Image generation >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
use qrcode::QrCode;
use image::Luma;

fn image_generation(image: &str){
    // Encode some data into string:
    let code = QrCode::new(image).unwrap();

    //render the &str into an image:
    let image = code.render::<Luma<u8>>().build();

    // save the image:
    image.save("/tmp/qrcode2.png").unwrap();
}

fn main(){
    image_generation("string generation");
}

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<< String generation >>>>>>>>>>>>>>>>>>>>>>>>>>>>>
use qrcode::QrCode;

fn string_generation(string: &str){
    let code = QrCode::new(string).unwrap();
    let string = code.render::<char>()
                 .quiet_zone(false)
                 .module_dimensions(2, 1)
                 .build();
    println!("{}",string);
}
fn main(){
    string_generation("hello");
}

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<< SVG generation >>>>>>>>>>>>>>>>>>>>>>>>>>>>>
use qrcode::{QrCode, Version, EcLevel};
use qrcode::render::svg;
fn main(){
    let code = QrCode::with_version(b"01234567",Version::Micro(2) ,EcLevel::L).unwrap();
    let image = code.render()
                    .min_dimensions(200, 200)
                    .dark_color(svg::Color("#800000"))
                    .light_color(svg::Color("#ffff80"))
                    .build();
    println!("{}",image);
}

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<< Unicode string generation >>>>>>>>>>>>>>>>>>>>>>>>>>>>>
use qrcode::QrCode;
use qrcode::render::unicode;

fn main(){
    let code = QrCode::new("Mow mow").unwrap();
    let image = code.render::<unicode::Dense1x2>()
                    .dark_color(unicode::Dense1x2::Light)
                    .light_color(unicode::Dense1x2::Dark)
                    .build();
    println!("{}",image);
}
