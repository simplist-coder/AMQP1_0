use crate::utils::vec::VecExt;

pub trait Encode {
    fn encode(self) -> Encoded;
}

pub enum Encoded {
    Empty {
        constructor: u8,
    },
    Fixed {
        constructor: u8,
        data: Vec<u8>,
    },
    Variable {
        constructor: u8,
        data: Vec<u8>,
    },
    Compound {
        constructor: u8,
        count: usize,
        data: Vec<u8>,
    },
    Array {
        constructor: u8,
        count: usize,
        element_constructor: u8,
        data: Vec<u8>,
    },
    Composite {
        constructor: u8,
        descriptor: Vec<u8>,
        data: Vec<u8>,
    },
}

impl Encoded {
    pub fn new_empty(constructor: u8) -> Self {
        Encoded::Empty { constructor }
    }

    pub fn new_fixed(constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Fixed { constructor, data }
    }

    pub fn new_variable(constructor: u8, data: Vec<u8>) -> Self {
        Encoded::Variable { constructor, data }
    }

    pub fn new_compound(constructor: u8, count: usize, data: Vec<u8>) -> Self {
        Encoded::Compound {
            constructor,
            count,
            data,
        }
    }

    pub fn new_array(
        constructor: u8,
        count: usize,
        element_constructor: u8,
        data: Vec<u8>,
    ) -> Self {
        Encoded::Array {
            constructor,
            count,
            element_constructor,
            data,
        }
    }

    pub fn new_composite(constructor: u8, descriptor: Vec<u8>, data: Vec<u8>) -> Self {
        Encoded::Composite {
            constructor,
            descriptor,
            data,
        }
    }

    pub fn constructor(&self) -> u8 {
        match self {
            Self::Empty { constructor } => constructor.to_owned(),
            Self::Fixed { constructor, .. } => constructor.to_owned(),
            Self::Variable { constructor, .. } => constructor.to_owned(),
            Self::Compound { constructor, .. } => constructor.to_owned(),
            Self::Array { constructor, .. } => constructor.to_owned(),
            Self::Composite { constructor, .. } => constructor.to_owned(),
        }
    }

    pub fn data_len(&self) -> usize {
        match self {
            Self::Empty { .. } => 0,
            Self::Fixed { data, .. } => data.len(),
            Self::Variable { data, .. } => data.len(),
            Self::Compound { data, .. } => data.len(),
            Self::Array { data, .. } => data.len(),
            Self::Composite { data, .. } => data.len(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.serialize()
    }
}

fn encode_empty(constructor: Option<u8>) -> Vec<u8> {
    encode_constructor(constructor)
}

fn encode_fixed(constructor: Option<u8>, mut data: Vec<u8>) -> Vec<u8> {
    data.prepend(&mut encode_constructor(constructor));
    data
}

fn encode_variable(constructor: Option<u8>, mut data: Vec<u8>) -> Vec<u8> {
    data.prepend(&mut encode_size(data.len()));
    data.prepend(&mut encode_constructor(constructor));
    data
}

fn encode_compound(constructor: Option<u8>, count: usize, mut data: Vec<u8>) -> Vec<u8> {
    let size = data.len();
    data.prepend(&mut encode_count(count));
    data.prepend(&mut encode_size(size));
    data.prepend(&mut encode_constructor(constructor));
    data
}

fn encode_array(
    constructor: Option<u8>,
    count: usize,
    element_constructor: u8,
    mut data: Vec<u8>,
) -> Vec<u8> {
    let size = data.len();
    data.prepend(&mut vec![element_constructor]);
    data.prepend(&mut encode_count(count));
    data.prepend(&mut encode_size(size));
    data.prepend(&mut encode_constructor(constructor));
    data
}

fn encode_composite(
    constructor: Option<u8>,
    mut descriptor: Vec<u8>,
    mut data: Vec<u8>,
) -> Vec<u8> {
    data.prepend(&mut descriptor);
    data.prepend(&mut encode_constructor(constructor));
    data
}

fn encode_size(len: usize) -> Vec<u8> {
    match len {
        0..=255 => (len as u8).to_be_bytes().to_vec(),
        _ => (len as u32).to_be_bytes().to_vec(),
    }
}

fn encode_count(count: usize) -> Vec<u8> {
    match count {
        0..=255 => (count as u8).to_be_bytes().to_vec(),
        _ => (count as u32).to_be_bytes().to_vec(),
    }
}

fn encode_constructor(constructor: Option<u8>) -> Vec<u8> {
    match constructor {
        None => vec![],
        Some(con) => vec![con],
    }
}

impl Encoded {
    pub(crate) fn serialize_without_constructors(self) -> Vec<u8> {
        match self {
            Encoded::Empty { .. } => encode_empty(None),
            Encoded::Fixed { data, .. } => encode_fixed(None, data),
            Encoded::Variable { data, .. } => encode_variable(None, data),
            Encoded::Compound { count, data, .. } => encode_compound(None, count, data),
            Encoded::Array {
                count,
                element_constructor,
                data,
                ..
            } => encode_array(None, count, element_constructor, data),
            Encoded::Composite { .. } => {
                panic!("Composite values must not be encoded without constructor")
            }
        }
    }

    fn serialize(self) -> Vec<u8> {
        match self {
            Encoded::Empty { constructor } => encode_empty(Some(constructor)),
            Encoded::Fixed { constructor, data } => encode_fixed(Some(constructor), data),
            Encoded::Variable { constructor, data } => encode_variable(Some(constructor), data),
            Encoded::Compound {
                constructor,
                count,
                data,
            } => encode_compound(Some(constructor), count, data),
            Encoded::Array {
                constructor,
                count,
                element_constructor,
                data,
            } => encode_array(Some(constructor), count, element_constructor, data),
            Encoded::Composite {
                constructor,
                descriptor,
                data,
            } => encode_composite(Some(constructor), descriptor, data),
        }
    }
}

impl From<Encoded> for Vec<u8> {
    fn from(value: Encoded) -> Self {
        value.into_bytes()
    }
}

impl From<u8> for Encoded {
    fn from(value: u8) -> Self {
        Encoded::Empty { constructor: value }
    }
}
