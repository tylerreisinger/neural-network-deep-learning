extern crate rand;
extern crate byteorder;

pub mod mnist;

use std::path::Path;
use std::io;
use mnist::image;

fn main() {
    let mut reader = image::ImageReader::new(&Path::new("data/train-images-idx3-ubyte"))
                     .unwrap();

    for (i, img) in reader.images().enumerate() {
        println!("{}", i); 
    }
}
