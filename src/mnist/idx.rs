use std::path;
use std::fs;
use std::io;
use std::io::Read;
use std::marker;
use std::default::Default;

use byteorder::{BigEndian, ReadBytesExt, ByteOrder};

use super::error::{MnistError, Result};

pub trait ElementScalar: Sized + Copy + Default {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()>;
    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(read: &mut R) -> Result<Self>;
}

impl ElementScalar for u8 {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()> {
        if ty == ElementType::U8 { Ok(()) } 
        else { Err(MnistError::InvalidElementType) }
    }

    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(reader: &mut R) -> Result<u8> {
        Ok(reader.read_u8()?)
    }
}

impl ElementScalar for i8 {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()> {
        if ty == ElementType::I8 { Ok(()) } 
        else { Err(MnistError::InvalidElementType) }
    }
    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(reader: &mut R) -> Result<i8> {
        Ok(reader.read_i8()?)
    }
}

impl ElementScalar for i16 {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()> {
        if ty == ElementType::I16 { Ok(()) } 
        else { Err(MnistError::InvalidElementType) }
    }
    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(reader: &mut R) -> Result<i16> {
        Ok(reader.read_i16::<T>()?)
    }
}
impl ElementScalar for i32 {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()> {
        if ty == ElementType::I32 { Ok(()) } 
        else { Err(MnistError::InvalidElementType) }
    }
    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(reader: &mut R) -> Result<i32> {
        Ok(reader.read_i32::<T>()?)
    }
}
impl ElementScalar for f32 {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()> {
        if ty == ElementType::F32 { Ok(()) } 
        else { Err(MnistError::InvalidElementType) }
    }
    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(reader: &mut R) -> Result<f32> {
        Ok(reader.read_f32::<T>()?)
    }
}
impl ElementScalar for f64 {
    fn is_elem_type_compatible(ty: ElementType) -> Result<()> {
        if ty == ElementType::F64 { Ok(()) } 
        else { Err(MnistError::InvalidElementType) }
    }
    fn read_element<T: ByteOrder, R: Read + ReadBytesExt>(reader: &mut R) -> Result<f64> {
        Ok(reader.read_f64::<T>()?)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ElementType {
    U8 = 0x08,
    I8 = 0x09,
    I16 = 0x0b,
    I32 = 0x0c,
    F32 = 0x0d,
    F64 = 0x0e,
}

impl ElementType {
    pub fn from_value(val: u8) -> Result<ElementType> {
        match val {
            0x08 => Ok(ElementType::U8),
            0x09 => Ok(ElementType::I8),
            0x0b => Ok(ElementType::I16),
            0x0c => Ok(ElementType::I32),
            0x0d => Ok(ElementType::F32),
            0x0e => Ok(ElementType::F64),
            _ => Err(MnistError::InvalidElementType),
        }
    }

    pub fn size_in_bytes(&self) -> u32 {
        match *self {
            ElementType::U8 => 1,
            ElementType::I8 => 1,
            ElementType::I16 => 2,
            ElementType::I32 => 4,
            ElementType::F32 => 4,
            ElementType::F64 => 8,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IdxHeader {
    pub elem_type: ElementType,
    pub dimension_sizes: Vec<u32>,
}

#[derive(Debug)]
pub struct IdxReader<R: Read + ReadBytesExt> {
    reader: R,
    header: IdxHeader,
}

#[derive(Clone, Debug)]
pub struct Item<T> 
    where T: ElementScalar
{
    elems: Vec<T>,
    dimension_sizes: Vec<u32>,
}

#[derive(Debug)]
pub struct Elements<T, R> 
    where T: ElementScalar,
          R: Read + ReadBytesExt,
{
    reader: IdxReader<R>,
    elem_type: marker::PhantomData<T>,
}

#[derive(Debug)]
pub struct Items<T, R> 
    where T: ElementScalar,
          R: Read + ReadBytesExt,
{
    reader: IdxReader<R>,
    elem_type: marker::PhantomData<T>,
}

impl IdxReader<io::BufReader<fs::File>> {
    pub fn from_file(file_name: &path::Path) -> Result<IdxReader<io::BufReader<fs::File>>> {
        const BUF_READER_CAPACITY: usize = (1 << 20);

        let f = fs::File::open(file_name)?;
        let mut reader = io::BufReader::with_capacity(BUF_READER_CAPACITY, f);
        
        let header = IdxReader::read_header(&mut reader)?;

        Ok(
            IdxReader {
                reader: reader,
                header: header,
            }
        )
    }

}

impl<R: Read + ReadBytesExt> IdxReader<R> {
    pub fn new(mut reader: R) -> Result<IdxReader<R>> {
        let header = IdxReader::read_header(&mut reader)?;

        Ok(
            IdxReader {
                reader: reader,
                header: header,
            }
        )
    }

    pub fn dimensions(&self) -> &[u32] {
        &self.header.dimension_sizes
    }

    pub fn num_elems(&self) -> usize {
        let mut total = 1;
        for size in &self.header.dimension_sizes {
            total *= *size as usize;
        }
        total
    }
    pub fn element_type(&self) -> ElementType {
        self.header.elem_type.clone()
    }
    pub fn reader(&mut self) -> &mut R {
        &mut self.reader
    }
    pub fn item_size(&self) -> usize {
        let mut total = 1;
        for size in &self.header.dimension_sizes[1..] {
            total *= *size as usize
        }
        total
    }

    pub fn read_bytes_to_end(&mut self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.reader.read_to_end(&mut bytes)?;

        Ok(bytes)
    }

    pub fn read_elements<T>(&mut self, buf: &mut [T]) -> Result<()> 
        where T: ElementScalar 
    {
        self.assert_elem_type_compatible::<T>();
        for i in buf.iter_mut() {
            let elem: T = self.read_element()?;
            *i = elem;
        }
        Ok(())
    }
    pub fn elements<T>(self) -> Elements<T, R>
        where T: ElementScalar 
    {
        self.assert_elem_type_compatible::<T>();
        Elements {
            reader: self,
            elem_type: marker::PhantomData,
        }
    }

    pub fn read_item<T>(&mut self) -> Result<Item<T>>
        where T: ElementScalar 
    {
        let item_sizes = self.item_size();
        let mut elems: Vec<T> = vec![T::default(); item_sizes];
        self.read_elements(&mut elems[..])?;

        Ok(
            Item {
                elems: elems,
                dimension_sizes: self.get_item_geometry(),
            }
        )
    }
    pub fn read_items_to_end<T>(&mut self, buf: &mut Vec<Item<T>>) -> Result<()>
        where T: ElementScalar 
    {
        loop {
            let item = self.read_item::<T>();
            match item {
                Ok(x) => {
                    buf.push(x);
                },
                Err(MnistError::Io(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    break;
                },
                Err(e) => {
                    return Err(e);
                },
            }
        }
        return Ok(());
    }
    pub fn items<T>(self) -> Items<T, R> 
        where T: ElementScalar 
    {
        self.assert_elem_type_compatible::<T>();
        assert!(self.dimensions().len() > 1);
        Items {
            reader: self,
            elem_type: marker::PhantomData,
        }
    }

    fn get_item_geometry(&self) -> Vec<u32> {
        match self.dimensions().len() {
            0 => Vec::new(),
            1 => self.header.dimension_sizes.clone(),
            _ => self.dimensions()[1..].to_vec(),
        }
    }
    fn assert_elem_type_compatible<T>(&self) 
        where T: ElementScalar
    {
        T::is_elem_type_compatible(self.element_type())
            .expect("Requested element type does not match file contents");
    }

    fn read_element<T: ElementScalar>(&mut self) -> Result<T> {
        let element = T::read_element::<BigEndian, _>(&mut self.reader);

        element
    }
    fn read_header(reader: &mut R) -> Result<IdxHeader> {
        let zero = reader.read_u16::<BigEndian>()?;

        if zero != 0x0000 {
            return Err(MnistError::InvalidFormat)
        }

        let elem_type = reader.read_u8()?;
        let type_enum = ElementType::from_value(elem_type)?;

        let num_dims = reader.read_u8()?;
        let mut dim_sizes = vec![0; num_dims as usize];

        for i in 0..dim_sizes.len() {
            dim_sizes[i] = reader.read_u32::<BigEndian>()?;
        }

        Ok(
            IdxHeader {
                elem_type: type_enum,
                dimension_sizes: dim_sizes,
            }
        )
    }
}

impl<T, R> Iterator for Elements<T, R> 
    where R: Read + ReadBytesExt,
          T: ElementScalar
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        let elem = self.reader.read_element::<T>();

        match elem {
            Ok(x) => Some(Ok(x)),
            Err(MnistError::Io(e)) => 
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    None
                } else {
                    Some(Err(e.into())) 
                },
            Err(e) => Some(Err(e)),

        }
    }
}

impl<T> Item<T>
    where T: ElementScalar 
{
    pub fn data(&self) -> &[T] {
        &self.elems[..]
    }
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.elems[..]
    }
    pub fn dimensions(&self) -> &[u32] {
        &self.dimension_sizes[..]
    }
    pub fn width(&self) -> Option<u32> {
        if self.dimension_sizes.len() > 0 {
            Some(self.dimension_sizes[self.dimension_sizes.len()-1])
        } else {
            None
        }
    }
    pub fn height(&self) -> Option<u32> {
        if self.dimension_sizes.len() > 1 {
            Some(self.dimension_sizes[self.dimension_sizes.len()-2])
        } else {
            None
        }
    }
    pub fn total_elements(&self) -> u32 {
        let mut total = 1;
        for i in self.dimension_sizes.iter() {
            total *= *i;
        }
        total
    }
}

impl<T, R> Iterator for Items<T, R> 
    where T: ElementScalar,
          R: Read + ReadBytesExt,
{
    type Item = Result<Item<T>>;

    fn next(&mut self) -> Option<Result<Item<T>>> {
        let item = self.reader.read_item::<T>();

        match item {
            Ok(i) => Some(Ok(i)),
            Err(MnistError::Io(e)) => 
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    None
                } else {
                    Some(Err(e.into())) 
                },
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
