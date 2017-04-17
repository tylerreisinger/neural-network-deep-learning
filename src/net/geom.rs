use std::fmt;

#[derive(Clone, Debug)]
pub struct Geometry {
    layers_geometry: Vec<usize>,
    num_neurons: usize,
}

impl Geometry {
    pub fn new(layers: Vec<usize>) -> Geometry {
        let num_neurons = 
            layers.iter().fold(0, |sum, &x| sum + x);

        Geometry {
            layers_geometry: layers,
            num_neurons: num_neurons,
        }
    }
}

impl fmt::Display for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, size) in self.layers_geometry.iter().enumerate() {
            write!(f, "{}", size)?;
            if i < self.layers_geometry.len()-1 {
                write!(f, " -> ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}
