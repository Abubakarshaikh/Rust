[dependencies]
bardecoder = "0.2.2"
image = "0.22"

use bardecoder;
use bardecoder::prepare::BlockedMean;

use image;

fn main() {
    let img = image::open("<<image location>>").unwrap();

    // Use default decoder builder
    let mut db = bardecoder::default_builder();

    // Use some different arguments in one of the default components
    db.prepare(Box::new(BlockedMean::new(7, 9)));

    // Build the actual decoder
    let decoder = db.build();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }
}
