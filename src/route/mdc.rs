use log::LogRecord;
use log_mdc;
use log4rs::file::Deserializers;
use regex::{Regex, Captures};
use serde_value::Value;
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fmt::{self, Write};

use route::{Route, Cache, Appender, Entry};

lazy_static! {
    static ref PATTERN: Regex = Regex::new("{mdc:([^}]+)}").unwrap();
}

fn get_keys(config: &Value, keys: &mut HashSet<String>) {
    match *config {
        Value::Map(ref m) => {
            for (k, v) in m {
                get_keys(k, keys);
                get_keys(v, keys);
            }
        }
        Value::Newtype(ref v) => get_keys(v, keys),
        Value::Option(ref v) => {
            if let Some(ref v) = *v {
                get_keys(v, keys);
            }
        },
        Value::Seq(ref vs) => {
            for v in vs {
                get_keys(v, keys);
            }
        }
        Value::String(ref s) => {
            for capture in PATTERN.captures_iter(s) {
                keys.insert(capture.at(1).unwrap().to_owned());
            }
        }
        _ => {}
    }
}

fn expand_config(config: &Value) -> Value {
    match *config {
        Value::Map(ref m) => {
            let mut m2 = BTreeMap::new();
            for (k, v) in m {
                m2.insert(expand_config(k), expand_config(v));
            }
            Value::Map(m2)
        }
        Value::Newtype(ref v) => Value::Newtype(Box::new(expand_config(v))),
        Value::Option(ref v) => {
            Value::Option(v.as_ref().map(|v| Box::new(expand_config(v))))
        }
        Value::Seq(ref vs) => Value::Seq(vs.iter().map(|v| expand_config(v)).collect()),
        Value::String(ref s) => {
            let s = PATTERN.replace(s, |c: &Captures| {
                log_mdc::get(c.at(1).unwrap(), |v| {
                    match v {
                        Some(v) => v.to_owned(),
                        None => "<missing>".to_owned(),
                    }
                })
            });
            Value::String(s)
        }
        Value::Bool(b) => Value::Bool(b),
        Value::Bytes(ref b) => Value::Bytes(b.clone()),
        Value::Char(c) => Value::Char(c),
        Value::F32(f) => Value::F32(f),
        Value::F64(f) => Value::F64(f),
        Value::I8(i) => Value::I8(i),
        Value::I16(i) => Value::I16(i),
        Value::I32(i) => Value::I32(i),
        Value::I64(i) => Value::I64(i),
        Value::Isize(i) => Value::Isize(i),
        Value::U8(u) => Value::U8(u),
        Value::U16(u) => Value::U16(u),
        Value::U32(u) => Value::U32(u),
        Value::U64(u) => Value::U64(u),
        Value::Usize(u) => Value::Usize(u),
        Value::Unit => Value::Unit,
        Value::UnitStruct(s) => Value::UnitStruct(s),
    }
}

pub struct MdcRouter {
    deserializers: Deserializers,
    kind: String,
    config: Value,
    keys: HashSet<String>,
}

impl fmt::Debug for MdcRouter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("MdcRouter")
            .finish()
    }
}

impl Route for MdcRouter {
    fn route(&self, record: &LogRecord, cache: &mut Cache) -> Result<Appender, Box<Error>> {
        match cache.entry(self.key()) {
            Entry::Occupied(e) => Ok(e.into_value()),
            Entry::Vacant(e) => {
                let config = expand_config(&self.config);
                let appender = self.deserializers.deserialize(&self.kind, config)?;
                Ok(e.insert(appender))
            }
        }
    }
}

impl MdcRouter {
    fn key(&self) -> String {
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
}