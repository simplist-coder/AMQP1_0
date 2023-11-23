pub trait Encode {
    fn encode(&self) -> Encoded;
}

pub enum Encoded {
    Empty(u8),
    // Constructor
    Fixed(u8, Vec<u8>),
    // Constructor, Data
    Variable(u8, Vec<u8>),
    // Constructor, Data, size is computed from data
    Compound(u8, u32, Vec<u8>),
    // Constructor, count, data
    Array(u8, u32, u8, Vec<u8>), // Constructor, count, element constructor, data
}

impl Encoded {
    pub fn new_empty(constructor: u8) -> Self {
        Encoded::Empty(constructor)
    }

    pub fn new_fixed(constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Fixed(constructor, data)
    }

    pub fn new_variable(constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Variable(constructor, data)
    }

    pub fn new_compound(constructor: u8, count: u32, data: Vec<u8>) -> Self {
        Encoded::Compound(constructor, count, data)
    }

    pub fn new_array(constructor: u8, count: u32, element_constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Array(constructor, count, element_constructor, data)
    }

    pub fn constructor(&self) -> u8 {
        match self {
            Self::Empty(c) => c.to_owned(),
            Self::Fixed(c, _) => c.to_owned(),
            Self::Variable(c, _) => c.to_owned(),
            Self::Compound(c, _, _) => c.to_owned(),
            Self::Array(c, _, _, _) => c.to_owned(),
        }
    }

    pub fn data_len(&self) -> usize {
        match self {
            Self::Empty(_) => 0,
            Self::Fixed(_, data) => data.len(),
            Self::Variable(_, data) => data.len(),
            Self::Compound(_, _, data) => data.len(),
            Self::Array(_, _, _, data) => data.len(),
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.into()
    }
}

impl From<Encoded> for Vec<u8> {
    fn from(value: Encoded) -> Self {
        let mut res = Vec::new();
        match value {
            Encoded::Empty(c) => res.push(c),
            Encoded::Fixed(c, mut data) => {
                res.push(c);
                res.append(&mut data);
            }
            Encoded::Variable(c, mut data) => {
                res.push(c);
                let mut size: Vec<u8> = match c {
                    x if x >= 0xA0 && x <= 0xAF => vec![data.len() as u8],
                    _ => (data.len() as u32).to_be_bytes().to_vec(),
                };
                res.append(&mut size);
                res.append(&mut data);
            }
            Encoded::Compound(c, count, mut data) => {
                res.push(c);
                res.append(&mut count.to_be_bytes().to_vec());
                res.append(&mut data);
            }
            Encoded::Array(_, _, _, _) => {
                todo!("Implement Array encode to bytes")
            }
        }
        res
    }
}

impl From<u8> for Encoded {
    fn from(value: u8) -> Self {
        Encoded::Empty(value)
    }
}
