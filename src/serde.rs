#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_RoutingAppenderConfig: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for RoutingAppenderConfig {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<RoutingAppenderConfig, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    1usize => { Ok(__Field::__field1) }
                                    _ =>
                                    Err(_serde::de::Error::invalid_value("expected a field")),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "router" => { Ok(__Field::__field0) }
                                    "cache" => { Ok(__Field::__field1) }
                                    _ =>
                                    Err(_serde::de::Error::unknown_field(value)),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"router" => { Ok(__Field::__field0) }
                                    b"cache" => { Ok(__Field::__field1) }
                                    _ => {
                                        let value =
                                            ::std::string::String::from_utf8_lossy(value);
                                        Err(_serde::de::Error::unknown_field(&value))
                                    }
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    RoutingAppenderConfig;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     ->
                         ::std::result::Result<RoutingAppenderConfig,
                                               __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: <
                                       route::RouterConfig > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit :: < CacheConfig > (
                                       )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(1usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(RoutingAppenderConfig{router: __field0,
                                                 cache: __field1,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     ->
                         ::std::result::Result<RoutingAppenderConfig,
                                               __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<route::RouterConfig> = None;
                        let mut __field1: Option<CacheConfig> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("router"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  route::RouterConfig > (
                                                  )));
                                }
                                __Field::__field1 => {
                                    if __field1.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("cache"));
                                    }
                                    __field1 =
                                        Some(try!(visitor . visit_value :: <
                                                  CacheConfig > (  )));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "router" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None => ::std::default::Default::default(),
                            };
                        Ok(RoutingAppenderConfig{router: __field0,
                                                 cache: __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["router", "cache"];
                deserializer.deserialize_struct("RoutingAppenderConfig",
                                                FIELDS, __Visitor)
            }
        }
    };
/// Configuration for the `RoutingAppender`.
pub struct RoutingAppenderConfig {
    router: route::RouterConfig,
    cache: CacheConfig,
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CacheConfig: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CacheConfig {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CacheConfig, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    _ =>
                                    Err(_serde::de::Error::invalid_value("expected a field")),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "idle_timeout" => {
                                        Ok(__Field::__field0)
                                    }
                                    _ =>
                                    Err(_serde::de::Error::unknown_field(value)),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"idle_timeout" => {
                                        Ok(__Field::__field0)
                                    }
                                    _ => {
                                        let value =
                                            ::std::string::String::from_utf8_lossy(value);
                                        Err(_serde::de::Error::unknown_field(&value))
                                    }
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    CacheConfig;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CacheConfig, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match {
                                      struct __SerdeDeserializeWithStruct {
                                          value: Duration,
                                          phantom: ::std::marker::PhantomData<CacheConfig>,
                                      }
                                      impl _serde::de::Deserialize for
                                       __SerdeDeserializeWithStruct {
                                          fn deserialize<__D>(__d: &mut __D)
                                           ->
                                               ::std::result::Result<Self,
                                                                     __D::Error>
                                           where
                                           __D: _serde::de::Deserializer {
                                              let value =
                                                  try!(de_duration ( __d ));
                                              Ok(__SerdeDeserializeWithStruct{value:
                                                                                  value,
                                                                              phantom:
                                                                                  ::std::marker::PhantomData,})
                                          }
                                      }
                                      try!(visitor . visit :: <
                                           __SerdeDeserializeWithStruct > (
                                           )).map(|wrap| wrap.value)
                                  } {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(CacheConfig{idle_timeout: __field0,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CacheConfig, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<Duration> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("idle_timeout"));
                                    }
                                    __field0 =
                                        Some(({
                                                  struct __SerdeDeserializeWithStruct {
                                                      value: Duration,
                                                      phantom: ::std::marker::PhantomData<CacheConfig>,
                                                  }
                                                  impl _serde::de::Deserialize
                                                   for
                                                   __SerdeDeserializeWithStruct
                                                   {
                                                      fn deserialize<__D>(__d:
                                                                              &mut __D)
                                                       ->
                                                           ::std::result::Result<Self,
                                                                                 __D::Error>
                                                       where
                                                       __D: _serde::de::Deserializer {
                                                          let value =
                                                              try!(de_duration
                                                                   ( __d ));
                                                          Ok(__SerdeDeserializeWithStruct{value:
                                                                                              value,
                                                                                          phantom:
                                                                                              ::std::marker::PhantomData,})
                                                      }
                                                  }
                                                  try!(visitor . visit_value
                                                       :: <
                                                       __SerdeDeserializeWithStruct
                                                       > (  )).value
                                              }));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None => idle_time_default(),
                            };
                        Ok(CacheConfig{idle_timeout: __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["idle_timeout"];
                deserializer.deserialize_struct("CacheConfig", FIELDS,
                                                __Visitor)
            }
        }
    };
struct CacheConfig {
    idle_timeout: Duration,
}
