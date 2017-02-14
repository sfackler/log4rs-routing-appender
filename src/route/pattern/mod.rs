//! A router which constructs appenders from a template configuration.
//!
//! Strings in the configuration template may contain substitution directives. The format is similar
//! to that of the log4rs pattern encoder, except that it is prefixed with a `$` to avoid conflicts
//! with patterns in the templated configuration itself. Format specifications are not supported.
//!
//! Only one formatter is currently supported:
//!
//! * `mdc` - An entry from the [MDC][MDC]. The first argument is required, and specifies the key to
//!     look up. If the key is not present, an error is raised. A second, optional argument allows
//!     a replacement string to be used if the key is not present.
//!
//! # Examples
//!
//! Assume the MDC looks like `{user_id: sfackler}`.
//!
//! ```yaml
//! kind: file
//! path: "logs/${mdc(user_id)}/${mdc(job_id)}.log"
//! ```
//!
//! will fail to parse, since there is no MDC entry for `job_id`. If we add a default value, like
//!
//! ```yaml
//! kind: file
//! path: "logs/${mdc(user_id)}/${mdc(job_id)(no_job)}.log"
//! ```
//!
//! it will then parse to
//!
//! ```yaml
//! kind: file
//! path: "logs/sfackler/no_job.log"
//! ```
//!
//! [MDC]: https://crates.io/crates/log-mdc
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

/// Configuration for the `PatternRouter`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatternRouterConfig {
    pattern: AppenderConfig,
}

/// A router which expands an appender configuration template.
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
    fn route(&self,
             _: &LogRecord,
             cache: &mut Cache)
             -> Result<Appender, Box<Error + Sync + Send>> {
        match cache.entry(self.config.key()) {
            Entry::Occupied(e) => Ok(e.into_value()),
            Entry::Vacant(e) => {
                let appender = self.deserializers.deserialize(&self.kind, self.config.expand()?)?;
                Ok(e.insert(appender))
            }
        }
    }
}

/// A deserializer for the `PatternRouter`.
///
/// # Configuration
///
/// ```yaml
/// kind: pattern
///
/// # The configuration template to expand. Required.
/// pattern:
///   kind: file
///   path: "logs/${mdc(user_id)}/${mdc(job_id)(no_job)}.log"
/// ```
pub struct PatternRouterDeserializer;

impl Deserialize for PatternRouterDeserializer {
    type Trait = Route;
    type Config = PatternRouterConfig;

    fn deserialize(&self,
                   config: PatternRouterConfig,
                   deserializers: &Deserializers)
                   -> Result<Box<Route>, Box<Error + Sync + Send>> {
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
    fn deserialize<D>(d: D) -> Result<AppenderConfig, D::Error>
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
