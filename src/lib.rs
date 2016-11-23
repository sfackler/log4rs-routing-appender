extern crate antidote;
extern crate log;
extern crate log4rs;
extern crate lru_cache;

use antidote::Mutex;
use log::LogRecord;
use log4rs::append::Append;
use std::error::Error;
use std::fmt;

use route::{Cache, Route};

pub mod route;

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

trait AppenderInner {
    fn appender(&self) -> &Append;
}
