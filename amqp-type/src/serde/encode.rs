pub trait Encode {
    fn encode(&self) -> Encoded;
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

    pub fn constructor(&self) -> u8 {
        match self {
            Encoded::Empty { constructor } => constructor.to_owned(),
            Encoded::Fixed { constructor, .. } => constructor.to_owned(),
            Encoded::Variable { constructor, .. } => constructor.to_owned(),
            Encoded::Compound { constructor, .. } => constructor.to_owned(),
            Encoded::Array { constructor, .. } => constructor.to_owned(),
        }
    }

    pub fn data_len(&self) -> usize {
        match self {
            Self::Empty { .. } => 0,
            Self::Fixed { data, .. } => data.len(),
            Self::Variable { data, .. } => data.len(),
            Self::Compound { data, .. } => data.len(),
            Self::Array { data, .. } => data.len(),
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.into()
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
    data.prepend(&mut encode_count(count));
    data.prepend(&mut encode_constructor(constructor));
    data
}

fn encode_array(
    constructor: Option<u8>,
    count: usize,
    element_constructor: u8,
    mut data: Vec<u8>,
) -> Vec<u8> {
    data.prepend(&mut vec![element_constructor]);
    data.prepend(&mut encode_count(count));
    data.prepend(&mut encode_size(count));
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
        }
    }

    pub(crate) fn serialize(self) -> Vec<u8> {
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
        }
    }
}

impl From<Encoded> for Vec<u8> {
    fn from(value: Encoded) -> Self {
        value.serialize()
    }
}

/// From: https://www.reddit.com/r/rust/comments/kul4qz/vec_prepend_insert_from_slice/
trait VecExt<T>: AsMut<Vec<T>> {
    fn prepend(&mut self, other: &mut Vec<T>) {
        self.as_mut().splice(..0, other.drain(..));
    }
}

impl<T> VecExt<T> for Vec<T> {}

impl From<u8> for Encoded {
    fn from(value: u8) -> Self {
        Encoded::Empty { constructor: value }
    }
}
