use std::path;
use std::fs;
use std::io;
use std::io::{Read};

use byteorder::{BigEndian, ReadBytesExt};

use super::error as mnist_error;

#[derive(Clone, Debug)]
pub struct Header {
    pub num_images: u32,
    pub image_width: u32,
    pub image_height: u32,
}

pub type Image = Vec<u8>;

type Reader = io::BufReader<fs::File>;
#[derive(Debug)]
pub struct ImageReader {
    file: Reader,
    header: Header,
    image_index: u32,
}

#[derive(Debug)]
pub struct ImageIterator<'a> {
    reader: &'a mut ImageReader,
}

impl ImageReader {
    pub fn new(file_name: &path::Path) -> mnist_error::Result<ImageReader> {
        //1MiB 
        const BUF_READER_CAPACITY: usize = (2usize << 20);

        let f = fs::File::open(file_name)?;
        let mut reader = io::BufReader::with_capacity(BUF_READER_CAPACITY, f);

        ImageReader::check_magic_number(&mut reader)?;
        let header = ImageReader::read_header(&mut reader)?;

        let reader = ImageReader {
            file: reader,
            header: header,
            image_index: 0,
        };


        Ok(reader)
    }

    pub fn num_images(&self) -> u32 {
        self.header.num_images
    }
    pub fn image_width(&self) -> u32 {
        self.header.image_width
    }
    pub fn image_height(&self) -> u32 {
        self.header.image_height
    }
    pub fn num_pixels(&self) -> usize {
        (self.image_width() * self.image_height()) as usize
    }

    pub fn read_next_image(&mut self) -> mnist_error::Result<Option<Image>> {
        let mut pixel_buffer = vec![0; self.num_pixels()];

        let len = self.file.read_exact(pixel_buffer.as_mut_slice())?;

        Ok(Some(pixel_buffer))
    }

    fn check_magic_number(f: &mut Reader) -> mnist_error::Result<()> {
        let magic = f.read_u32::<BigEndian>()?;
        if magic != 0x00000803 {
            Err(mnist_error::MnistError::InvalidFormat)
        } else {
            Ok(())
        }
    }

    fn read_header(f: &mut Reader) -> mnist_error::Result<Header> {
        let num_images = f.read_u32::<BigEndian>()?;
        let width = f.read_u32::<BigEndian>()?;
        let height = f.read_u32::<BigEndian>()?;

        Ok(
            Header {
                num_images: num_images,
                image_width: width,
                image_height: height,
            }
        )
    }
}

impl<'a> ImageReader {
    pub fn images(&'a mut self) -> ImageIterator<'a> {
        ImageIterator {
            reader: self
        }
    }
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = Image;
    fn next(&mut self) -> Option<Image> {
        let image = self.reader.read_next_image();
        match image {
            Ok(img) => img,
            Err(_) => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.reader.num_images() as usize - self.reader.image_index as usize;
        (remaining, Some(remaining))
    }
}
