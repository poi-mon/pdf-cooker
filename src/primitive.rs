use crate::object::*;

#[derive(Debug)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Name(String),
    Number(u64),
    Map(Vec<Pair>),
    Ref(u32),
    Parent,
}

macro_rules! indent {
    ($num:expr) => {
        if ($num >= 0) {
            "  ".repeat($num as usize)
        } else {
            "".to_string()
        }
    };
}

impl Primitive {
    pub fn encode(&self, indent: u32, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Primitive::Array(array) => {
                writer.write_str("[")?;
                for elm in array.iter() {
                    elm.encode(indent, writer)?;
                    writer.write_str(" ")?;
                }
                writer.write_str("]")?;
            },
            Primitive::Map(pairs) => {
                writer.write_str("<<\n")?;
                for pair in pairs.iter() {
                    pair.encode(indent + 1, writer)?;
                }
                writer.write_fmt(format_args!("{}>>", indent!(indent as i32 - 1)))?;
            },
            Primitive::Name(name) => writer.write_fmt(format_args!("/{}", name))?,
            Primitive::Number(num) => writer.write_fmt(format_args!("{}", num))?,
            Primitive::Ref(uid) => writer.write_fmt(format_args!("{} 0 R", uid))?,
            Primitive::Parent => panic!("unresolved parent reference"),
        };

        Ok(())
    }

    pub fn is_type(&self, ty: &str) -> bool {
        match self {
            Primitive::Array(array) => array.iter().any(|elm| elm.is_type(ty)),
            Primitive::Map(pairs) => pairs.iter().any(|pair| {
                if pair.key == "Type" {
                    if let Primitive::Name(ref value) = *pair.value {
                        return value == ty;
                    }
                } 
                false
            }),
            _ => false
        }
    }
}

impl From<String> for Primitive {
    fn from(target: String) -> Primitive {
        Primitive::Name(target)
    }
}

impl From<&str> for Primitive {
    fn from(target: &str) -> Primitive {
        Primitive::Name(target.to_string())
    }
}

impl From<u64> for Primitive {
    fn from(target: u64) -> Primitive {
        Primitive::Number(target)
    }
}

impl From<i32> for Primitive {
    fn from(target: i32) -> Primitive {
        Primitive::Number(target as u64)
    }
}

impl From<usize> for Primitive {
    fn from(target: usize) -> Primitive {
        Primitive::Number(target as u64)
    }
}

impl From<Primitive> for Vec<Primitive> {
    fn from(target: Primitive) -> Vec<Primitive> {
        vec![target]
    }
}

impl<T> From<Vec<T>> for Primitive where T: Into<Primitive> {
    fn from(target: Vec<T>) -> Primitive {
        Primitive::Array(target.into_iter().map(Into::into).collect())
    }
}

#[derive(Debug)]
pub struct Pair {
    pub key: String, 
    pub value: Box<Primitive>,
}

impl Pair {
    pub fn encode(&self, indent: u32, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        writer.write_fmt(format_args!("{}/{} ", indent!(indent), self.key))?;
        self.value.encode(indent + 1, writer)?;
        writer.write_str("\n")?;
        Ok(())
    }
}

#[macro_export]
macro_rules! array {
    () => {
        Primitive::Array(vec![])
    };
    ($($elm:expr),*) => {
        Primitive::Array(vec![
            $(
                $elm.into()
            ),*
        ])
    };
    (@fromvec ($elm:expr)) => {
        Primitive::Array(elm)
    };
}

#[macro_export]
macro_rules! map {
    () => {
        Primitive::Map(vec![])
    };
    ($vec:expr) => {
        Primitive::Map($vec.into_iter().map(Into::into).collect::<Vec<Pair>>())
    };
    ($($key:expr => $value:expr),*) => {
        Primitive::Map(vec![
            $(
                Pair {
                    key: $key.into(),
                    value: Box::new($value.into())
                }
            ),*
        ])
    };
}

#[macro_export]
macro_rules! pair {
    ($key:expr => $value:expr) => {
        Pair { key: $key.into(), value: Box::new($value.into()) }
    };
}