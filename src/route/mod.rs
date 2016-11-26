//! Routers.
//!
//! A router determines the appender to which a log event should be sent.
use linked_hash_map::LinkedHashMap;
use log::LogRecord;
use log4rs::append::Append;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(feature = "file")]
use log4rs::file::Deserializable;

use {CacheInner, AppenderInner};

#[cfg(feature = "pattern-router")]
pub mod pattern;

struct TrackedAppender {
    appender: Appender,
    used: Instant,
}

/// A cache of appenders.
///
/// It stores appenders identified by arbitrary strings. It is up to the router to decide how those
/// strings are formatted.
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
    /// Looks up the entry corresponding to the specified key.
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
            Some(appender) => Entry::Occupied(OccupiedEntry(self, appender)),
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

/// A (possibly vacant) entry of a `Cache`.
pub enum Entry<'a> {
    /// An entry which is present in the `Cache`.
    Occupied(OccupiedEntry<'a>),
    /// An entry which is not present in the `Cache`.
    Vacant(VacantEntry<'a>),
}

impl<'a> Entry<'a> {
    /// Returns the value of the entry, using the provided closure to create and insert it if the
    /// entry is not present in the cache.
    pub fn or_insert_with<F>(self, f: F) -> Appender
        where F: FnOnce() -> Box<Append>
    {
        match self {
            Entry::Occupied(e) => e.into_value(),
            Entry::Vacant(e) => e.insert(f()),
        }
    }
}

/// An entry which exists in the cache.
pub struct OccupiedEntry<'a>(&'a mut Cache, Appender);

impl<'a> OccupiedEntry<'a> {
    /// Consumes the entry, returning the associated appender.
    pub fn into_value(self) -> Appender {
        self.1
    }
}

/// An entry which does not exist in the cache.
pub struct VacantEntry<'a> {
    cache: &'a mut Cache,
    key: String,
    time: Instant,
}

impl<'a> VacantEntry<'a> {
    /// Inserts an appender into the cache, returning the wrapped version of it.
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

/// An opaque, wrapped appender stored by the `Cache`.
pub struct Appender(Arc<Box<Append>>);

impl AppenderInner for Appender {
    fn appender(&self) -> &Append {
        &**self.0
    }
}

/// A trait implemented by types that can route log events to appenders.
pub trait Route: fmt::Debug + 'static + Sync + Send {
    /// Returns the appender to which the provided log event should be routed.
    fn route(&self, record: &LogRecord, cache: &mut Cache) -> Result<Appender, Box<Error>>;
}

#[cfg(feature = "file")]
impl Deserializable for Route {
    fn name() -> &'static str {
        "router"
    }
}
