extern crate rand;
extern crate byteorder;

pub mod mnist;
pub mod net;
pub mod math;

use std::path::Path;
use mnist::idx;

fn main() {
    let labels = 
        idx::IdxReader::from_file(&Path::new("data/train-labels-idx1-ubyte")).unwrap();
    let images = 
        idx::IdxReader::from_file(&Path::new("data/train-images-idx3-ubyte")).unwrap();

}
