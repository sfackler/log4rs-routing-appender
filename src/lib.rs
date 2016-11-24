extern crate antidote;
extern crate log;
extern crate log4rs;
extern crate lru_cache;

#[cfg(feature = "log-mdc")]
extern crate log_mdc;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde-value")]
extern crate serde_value;
#[cfg(feature = "ordered-float")]
extern crate ordered_float;

use antidote::Mutex;
use log::LogRecord;
use log4rs::append::Append;
use std::error::Error;
use std::fmt;

#[cfg(feature = "file")]
use log4rs::file::{Deserialize, Deserializers};

use route::{Cache, Route};

pub mod route;

#[cfg(feature = "file")]
include!("serde.rs");

#[cfg(feature = "file")]
pub fn register(d: &mut Deserializers) {
    d.insert("routing", RoutingAppenderDeserializer);

    #[cfg(feature = "pattern-router")]
    d.insert("pattern", route::pattern::PatternRouterDeserializer);
}

pub struct RoutingAppender {
    router: Box<Route>,
    cache: Mutex<Cache>,
}

impl fmt::Debug for RoutingAppender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RoutingAppender")
            .field("router", &self.router)
            .finish()
    }
}

impl Append for RoutingAppender {
    fn append(&self, record: &LogRecord) -> Result<(), Box<Error>> {
        let appender = self.router.route(record, &mut self.cache.lock())?;
        appender.appender().append(record)
    }
}

#[cfg(feature = "file")]
pub struct RoutingAppenderDeserializer;

#[cfg(feature = "file")]
impl Deserialize for RoutingAppenderDeserializer {
    type Trait = Append;
    type Config = RoutingAppenderConfig;

    fn deserialize(&self,
                   config: RoutingAppenderConfig,
                   deserializers: &Deserializers)
                   -> Result<Box<Append>, Box<Error>> {
        let router = deserializers.deserialize(&config.router.kind, config.router.config)?;
        let cache = Cache::new(config.cache.size);
        Ok(Box::new(RoutingAppender {
            router: router,
            cache: Mutex::new(cache),
        }))
    }
}

trait CacheInner {
    fn new(capacity: usize) -> Cache;
}

trait AppenderInner {
    fn appender(&self) -> &Append;
}
