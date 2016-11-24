use log4rs::file::{Deserialize, Deserializers};
use log::LogRecord;
use serde::de;
use serde_value::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use route::{Route, Cache, Appender, Entry};
use route::pattern::template::Template;

mod parser;
mod template;

include!("serde.rs");

pub struct PatternRouter {
    deserializers: Deserializers,
    kind: String,
    config: Template,
}

impl fmt::Debug for PatternRouter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PatternRouter")
            .finish()
    }
}

impl Route for PatternRouter {
    fn route(&self, _: &LogRecord, cache: &mut Cache) -> Result<Appender, Box<Error>> {
        match cache.entry(self.config.key()) {
            Entry::Occupied(e) => Ok(e.into_value()),
            Entry::Vacant(e) => {
                let appender = self.deserializers.deserialize(&self.kind, self.config.expand())?;
                Ok(e.insert(appender))
            }
        }
    }
}

pub struct PatternRouterDeserializer;

impl Deserialize for PatternRouterDeserializer {
    type Trait = Route;
    type Config = PatternRouterConfig;

    fn deserialize(&self,
                   config: PatternRouterConfig,
                   deserializers: &Deserializers)
                   -> Result<Box<Route>, Box<Error>> {
        Ok(Box::new(PatternRouter {
            deserializers: deserializers.clone(),
            kind: config.pattern.kind,
            config: Template::new(&config.pattern.config)?,
        }))
    }
}

struct AppenderConfig {
    kind: String,
    config: Value,
}

impl de::Deserialize for AppenderConfig {
    fn deserialize<D>(d: &mut D) -> Result<AppenderConfig, D::Error>
        where D: de::Deserializer
    {
        let mut map = BTreeMap::<Value, Value>::deserialize(d)?;

        let kind = match map.remove(&Value::String("kind".to_owned())) {
            Some(kind) => kind.deserialize_into().map_err(|e| e.to_error())?,
            None => return Err(de::Error::missing_field("kind")),
        };

        Ok(AppenderConfig {
            kind: kind,
            config: Value::Map(map),
        })
    }
}
