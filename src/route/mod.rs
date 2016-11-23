use log::LogRecord;
use log4rs::append::Append;
use lru_cache::LruCache;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use AppenderInner;

#[cfg(feature = "mdc-router")]
pub mod mdc;

pub struct Cache(LruCache<String, Appender>);

impl Cache {
    pub fn entry<'a>(&'a mut self, key: String) -> Entry<'a> {
        let entry = self.0.get_mut(&key).map(|a| Appender(a.0.clone()));
        match entry {
            Some(appender) => Entry::Occupied(OccupiedEntry(self, appender)),
            None => Entry::Vacant(VacantEntry(self, key)),
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

pub struct VacantEntry<'a>(&'a mut Cache, String);

impl<'a> VacantEntry<'a> {
    pub fn insert(self, value: Box<Append>) -> Appender {
        let appender = Appender(Arc::new(value));
        (self.0).0.insert(self.1, Appender(appender.0.clone()));
        appender
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