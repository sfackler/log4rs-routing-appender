#![cfg(feature = "pattern-router")]

#[macro_use]
extern crate log;
extern crate log_mdc;
extern crate log4rs;
extern crate log4rs_routing_appender;
extern crate serde;
extern crate serde_value;
extern crate serde_yaml;

use log::LogRecord;
use log4rs::file::{Deserialize, Deserializers, RawConfig};
use log4rs::config::Config;
use log4rs::append::Append;
use log4rs_routing_appender::register;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

thread_local! {
    static APPENDS: RefCell<Vec<u32>> = RefCell::new(vec![]);
}

#[derive(Debug)]
struct TestAppender(u32);

impl Append for TestAppender {
    fn append(&self, _: &LogRecord) -> Result<(), Box<Error + Sync + Send>> {
        APPENDS.with(|a| a.borrow_mut().push(self.0));
        Ok(())
    }
}

struct TestAppenderDeserializer;

impl Deserialize for TestAppenderDeserializer {
    type Config = HashMap<String, String>;
    type Trait = Append;

    fn deserialize(&self,
                   config: HashMap<String, String>,
                   _: &Deserializers)
                   -> Result<Box<Append>, Box<Error + Sync + Send>> {
        Ok(Box::new(TestAppender(config["key"].parse().unwrap())))
    }
}

#[test]
fn pattern() {
    let mut d = Deserializers::new();
    register(&mut d);
    d.insert("test", TestAppenderDeserializer);

    let config = r#"
appenders:
  router:
    kind: routing
    router:
      kind: pattern
      pattern:
        kind: test
        key: "${mdc(key)}"
root:
  level: info
  appenders:
  - router
"#;
    let config = serde_yaml::from_str::<RawConfig>(config).unwrap();
    let (appenders, errors) = config.appenders_lossy(&d);
    assert!(errors.is_empty());
    let config = Config::builder()
        .appenders(appenders)
        .loggers(config.loggers())
        .build(config.root())
        .unwrap();
    log4rs::init_config(config).unwrap();

    log_mdc::insert("key", "0");
    error!("");
    log_mdc::insert("key", "1");
    error!("");
    log_mdc::insert("key", "0");
    error!("");
    log_mdc::insert("key", "1");
    error!("");

    APPENDS.with(|a| assert_eq!(*a.borrow(), [0, 1, 0, 1]));
}
