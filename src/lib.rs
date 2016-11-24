extern crate antidote;
extern crate linked_hash_map;
extern crate log;
extern crate log4rs;

#[cfg(feature = "humantime")]
extern crate humantime;
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
use std::time::Duration;

#[cfg(feature = "file")]
use log4rs::file::{Deserialize, Deserializers};
#[cfg(feature = "file")]
use serde::de;

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
        let cache = Cache::new(config.cache.idle_timeout);
        Ok(Box::new(RoutingAppender {
            router: router,
            cache: Mutex::new(cache),
        }))
    }
}

#[cfg(feature = "file")]
fn de_duration<D>(d: &mut D) -> Result<Option<Duration>, D::Error>
    where D: de::Deserializer
{
    struct S(Duration);

    impl de::Deserialize for S {
        fn deserialize<D>(d: &mut D) -> Result<S, D::Error>
            where D: de::Deserializer
        {
            struct V;

            impl de::Visitor for V {
                type Value = S;

                fn visit_str<E>(&mut self, v: &str) -> Result<S, E>
                    where E: de::Error
                {
                    humantime::parse_duration(v)
                        .map(S)
                        .map_err(|e| E::invalid_value(&e.to_string()))
                }
            }

            d.deserialize(V)
        }
    }

    Option::<S>::deserialize(d).map(|r| r.map(|s| s.0))
}

#[cfg(feature = "file")]
impl Default for CacheConfig {
    fn default() -> CacheConfig {
        CacheConfig {
            idle_timeout: idle_time_default(),
        }
    }
}

#[cfg(feature = "file")]
fn idle_time_default() -> Duration {
    Duration::from_secs(2 * 60)
}

trait CacheInner {
    fn new(expiration: Duration) -> Cache;
}

trait AppenderInner {
    fn appender(&self) -> &Append;
}
