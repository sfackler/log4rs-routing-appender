use linked_hash_map::LinkedHashMap;
use log::LogRecord;
use log4rs::append::Append;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(feature = "file")]
use log4rs::file::Deserializable;
#[cfg(feature = "file")]
use serde::de;
#[cfg(feature = "file")]
use serde_value::Value;
#[cfg(feature = "file")]
use std::collections::BTreeMap;

use {CacheInner, AppenderInner};

#[cfg(feature = "pattern-router")]
pub mod pattern;

struct TrackedAppender {
    appender: Appender,
    used: Instant,
}

pub struct Cache {
    map: LinkedHashMap<String, TrackedAppender>,
    ttl: Duration,
}

impl CacheInner for Cache {
    fn new(ttl: Duration) -> Cache {
        Cache {
            map: LinkedHashMap::new(),
            ttl: ttl,
        }
    }
}

impl Cache {
    pub fn entry<'a>(&'a mut self, key: String) -> Entry<'a> {
        let now = Instant::now();
        self.purge(now);

        let entry = match self.map.get_refresh(&key) {
            Some(entry) => {
                entry.used = now;
                Some(Appender(entry.appender.0.clone()))
            }
            None => None,
        };

        match entry {
            Some(appender) => {
                Entry::Occupied(OccupiedEntry(self, appender))
            }
            None => {
                Entry::Vacant(VacantEntry {
                    cache: self,
                    key: key,
                    time: now,
                })
            }
        }
    }

    fn purge(&mut self, now: Instant) {
        let timeout = now - self.ttl;
        loop {
            match self.map.front() {
                Some((_, v)) if v.used <= timeout => {}
                _ => break,
            }
            self.map.pop_front();
        }
    }
}

pub enum Entry<'a> {
    Occupied(OccupiedEntry<'a>),
    Vacant(VacantEntry<'a>),
}

impl<'a> Entry<'a> {
    pub fn or_insert_with<F>(self, f: F) -> Appender
        where F: FnOnce() -> Box<Append>
    {
        match self {
            Entry::Occupied(e) => e.into_value(),
            Entry::Vacant(e) => e.insert(f()),
        }
    }
}

pub struct OccupiedEntry<'a>(&'a mut Cache, Appender);

impl<'a> OccupiedEntry<'a> {
    pub fn into_value(self) -> Appender {
        self.1
    }
}

pub struct VacantEntry<'a> {
    cache: &'a mut Cache,
    key: String,
    time: Instant,
}

impl<'a> VacantEntry<'a> {
    pub fn insert(self, value: Box<Append>) -> Appender {
        let appender = Arc::new(value);
        let tracked = TrackedAppender {
            appender: Appender(appender.clone()),
            used: self.time,
        };
        self.cache.map.insert(self.key, tracked);
        Appender(appender)
    }
}

pub struct Appender(Arc<Box<Append>>);

impl AppenderInner for Appender {
    fn appender(&self) -> &Append {
        &**self.0
    }
}

pub trait Route: fmt::Debug + 'static + Sync + Send {
    fn route(&self, record: &LogRecord, cache: &mut Cache) -> Result<Appender, Box<Error>>;
}

#[cfg(feature = "file")]
impl Deserializable for Route {
    fn name() -> &'static str {
        "router"
    }
}

/// Configuration for a router.
#[derive(PartialEq, Eq, Debug)]
#[cfg(feature = "file")]
pub struct RouterConfig {
    /// The router kind.
    pub kind: String,
    /// The router configuration.
    pub config: Value,
}

#[cfg(feature = "file")]
impl de::Deserialize for RouterConfig {
    fn deserialize<D>(d: &mut D) -> Result<RouterConfig, D::Error>
        where D: de::Deserializer
    {
        let mut map = BTreeMap::<Value, Value>::deserialize(d)?;

        let kind = match map.remove(&Value::String("kind".to_owned())) {
            Some(kind) => kind.deserialize_into().map_err(|e| e.to_error())?,
            None => return Err(de::Error::missing_field("kind")),
        };

        Ok(RouterConfig {
            kind: kind,
            config: Value::Map(map),
        })
    }
}
