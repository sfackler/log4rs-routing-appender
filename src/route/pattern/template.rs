use serde_value::Value;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fmt::Write;
use log_mdc;

use route::pattern::parser::{Parser, Piece};

pub struct Template {
    value: ValueTemplate,
    keys: HashSet<String>,
}

impl Template {
    pub fn new(pattern: &Value) -> Result<Template, Box<Error>> {
        let value = ValueTemplate::new(pattern)?;
        let mut keys = HashSet::new();
        value.keys(&mut keys);
        Ok(Template {
            value: value,
            keys: keys,
        })
    }

    pub fn key(&self) -> String {
        let mut s = String::new();
        for key in &self.keys {
            log_mdc::get(key, |k| {
                match k {
                    Some(k) => write!(s, "{}{}", k.len(), k).unwrap(),
                    None => s.push('-'),
                }
            });
        }
        s
    }

    pub fn expand(&self) -> Value {
        self.value.expand()
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Chunk {
    Text(String),
    Mdc(String),
}

enum ValueTemplate {
    Map(BTreeMap<ValueTemplate, ValueTemplate>),
    Newtype(Box<ValueTemplate>),
    Option(Option<Box<ValueTemplate>>),
    Seq(Vec<ValueTemplate>),
    String(Vec<Chunk>),
    Bool(bool),
    Bytes(Vec<u8>),
    Char(char),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    Unit,
    UnitStruct(&'static str),
}

impl Eq for ValueTemplate {}

impl PartialEq for ValueTemplate {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&ValueTemplate::Bool(v0), &ValueTemplate::Bool(v1)) if v0 == v1 => true,
            (&ValueTemplate::Usize(v0), &ValueTemplate::Usize(v1)) if v0 == v1 => true,
            (&ValueTemplate::U8(v0), &ValueTemplate::U8(v1)) if v0 == v1 => true,
            (&ValueTemplate::U16(v0), &ValueTemplate::U16(v1)) if v0 == v1 => true,
            (&ValueTemplate::U32(v0), &ValueTemplate::U32(v1)) if v0 == v1 => true,
            (&ValueTemplate::U64(v0), &ValueTemplate::U64(v1)) if v0 == v1 => true,
            (&ValueTemplate::Isize(v0), &ValueTemplate::Isize(v1)) if v0 == v1 => true,
            (&ValueTemplate::I8(v0), &ValueTemplate::I8(v1)) if v0 == v1 => true,
            (&ValueTemplate::I16(v0), &ValueTemplate::I16(v1)) if v0 == v1 => true,
            (&ValueTemplate::I32(v0), &ValueTemplate::I32(v1)) if v0 == v1 => true,
            (&ValueTemplate::I64(v0), &ValueTemplate::I64(v1)) if v0 == v1 => true,
            (&ValueTemplate::F32(v0), &ValueTemplate::F32(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&ValueTemplate::F64(v0), &ValueTemplate::F64(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&ValueTemplate::Char(v0), &ValueTemplate::Char(v1)) if v0 == v1 => true,
            (&ValueTemplate::String(ref v0), &ValueTemplate::String(ref v1)) if v0 == v1 => true,
            (&ValueTemplate::Unit, &ValueTemplate::Unit) => true,
            (&ValueTemplate::UnitStruct(v0), &ValueTemplate::UnitStruct(v1)) if v0 == v1 => true,
            (&ValueTemplate::Option(ref v0), &ValueTemplate::Option(ref v1)) if v0 == v1 => true,
            (&ValueTemplate::Newtype(ref v0), &ValueTemplate::Newtype(ref v1)) if v0 == v1 => true,
            (&ValueTemplate::Seq(ref v0), &ValueTemplate::Seq(ref v1)) if v0 == v1 => true,
            (&ValueTemplate::Map(ref v0), &ValueTemplate::Map(ref v1)) if v0 == v1 => true,
            (&ValueTemplate::Bytes(ref v0), &ValueTemplate::Bytes(ref v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl PartialOrd for ValueTemplate {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for ValueTemplate {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (&ValueTemplate::Bool(v0), &ValueTemplate::Bool(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Usize(v0), &ValueTemplate::Usize(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::U8(v0), &ValueTemplate::U8(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::U16(v0), &ValueTemplate::U16(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::U32(v0), &ValueTemplate::U32(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::U64(v0), &ValueTemplate::U64(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Isize(v0), &ValueTemplate::Isize(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::I8(v0), &ValueTemplate::I8(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::I16(v0), &ValueTemplate::I16(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::I32(v0), &ValueTemplate::I32(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::I64(v0), &ValueTemplate::I64(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::F32(v0), &ValueTemplate::F32(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&ValueTemplate::F64(v0), &ValueTemplate::F64(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&ValueTemplate::Char(v0), &ValueTemplate::Char(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::String(ref v0), &ValueTemplate::String(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Unit, &ValueTemplate::Unit) => Ordering::Equal,
            (&ValueTemplate::UnitStruct(v0), &ValueTemplate::UnitStruct(v1)) => v0.cmp(v1),
            (&ValueTemplate::Option(ref v0), &ValueTemplate::Option(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Newtype(ref v0), &ValueTemplate::Newtype(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Seq(ref v0), &ValueTemplate::Seq(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Map(ref v0), &ValueTemplate::Map(ref v1)) => v0.cmp(v1),
            (&ValueTemplate::Bytes(ref v0), &ValueTemplate::Bytes(ref v1)) => v0.cmp(v1),
            (ref v0, ref v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl ValueTemplate {
    fn new(value: &Value) -> Result<ValueTemplate, Box<Error>> {
        let value = match *value {
            Value::Map(ref m) => {
                let mut m2 = BTreeMap::new();
                for (k, v) in m {
                    m2.insert(ValueTemplate::new(k)?, ValueTemplate::new(v)?);
                }
                ValueTemplate::Map(m2)
            }
            Value::Newtype(ref v) => ValueTemplate::Newtype(Box::new(ValueTemplate::new(v)?)),
            Value::Option(ref v) => {
                let v = match *v {
                    Some(ref v) => Some(Box::new(ValueTemplate::new(v)?)),
                    None => None,
                };
                ValueTemplate::Option(v)
            }
            Value::Seq(ref vs) => {
                let mut vs2 = vec![];
                for v in vs {
                    vs2.push(ValueTemplate::new(v)?);
                }
                ValueTemplate::Seq(vs2)
            }
            Value::String(ref s) => {
                let mut chunks = vec![];
                for piece in Parser::new(s) {
                    let c = match piece {
                        Piece::Text(t) => Chunk::Text(t.to_owned()),
                        Piece::Argument { name: "mdc", args } => {
                            if args.len() != 1 {
                                return Err(format!("expected exactly 1 argument: `{}`", s).into());
                            }
                            Chunk::Mdc(args[0].to_owned())
                        }
                        Piece::Argument { name, .. } => {
                            return Err(format!("unknown argument `{}`: `{}`", name, s).into());
                        }
                        Piece::Error(e) => return Err(format!("{}: `{}`", e, s).into()),
                    };
                    chunks.push(c);
                }
                ValueTemplate::String(chunks)
            }
            Value::Bool(b) => ValueTemplate::Bool(b),
            Value::Bytes(ref b) => ValueTemplate::Bytes(b.clone()),
            Value::Char(c) => ValueTemplate::Char(c),
            Value::F32(f) => ValueTemplate::F32(f),
            Value::F64(f) => ValueTemplate::F64(f),
            Value::I8(i) => ValueTemplate::I8(i),
            Value::I16(i) => ValueTemplate::I16(i),
            Value::I32(i) => ValueTemplate::I32(i),
            Value::I64(i) => ValueTemplate::I64(i),
            Value::Isize(i) => ValueTemplate::Isize(i),
            Value::U8(u) => ValueTemplate::U8(u),
            Value::U16(u) => ValueTemplate::U16(u),
            Value::U32(u) => ValueTemplate::U32(u),
            Value::U64(u) => ValueTemplate::U64(u),
            Value::Usize(u) => ValueTemplate::Usize(u),
            Value::Unit => ValueTemplate::Unit,
            Value::UnitStruct(s) => ValueTemplate::UnitStruct(s),
        };
        Ok(value)
    }

    fn discriminant(&self) -> usize {
        match *self {
            ValueTemplate::Bool(..) => 0,
            ValueTemplate::Usize(..) => 1,
            ValueTemplate::U8(..) => 2,
            ValueTemplate::U16(..) => 3,
            ValueTemplate::U32(..) => 4,
            ValueTemplate::U64(..) => 5,
            ValueTemplate::Isize(..) => 6,
            ValueTemplate::I8(..) => 7,
            ValueTemplate::I16(..) => 8,
            ValueTemplate::I32(..) => 9,
            ValueTemplate::I64(..) => 10,
            ValueTemplate::F32(..) => 11,
            ValueTemplate::F64(..) => 12,
            ValueTemplate::Char(..) => 13,
            ValueTemplate::String(..) => 14,
            ValueTemplate::Unit => 15,
            ValueTemplate::UnitStruct(..) => 16,
            ValueTemplate::Option(..) => 17,
            ValueTemplate::Newtype(..) => 18,
            ValueTemplate::Seq(..) => 19,
            ValueTemplate::Map(..) => 20,
            ValueTemplate::Bytes(..) => 21,
        }
    }

    fn keys(&self, keys: &mut HashSet<String>) {
        match *self {
            ValueTemplate::Map(ref m) => {
                for (k, v) in m {
                    k.keys(keys);
                    v.keys(keys);
                }
            }
            ValueTemplate::Newtype(ref v) => v.keys(keys),
            ValueTemplate::Option(ref v) => {
                if let Some(ref v) = *v {
                    v.keys(keys);
                }
            }
            ValueTemplate::Seq(ref vs) => {
                for v in vs {
                    v.keys(keys);
                }
            }
            ValueTemplate::String(ref chunks) => {
                for chunk in chunks {
                    if let Chunk::Mdc(ref key) = *chunk {
                        keys.insert(key.clone());
                    }
                }
            }
            _ => {}
        }
    }

    fn expand(&self) -> Value {
        match *self {
            ValueTemplate::Map(ref m) => {
                Value::Map(m.iter().map(|(k, v)| (k.expand(), v.expand())).collect())
            }
            ValueTemplate::Newtype(ref v) => Value::Newtype(Box::new(v.expand())),
            ValueTemplate::Option(ref v) => Value::Option(v.as_ref().map(|v| Box::new(v.expand()))),
            ValueTemplate::Seq(ref vs) => Value::Seq(vs.iter().map(|v| v.expand()).collect()),
            ValueTemplate::String(ref chunks) => {
                let mut s = String::new();
                for chunk in chunks {
                    match *chunk {
                        Chunk::Text(ref t) => s.push_str(t),
                        Chunk::Mdc(ref k) => {
                            log_mdc::get(k, |v| {
                                match v {
                                    Some(v) => s.push_str(v),
                                    None => s.push_str("<missing>"),
                                }
                            })
                        }
                    }
                }
                Value::String(s)
            }
            ValueTemplate::Bool(b) => Value::Bool(b),
            ValueTemplate::Bytes(ref b) => Value::Bytes(b.clone()),
            ValueTemplate::Char(c) => Value::Char(c),
            ValueTemplate::F32(f) => Value::F32(f),
            ValueTemplate::F64(f) => Value::F64(f),
            ValueTemplate::I8(i) => Value::I8(i),
            ValueTemplate::I16(i) => Value::I16(i),
            ValueTemplate::I32(i) => Value::I32(i),
            ValueTemplate::I64(i) => Value::I64(i),
            ValueTemplate::Isize(i) => Value::Isize(i),
            ValueTemplate::U8(i) => Value::U8(i),
            ValueTemplate::U16(i) => Value::U16(i),
            ValueTemplate::U32(i) => Value::U32(i),
            ValueTemplate::U64(i) => Value::U64(i),
            ValueTemplate::Usize(i) => Value::Usize(i),
            ValueTemplate::Unit => Value::Unit,
            ValueTemplate::UnitStruct(n) => Value::UnitStruct(n),
        }
    }
}

