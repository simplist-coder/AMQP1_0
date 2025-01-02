use crate::primitive::composite::{Composite, Descriptor};
use crate::primitive::compound::list::List;
use crate::primitive::Primitive;

pub struct CompositeBuilder {
    descriptor: Descriptor,
    list: Vec<Primitive>,
}

impl CompositeBuilder {
    pub fn new(descriptor: Descriptor) -> Self {
        CompositeBuilder {
            descriptor,
            list: Vec::new(),
        }
    }

    pub fn push(mut self, primitive: Primitive) -> Self {
        self.list.push(primitive);
        self
    }

    pub fn build(self) -> Composite {
        Composite::new(self.descriptor, List::from(self.list))
    }
}

#[cfg(test)]
mod tests {
    use crate::error::amqp_error::AmqpError;
    use crate::error::AppError;
    use crate::primitive::composite::builder::CompositeBuilder;
    use crate::primitive::composite::{CompositeType, Descriptor};
    use crate::primitive::variable_width::symbol::Symbol;
    use crate::primitive::Primitive;
    use crate::serde::encode::Encode;
    use indexmap::IndexMap;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct TestStruct {
        id: u64,
        name: String,
        properties: IndexMap<String, String>,
    }

    impl TryFrom<Primitive> for TestStruct {
        type Error = AppError;

        fn try_from(value: Primitive) -> Result<Self, Self::Error> {
            if let Primitive::Composite(mut composite) = value {
                Ok(TestStruct {
                    id: composite.pop_front().try_into()?,
                    name: composite.pop_front().try_into()?,
                    properties: composite.pop_front().try_into()?,
                })
            } else {
                Err(AmqpError::DecodeError)?
            }
        }
    }

    impl CompositeType for TestStruct {
        fn descriptor(&self) -> Descriptor {
            Symbol::with_ascii("TestStruct").into()
        }
    }
    impl From<TestStruct> for Primitive {
        fn from(value: TestStruct) -> Self {
            CompositeBuilder::new(value.descriptor())
                .push(value.id.into())
                .push(value.name.into())
                .push(value.properties.into())
                .build()
                .into()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct NestedStruct {
        id: u64,
        test_struct: TestStruct,
        test_enum: TestEnum,
    }

    impl TryFrom<Primitive> for NestedStruct {
        type Error = AppError;

        fn try_from(value: Primitive) -> Result<Self, Self::Error> {
            if let Primitive::Composite(mut composite) = value {
                Ok(NestedStruct {
                    id: composite.pop_front().try_into()?,
                    test_struct: composite.pop_front().try_into()?,
                    test_enum: composite.pop_front().try_into()?,
                })
            } else {
                Err(AmqpError::DecodeError)?
            }
        }
    }

    impl CompositeType for NestedStruct {
        fn descriptor(&self) -> Descriptor {
            Symbol::with_ascii("NestedStruct").into()
        }
    }

    impl From<NestedStruct> for Primitive {
        fn from(value: NestedStruct) -> Self {
            CompositeBuilder::new(value.descriptor())
                .push(value.id.into())
                .push(value.test_struct.into())
                .push(value.test_enum.into())
                .build()
                .into()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum TestEnum {
        Empty,
        Value(String),
        Many(String, i64, u32),
        Nested(TestStruct),
    }

    impl ::core::convert::TryFrom<Primitive> for TestEnum {
        type Error = AppError;

        fn try_from(value: Primitive) -> Result<Self, Self::Error> {
            if let Primitive::Composite(mut composite) = value {
                match composite.descriptor() {
                    Descriptor::Symbol(s) => {
                        match s.inner() {
                            "TestEnum::Empty" => Ok(TestEnum::Empty),
                            "TestEnum::Value" => Ok(TestEnum::Value(composite.pop_front().try_into()?)),
                            "TestEnum::Many" => Ok(TestEnum::Many(
                                composite.pop_front().try_into()?,
                                composite.pop_front().try_into()?,
                                composite.pop_front().try_into()?,
                            )),
                            "TestEnum::Nested" => Ok(TestEnum::Nested(
                                composite.pop_front().try_into()?,
                            )),
                            _ => Err(AmqpError::DecodeError)?
                        }
                    }
                    _ => {
                        Err(AmqpError::DecodeError)?
                    }
                }
            } else {
                Err(AmqpError::DecodeError)?
            }
        }
    }

    impl CompositeType for TestEnum {
        fn descriptor(&self) -> Descriptor {
            match self {
                TestEnum::Empty => Symbol::with_ascii("TestEnum::Empty").into(),
                TestEnum::Value(_) => Symbol::with_ascii("TestEnum::Value").into(),
                TestEnum::Many(_, _, _) => Symbol::with_ascii("TestEnum::Many").into(),
                TestEnum::Nested(_) => Symbol::with_ascii("TestEnum::Nested").into(),
            }
        }
    }

    impl From<TestEnum> for Primitive {
        fn from(value: TestEnum) -> Self {
            let mut builder = CompositeBuilder::new(value.descriptor());
            match value {
                TestEnum::Empty => {}
                TestEnum::Value(x) => {
                    builder = builder.push(x.into());
                }
                TestEnum::Many(a, b, c) => {
                    builder = builder
                        .push(a.into())
                        .push(b.into())
                        .push(c.into());
                }
                TestEnum::Nested(a) => {
                    builder = builder.push(a.into());
                }
            }
            builder.build().into()
        }
    }

    #[test]
    fn test_encode_decode_round_trip_for_arbitrary_struct() {
        let initial = TestStruct {
            id: 5,
            name: "hello".to_string(),
            properties: Default::default(),
        };
        let encoded = Primitive::from(initial.clone()).encode().into_bytes();
        let decoded =
            TestStruct::try_from(Primitive::try_decode(&mut encoded.into_iter()).unwrap()).unwrap();
        assert_eq!(decoded, initial);
    }

    #[test]
    fn test_encode_decode_round_trip_for_arbitrary_nested_struct() {
        let initial = NestedStruct {
            id: 5,
            test_struct: TestStruct {
                id: 5,
                name: "hello".to_string(),
                properties: Default::default(),
            },
            test_enum: TestEnum::Value("world".to_string()),
        };
        let encoded = Primitive::from(initial.clone()).encode().into_bytes();
        let decoded =
            NestedStruct::try_from(Primitive::try_decode(&mut encoded.into_iter()).unwrap())
                .unwrap();
        assert_eq!(decoded, initial);
    }

    #[test]
    fn test_encode_decode_round_trip_for_arbitrary_nested_struct_in_enum() {
        let initial = NestedStruct {
            id: 5,
            test_struct: TestStruct {
                id: 5,
                name: "hello".to_string(),
                properties: Default::default(),
            },
            test_enum: TestEnum::Nested(TestStruct {
                id: 6,
                name: "hahahaha".to_string(),
                properties: Default::default(),
            }),
        };
        let encoded = Primitive::from(initial.clone()).encode().into_bytes();
        let decoded =
            NestedStruct::try_from(Primitive::try_decode(&mut encoded.into_iter()).unwrap())
                .unwrap();
        assert_eq!(decoded, initial);
    }
}
