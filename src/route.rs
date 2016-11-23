use log::LogRecord;
use log4rs::append::Append;
use lru_cache::LruCache;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use AppenderInner;

pub struct Cache(LruCache<String, Appender>);

pub struct Appender(Arc<Box<Append>>);

impl AppenderInner for Appender {
    fn appender(&self) -> &Append {
        &**self.0
    }
}

pub trait Route: fmt::Debug + 'static + Sync + Send {
    fn route(&self, record: &LogRecord, cache: &mut Cache) -> Result<Appender, Box<Error>>;
}