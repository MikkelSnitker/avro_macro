#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{
    fmt::format, io::{Read, Write},
    time,
};
use apache_avro::{schema, AvroSchema};
use avro_macro::{schema, TaggedEnum};
use serde::{
    de::{self, value},
    ser::{self, SerializeSeq, SerializeTupleVariant},
    Deserialize, Serialize,
};
mod Test {
    pub mod notification_click {
        pub struct NotificationClick {
            #[avro(rename = "countryCode")]
            #[serde(rename = "countryCode")]
            pub country_code: String,
            #[avro(rename = "createdAt")]
            #[serde(rename = "createdAt")]
            pub created_at: String,
            #[avro(rename = "entityId")]
            #[serde(rename = "entityId")]
            pub entity_id: String,
            #[avro(rename = "entityType")]
            #[serde(rename = "entityType")]
            pub entity_type: String,
            #[avro(rename = "id")]
            #[serde(rename = "id")]
            pub id: String,
            #[avro(rename = "type")]
            #[serde(rename = "type")]
            pub r#type: String,
            #[avro(rename = "url")]
            #[serde(rename = "url")]
            pub url: String,
            #[avro(rename = "userId")]
            #[serde(rename = "userId")]
            pub user_id: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NotificationClick {
            #[inline]
            fn clone(&self) -> NotificationClick {
                NotificationClick {
                    country_code: ::core::clone::Clone::clone(&self.country_code),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    entity_id: ::core::clone::Clone::clone(&self.entity_id),
                    entity_type: ::core::clone::Clone::clone(&self.entity_type),
                    id: ::core::clone::Clone::clone(&self.id),
                    r#type: ::core::clone::Clone::clone(&self.r#type),
                    url: ::core::clone::Clone::clone(&self.url),
                    user_id: ::core::clone::Clone::clone(&self.user_id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for NotificationClick {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "NotificationClick",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "countryCode",
                        &self.country_code,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "entityId",
                        &self.entity_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "entityType",
                        &self.entity_type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.r#type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "userId",
                        &self.user_id,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for NotificationClick {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "countryCode" => _serde::__private::Ok(__Field::__field0),
                                "createdAt" => _serde::__private::Ok(__Field::__field1),
                                "entityId" => _serde::__private::Ok(__Field::__field2),
                                "entityType" => _serde::__private::Ok(__Field::__field3),
                                "id" => _serde::__private::Ok(__Field::__field4),
                                "type" => _serde::__private::Ok(__Field::__field5),
                                "url" => _serde::__private::Ok(__Field::__field6),
                                "userId" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"countryCode" => _serde::__private::Ok(__Field::__field0),
                                b"createdAt" => _serde::__private::Ok(__Field::__field1),
                                b"entityId" => _serde::__private::Ok(__Field::__field2),
                                b"entityType" => _serde::__private::Ok(__Field::__field3),
                                b"id" => _serde::__private::Ok(__Field::__field4),
                                b"type" => _serde::__private::Ok(__Field::__field5),
                                b"url" => _serde::__private::Ok(__Field::__field6),
                                b"userId" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<NotificationClick>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = NotificationClick;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct NotificationClick",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct NotificationClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(NotificationClick {
                                country_code: __field0,
                                created_at: __field1,
                                entity_id: __field2,
                                entity_type: __field3,
                                id: __field4,
                                r#type: __field5,
                                url: __field6,
                                user_id: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "countryCode",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "entityId",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "entityType",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("userId"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("countryCode")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("entityId")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("entityType")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("type")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("userId")?
                                }
                            };
                            _serde::__private::Ok(NotificationClick {
                                country_code: __field0,
                                created_at: __field1,
                                entity_id: __field2,
                                entity_type: __field3,
                                id: __field4,
                                r#type: __field5,
                                url: __field6,
                                user_id: __field7,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "countryCode",
                        "createdAt",
                        "entityId",
                        "entityType",
                        "id",
                        "type",
                        "url",
                        "userId",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "NotificationClick",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<NotificationClick>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for NotificationClick {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for NotificationClick {
            #[inline]
            fn eq(&self, other: &NotificationClick) -> bool {
                self.country_code == other.country_code
                    && self.created_at == other.created_at
                    && self.entity_id == other.entity_id
                    && self.entity_type == other.entity_type && self.id == other.id
                    && self.r#type == other.r#type && self.url == other.url
                    && self.user_id == other.user_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NotificationClick {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "country_code",
                    "created_at",
                    "entity_id",
                    "entity_type",
                    "id",
                    "type",
                    "url",
                    "user_id",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.country_code,
                    &self.created_at,
                    &self.entity_id,
                    &self.entity_type,
                    &self.id,
                    &self.r#type,
                    &self.url,
                    &&self.user_id,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "NotificationClick",
                    names,
                    values,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for NotificationClick {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("NotificationClick")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "NotificationClick",
                                ),
                            );
                            res
                        }[..],
                    )
                    .fully_qualified_name(enclosing_namespace);
                let enclosing_namespace = &name.namespace;
                if named_schemas.contains_key(&name) {
                    apache_avro::schema::Schema::Ref {
                        name: name.clone(),
                    }
                } else {
                    named_schemas
                        .insert(
                            name.clone(),
                            apache_avro::schema::Schema::Ref {
                                name: name.clone(),
                            },
                        );
                    let schema_fields = <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            apache_avro::schema::RecordField {
                                name: "countryCode".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "createdAt".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 1usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "entityId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 2usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "entityType".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 3usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "id".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 4usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "type".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 5usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "url".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 6usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "userId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 7usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("NotificationClick")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "NotificationClick",
                                    ),
                                );
                                res
                            }[..],
                        );
                    let lookup: std::collections::BTreeMap<String, usize> = schema_fields
                        .iter()
                        .map(|field| (field.name.to_owned(), field.position))
                        .collect();
                    apache_avro::schema::Schema::Record(apache_avro::schema::RecordSchema {
                        name,
                        aliases: None,
                        doc: None,
                        fields: schema_fields,
                        lookup,
                        attributes: Default::default(),
                    })
                }
            }
        }
    }
    pub use notification_click::NotificationClick;
    use serde::{Deserialize, Serialize};
    pub enum Events {
        NotificationClick(NotificationClick),
    }
    pub fn from_str(tag: &str, value: &str) -> Result<Events, serde_json::Error> {
        ::core::panicking::panic("not yet implemented")
    }
}
fn main() {
    {
        ::std::io::_print(format_args!("OK\n"));
    };
}
