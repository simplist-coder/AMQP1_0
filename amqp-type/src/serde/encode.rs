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
        count: u32,
        data: Vec<u8>,
    },
    Array {
        constructor: u8,
        count: u32,
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

    pub fn new_compound(constructor: u8, count: u32, data: Vec<u8>) -> Self {
        Encoded::Compound {
            constructor,
            count,
            data,
        }
    }

    pub fn new_array(constructor: u8, count: u32, element_constructor: u8, data: Vec<u8>) -> Self {
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

fn encode_empty(constructor: u8) -> Vec<u8> {
    vec![constructor]
}

fn encode_fixed(constructor: u8, mut data: Vec<u8>) -> Vec<u8> {
    data.prepend(&mut vec![constructor]);
    data
}

fn encode_variable(constructor: u8, mut data: Vec<u8>) -> Vec<u8> {
    let mut size: Vec<u8> = match constructor {
        // all variable elements with size < u8::MAX have a constructor of form 0xAP where P can be any value between 0 and F.
        x if x >= 0xA0 && x <= 0xAF => vec![data.len() as u8],
        _ => (data.len() as u32).to_be_bytes().to_vec(),
    };
    data.prepend(&mut size);
    data.prepend(&mut vec![constructor]);
    data
}

fn encode_compound(constructor: u8, count: u32, mut data: Vec<u8>) -> Vec<u8> {
    data.prepend(&mut count.to_be_bytes().to_vec());
    data.prepend(&mut vec![constructor]);
    data
}

fn encode_array(
    _constructor: u8,
    _count: u32,
    _element_constructor: u8,
    _data: Vec<u8>,
) -> Vec<u8> {
    todo!()
}

impl Encoded {
    pub(crate) fn serialize_without_constructors(self) -> Vec<u8> {
        todo!()
    }

    pub(crate) fn serialize(self) -> Vec<u8> {
        match self {
            Encoded::Empty { constructor } => encode_empty(constructor),
            Encoded::Fixed { constructor, data } => encode_fixed(constructor, data),
            Encoded::Variable { constructor, data } => encode_variable(constructor, data),
            Encoded::Compound {
                constructor,
                count,
                data,
            } => encode_compound(constructor, count, data),
            Encoded::Array {
                constructor,
                count,
                element_constructor,
                data,
            } => encode_array(constructor, count, element_constructor, data),
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
