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
    use serde::{Deserialize, Serialize};
    pub mod product_created {
        pub struct ProductCreated {
            #[avro(rename = "createdAt")]
            #[serde(rename = "createdAt")]
            pub created_at: String,
            #[avro(rename = "productRefId")]
            #[serde(rename = "productRefId")]
            pub product_ref_id: String,
            #[avro(rename = "productlistId")]
            #[serde(rename = "productlistId")]
            pub productlist_id: String,
            #[avro(rename = "updatedAt")]
            #[serde(rename = "updatedAt")]
            pub updated_at: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductCreated {
            #[inline]
            fn clone(&self) -> ProductCreated {
                ProductCreated {
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    product_ref_id: ::core::clone::Clone::clone(&self.product_ref_id),
                    productlist_id: ::core::clone::Clone::clone(&self.productlist_id),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductCreated {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductCreated",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "productRefId",
                        &self.product_ref_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "productlistId",
                        &self.productlist_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updatedAt",
                        &self.updated_at,
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
            impl<'de> _serde::Deserialize<'de> for ProductCreated {
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
                                "createdAt" => _serde::__private::Ok(__Field::__field0),
                                "productRefId" => _serde::__private::Ok(__Field::__field1),
                                "productlistId" => _serde::__private::Ok(__Field::__field2),
                                "updatedAt" => _serde::__private::Ok(__Field::__field3),
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
                                b"createdAt" => _serde::__private::Ok(__Field::__field0),
                                b"productRefId" => _serde::__private::Ok(__Field::__field1),
                                b"productlistId" => _serde::__private::Ok(__Field::__field2),
                                b"updatedAt" => _serde::__private::Ok(__Field::__field3),
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
                        marker: _serde::__private::PhantomData<ProductCreated>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductCreated;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductCreated",
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
                                            &"struct ProductCreated with 4 elements",
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
                                            &"struct ProductCreated with 4 elements",
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
                                            &"struct ProductCreated with 4 elements",
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
                                            &"struct ProductCreated with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductCreated {
                                created_at: __field0,
                                product_ref_id: __field1,
                                productlist_id: __field2,
                                updated_at: __field3,
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
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
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
                                                    "productRefId",
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
                                                    "productlistId",
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
                                                    "updatedAt",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("productRefId")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("productlistId")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updatedAt")?
                                }
                            };
                            _serde::__private::Ok(ProductCreated {
                                created_at: __field0,
                                product_ref_id: __field1,
                                productlist_id: __field2,
                                updated_at: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "createdAt",
                        "productRefId",
                        "productlistId",
                        "updatedAt",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductCreated",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ProductCreated>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductCreated {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductCreated {
            #[inline]
            fn eq(&self, other: &ProductCreated) -> bool {
                self.created_at == other.created_at
                    && self.product_ref_id == other.product_ref_id
                    && self.productlist_id == other.productlist_id
                    && self.updated_at == other.updated_at
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductCreated {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ProductCreated",
                    "created_at",
                    &self.created_at,
                    "product_ref_id",
                    &self.product_ref_id,
                    "productlist_id",
                    &self.productlist_id,
                    "updated_at",
                    &&self.updated_at,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for ProductCreated {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductCreated")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductCreated",
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
                                name: "createdAt".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "productRefId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 1usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "productlistId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 2usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "updatedAt".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 3usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("ProductCreated")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductCreated",
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
    pub use product_created::ProductCreated;
    pub mod product_deleted {
        pub struct ProductDeleted {
            #[avro(rename = "deletedAt")]
            #[serde(rename = "deletedAt")]
            pub deleted_at: String,
            #[avro(rename = "id")]
            #[serde(rename = "id")]
            pub id: String,
            #[avro(rename = "productRefId")]
            #[serde(rename = "productRefId")]
            pub product_ref_id: Option<String>,
            #[avro(rename = "productlistId")]
            #[serde(rename = "productlistId")]
            pub productlist_id: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductDeleted {
            #[inline]
            fn clone(&self) -> ProductDeleted {
                ProductDeleted {
                    deleted_at: ::core::clone::Clone::clone(&self.deleted_at),
                    id: ::core::clone::Clone::clone(&self.id),
                    product_ref_id: ::core::clone::Clone::clone(&self.product_ref_id),
                    productlist_id: ::core::clone::Clone::clone(&self.productlist_id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductDeleted {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductDeleted",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "deletedAt",
                        &self.deleted_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "productRefId",
                        &self.product_ref_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "productlistId",
                        &self.productlist_id,
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
            impl<'de> _serde::Deserialize<'de> for ProductDeleted {
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
                                "deletedAt" => _serde::__private::Ok(__Field::__field0),
                                "id" => _serde::__private::Ok(__Field::__field1),
                                "productRefId" => _serde::__private::Ok(__Field::__field2),
                                "productlistId" => _serde::__private::Ok(__Field::__field3),
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
                                b"deletedAt" => _serde::__private::Ok(__Field::__field0),
                                b"id" => _serde::__private::Ok(__Field::__field1),
                                b"productRefId" => _serde::__private::Ok(__Field::__field2),
                                b"productlistId" => _serde::__private::Ok(__Field::__field3),
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
                        marker: _serde::__private::PhantomData<ProductDeleted>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductDeleted;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductDeleted",
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
                                            &"struct ProductDeleted with 4 elements",
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
                                            &"struct ProductDeleted with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ProductDeleted with 4 elements",
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
                                            &"struct ProductDeleted with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductDeleted {
                                deleted_at: __field0,
                                id: __field1,
                                product_ref_id: __field2,
                                productlist_id: __field3,
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
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "deletedAt",
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
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
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
                                                    "productRefId",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "productlistId",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("deletedAt")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("productRefId")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("productlistId")?
                                }
                            };
                            _serde::__private::Ok(ProductDeleted {
                                deleted_at: __field0,
                                id: __field1,
                                product_ref_id: __field2,
                                productlist_id: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "deletedAt",
                        "id",
                        "productRefId",
                        "productlistId",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductDeleted",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ProductDeleted>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductDeleted {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductDeleted {
            #[inline]
            fn eq(&self, other: &ProductDeleted) -> bool {
                self.deleted_at == other.deleted_at && self.id == other.id
                    && self.product_ref_id == other.product_ref_id
                    && self.productlist_id == other.productlist_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductDeleted {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ProductDeleted",
                    "deleted_at",
                    &self.deleted_at,
                    "id",
                    &self.id,
                    "product_ref_id",
                    &self.product_ref_id,
                    "productlist_id",
                    &&self.productlist_id,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for ProductDeleted {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductDeleted")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductDeleted",
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
                                name: "deletedAt".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "id".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 1usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "productRefId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 2usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "productlistId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 3usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("ProductDeleted")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductDeleted",
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
    pub use product_deleted::ProductDeleted;
    pub mod product_match_buy_click {
        pub struct ProductMatchBuyClick {
            #[avro(rename = "countryCode")]
            #[serde(rename = "countryCode")]
            pub country_code: Option<String>,
            #[avro(rename = "createdAt")]
            #[serde(rename = "createdAt")]
            pub created_at: String,
            #[avro(rename = "entityId")]
            #[serde(rename = "entityId")]
            pub entity_id: Option<String>,
            #[avro(rename = "entityType")]
            #[serde(rename = "entityType")]
            pub entity_type: Option<String>,
            #[avro(rename = "fromUrl")]
            #[serde(rename = "fromUrl")]
            pub from_url: Option<String>,
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
            pub user_id: Option<String>,
            #[avro(rename = "wishId")]
            #[serde(rename = "wishId")]
            pub wish_id: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductMatchBuyClick {
            #[inline]
            fn clone(&self) -> ProductMatchBuyClick {
                ProductMatchBuyClick {
                    country_code: ::core::clone::Clone::clone(&self.country_code),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    entity_id: ::core::clone::Clone::clone(&self.entity_id),
                    entity_type: ::core::clone::Clone::clone(&self.entity_type),
                    from_url: ::core::clone::Clone::clone(&self.from_url),
                    id: ::core::clone::Clone::clone(&self.id),
                    r#type: ::core::clone::Clone::clone(&self.r#type),
                    url: ::core::clone::Clone::clone(&self.url),
                    user_id: ::core::clone::Clone::clone(&self.user_id),
                    wish_id: ::core::clone::Clone::clone(&self.wish_id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductMatchBuyClick {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductMatchBuyClick",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
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
                        "fromUrl",
                        &self.from_url,
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
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "wishId",
                        &self.wish_id,
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
            impl<'de> _serde::Deserialize<'de> for ProductMatchBuyClick {
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
                        __field8,
                        __field9,
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
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
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
                                "fromUrl" => _serde::__private::Ok(__Field::__field4),
                                "id" => _serde::__private::Ok(__Field::__field5),
                                "type" => _serde::__private::Ok(__Field::__field6),
                                "url" => _serde::__private::Ok(__Field::__field7),
                                "userId" => _serde::__private::Ok(__Field::__field8),
                                "wishId" => _serde::__private::Ok(__Field::__field9),
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
                                b"fromUrl" => _serde::__private::Ok(__Field::__field4),
                                b"id" => _serde::__private::Ok(__Field::__field5),
                                b"type" => _serde::__private::Ok(__Field::__field6),
                                b"url" => _serde::__private::Ok(__Field::__field7),
                                b"userId" => _serde::__private::Ok(__Field::__field8),
                                b"wishId" => _serde::__private::Ok(__Field::__field9),
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
                        marker: _serde::__private::PhantomData<ProductMatchBuyClick>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductMatchBuyClick;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductMatchBuyClick",
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
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ProductMatchBuyClick with 10 elements",
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
                                            &"struct ProductMatchBuyClick with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ProductMatchBuyClick with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ProductMatchBuyClick with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ProductMatchBuyClick with 10 elements",
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
                                            &"struct ProductMatchBuyClick with 10 elements",
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
                                            &"struct ProductMatchBuyClick with 10 elements",
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
                                            &"struct ProductMatchBuyClick with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct ProductMatchBuyClick with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct ProductMatchBuyClick with 10 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductMatchBuyClick {
                                country_code: __field0,
                                created_at: __field1,
                                entity_id: __field2,
                                entity_type: __field3,
                                from_url: __field4,
                                id: __field5,
                                r#type: __field6,
                                url: __field7,
                                user_id: __field8,
                                wish_id: __field9,
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
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
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
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
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
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
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
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "fromUrl",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("userId"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("wishId"),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
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
                                    _serde::__private::de::missing_field("fromUrl")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("type")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("userId")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("wishId")?
                                }
                            };
                            _serde::__private::Ok(ProductMatchBuyClick {
                                country_code: __field0,
                                created_at: __field1,
                                entity_id: __field2,
                                entity_type: __field3,
                                from_url: __field4,
                                id: __field5,
                                r#type: __field6,
                                url: __field7,
                                user_id: __field8,
                                wish_id: __field9,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "countryCode",
                        "createdAt",
                        "entityId",
                        "entityType",
                        "fromUrl",
                        "id",
                        "type",
                        "url",
                        "userId",
                        "wishId",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductMatchBuyClick",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ProductMatchBuyClick,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductMatchBuyClick {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductMatchBuyClick {
            #[inline]
            fn eq(&self, other: &ProductMatchBuyClick) -> bool {
                self.country_code == other.country_code
                    && self.created_at == other.created_at
                    && self.entity_id == other.entity_id
                    && self.entity_type == other.entity_type
                    && self.from_url == other.from_url && self.id == other.id
                    && self.r#type == other.r#type && self.url == other.url
                    && self.user_id == other.user_id && self.wish_id == other.wish_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductMatchBuyClick {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "country_code",
                    "created_at",
                    "entity_id",
                    "entity_type",
                    "from_url",
                    "id",
                    "type",
                    "url",
                    "user_id",
                    "wish_id",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.country_code,
                    &self.created_at,
                    &self.entity_id,
                    &self.entity_type,
                    &self.from_url,
                    &self.id,
                    &self.r#type,
                    &self.url,
                    &self.user_id,
                    &&self.wish_id,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ProductMatchBuyClick",
                    names,
                    values,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for ProductMatchBuyClick {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductMatchBuyClick")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductMatchBuyClick",
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
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
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
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 2usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "entityType".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 3usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "fromUrl".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 4usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "id".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 5usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "type".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 6usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "url".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 7usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "userId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 8usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "wishId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 9usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("ProductMatchBuyClick")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductMatchBuyClick",
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
    pub use product_match_buy_click::ProductMatchBuyClick;
    pub mod product_search {
        pub struct ProductSearchFilters {}
        #[automatically_derived]
        impl ::core::clone::Clone for ProductSearchFilters {
            #[inline]
            fn clone(&self) -> ProductSearchFilters {
                ProductSearchFilters {}
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductSearchFilters {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductSearchFilters",
                        false as usize,
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
            impl<'de> _serde::Deserialize<'de> for ProductSearchFilters {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
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
                        marker: _serde::__private::PhantomData<ProductSearchFilters>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductSearchFilters;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductSearchFilters",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            _: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            _serde::__private::Ok(ProductSearchFilters {})
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            _serde::__private::Ok(ProductSearchFilters {})
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductSearchFilters",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ProductSearchFilters,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductSearchFilters {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductSearchFilters {
            #[inline]
            fn eq(&self, other: &ProductSearchFilters) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductSearchFilters {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "ProductSearchFilters")
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for ProductSearchFilters {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductSearchFilters")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductSearchFilters",
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
                    let schema_fields = ::alloc::vec::Vec::new();
                    let name = apache_avro::schema::Name::new("ProductSearchFilters")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductSearchFilters",
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
        pub struct ProductSearchMergedProducts {
            #[avro(rename = "id")]
            #[serde(rename = "id")]
            pub id: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductSearchMergedProducts {
            #[inline]
            fn clone(&self) -> ProductSearchMergedProducts {
                ProductSearchMergedProducts {
                    id: ::core::clone::Clone::clone(&self.id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductSearchMergedProducts {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductSearchMergedProducts",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
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
            impl<'de> _serde::Deserialize<'de> for ProductSearchMergedProducts {
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
                                "id" => _serde::__private::Ok(__Field::__field0),
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
                                b"id" => _serde::__private::Ok(__Field::__field0),
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
                        marker: _serde::__private::PhantomData<
                            ProductSearchMergedProducts,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductSearchMergedProducts;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductSearchMergedProducts",
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
                                            &"struct ProductSearchMergedProducts with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductSearchMergedProducts {
                                id: __field0,
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
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            _serde::__private::Ok(ProductSearchMergedProducts {
                                id: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductSearchMergedProducts",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ProductSearchMergedProducts,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductSearchMergedProducts {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductSearchMergedProducts {
            #[inline]
            fn eq(&self, other: &ProductSearchMergedProducts) -> bool {
                self.id == other.id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductSearchMergedProducts {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ProductSearchMergedProducts",
                    "id",
                    &&self.id,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent
        for ProductSearchMergedProducts {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductSearchMergedProducts")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductSearchMergedProducts",
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
                                name: "id".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new(
                            "ProductSearchMergedProducts",
                        )
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductSearchMergedProducts",
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
        pub struct ProductSearchRerankedProducts {
            #[avro(rename = "id")]
            #[serde(rename = "id")]
            pub id: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductSearchRerankedProducts {
            #[inline]
            fn clone(&self) -> ProductSearchRerankedProducts {
                ProductSearchRerankedProducts {
                    id: ::core::clone::Clone::clone(&self.id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductSearchRerankedProducts {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductSearchRerankedProducts",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
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
            impl<'de> _serde::Deserialize<'de> for ProductSearchRerankedProducts {
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
                                "id" => _serde::__private::Ok(__Field::__field0),
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
                                b"id" => _serde::__private::Ok(__Field::__field0),
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
                        marker: _serde::__private::PhantomData<
                            ProductSearchRerankedProducts,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductSearchRerankedProducts;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductSearchRerankedProducts",
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
                                            &"struct ProductSearchRerankedProducts with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductSearchRerankedProducts {
                                id: __field0,
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
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            _serde::__private::Ok(ProductSearchRerankedProducts {
                                id: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductSearchRerankedProducts",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ProductSearchRerankedProducts,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductSearchRerankedProducts {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductSearchRerankedProducts {
            #[inline]
            fn eq(&self, other: &ProductSearchRerankedProducts) -> bool {
                self.id == other.id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductSearchRerankedProducts {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ProductSearchRerankedProducts",
                    "id",
                    &&self.id,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent
        for ProductSearchRerankedProducts {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new(
                        "ProductSearchRerankedProducts",
                    )
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductSearchRerankedProducts",
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
                                name: "id".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new(
                            "ProductSearchRerankedProducts",
                        )
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductSearchRerankedProducts",
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
        pub struct ProductSearchVectorProducts {
            #[avro(rename = "distance")]
            #[serde(rename = "distance")]
            pub distance: f64,
            #[avro(rename = "id")]
            #[serde(rename = "id")]
            pub id: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductSearchVectorProducts {
            #[inline]
            fn clone(&self) -> ProductSearchVectorProducts {
                ProductSearchVectorProducts {
                    distance: ::core::clone::Clone::clone(&self.distance),
                    id: ::core::clone::Clone::clone(&self.id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductSearchVectorProducts {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductSearchVectorProducts",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "distance",
                        &self.distance,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
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
            impl<'de> _serde::Deserialize<'de> for ProductSearchVectorProducts {
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
                                "distance" => _serde::__private::Ok(__Field::__field0),
                                "id" => _serde::__private::Ok(__Field::__field1),
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
                                b"distance" => _serde::__private::Ok(__Field::__field0),
                                b"id" => _serde::__private::Ok(__Field::__field1),
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
                        marker: _serde::__private::PhantomData<
                            ProductSearchVectorProducts,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductSearchVectorProducts;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductSearchVectorProducts",
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
                                f64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ProductSearchVectorProducts with 2 elements",
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
                                            &"struct ProductSearchVectorProducts with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductSearchVectorProducts {
                                distance: __field0,
                                id: __field1,
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
                            let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "distance",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("distance")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            _serde::__private::Ok(ProductSearchVectorProducts {
                                distance: __field0,
                                id: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["distance", "id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductSearchVectorProducts",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ProductSearchVectorProducts,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductSearchVectorProducts {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductSearchVectorProducts {
            #[inline]
            fn eq(&self, other: &ProductSearchVectorProducts) -> bool {
                self.distance == other.distance && self.id == other.id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductSearchVectorProducts {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ProductSearchVectorProducts",
                    "distance",
                    &self.distance,
                    "id",
                    &&self.id,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent
        for ProductSearchVectorProducts {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductSearchVectorProducts")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductSearchVectorProducts",
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
                                name: "distance".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::Double,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "id".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 1usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new(
                            "ProductSearchVectorProducts",
                        )
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductSearchVectorProducts",
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
        pub struct ProductSearch {
            #[avro(rename = "countryCode")]
            #[serde(rename = "countryCode")]
            pub country_code: String,
            #[avro(rename = "embeddingModel")]
            #[serde(rename = "embeddingModel")]
            pub embedding_model: String,
            #[avro(rename = "filters")]
            #[serde(rename = "filters")]
            pub filters: ProductSearchFilters,
            #[avro(rename = "indexId")]
            #[serde(rename = "indexId")]
            pub index_id: String,
            #[avro(rename = "mergedProducts")]
            #[serde(rename = "mergedProducts")]
            pub merged_products: Vec<ProductSearchMergedProducts>,
            #[avro(rename = "rerankVersion")]
            #[serde(rename = "rerankVersion")]
            pub rerank_version: String,
            #[avro(rename = "rerankedProducts")]
            #[serde(rename = "rerankedProducts")]
            pub reranked_products: Vec<ProductSearchRerankedProducts>,
            #[avro(rename = "searchTerm")]
            #[serde(rename = "searchTerm")]
            pub search_term: String,
            #[avro(rename = "vectorProducts")]
            #[serde(rename = "vectorProducts")]
            pub vector_products: Vec<ProductSearchVectorProducts>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductSearch {
            #[inline]
            fn clone(&self) -> ProductSearch {
                ProductSearch {
                    country_code: ::core::clone::Clone::clone(&self.country_code),
                    embedding_model: ::core::clone::Clone::clone(&self.embedding_model),
                    filters: ::core::clone::Clone::clone(&self.filters),
                    index_id: ::core::clone::Clone::clone(&self.index_id),
                    merged_products: ::core::clone::Clone::clone(&self.merged_products),
                    rerank_version: ::core::clone::Clone::clone(&self.rerank_version),
                    reranked_products: ::core::clone::Clone::clone(
                        &self.reranked_products,
                    ),
                    search_term: ::core::clone::Clone::clone(&self.search_term),
                    vector_products: ::core::clone::Clone::clone(&self.vector_products),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductSearch {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductSearch",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "countryCode",
                        &self.country_code,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "embeddingModel",
                        &self.embedding_model,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "filters",
                        &self.filters,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "indexId",
                        &self.index_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "mergedProducts",
                        &self.merged_products,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rerankVersion",
                        &self.rerank_version,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rerankedProducts",
                        &self.reranked_products,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "searchTerm",
                        &self.search_term,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "vectorProducts",
                        &self.vector_products,
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
            impl<'de> _serde::Deserialize<'de> for ProductSearch {
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
                        __field8,
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
                                8u64 => _serde::__private::Ok(__Field::__field8),
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
                                "embeddingModel" => _serde::__private::Ok(__Field::__field1),
                                "filters" => _serde::__private::Ok(__Field::__field2),
                                "indexId" => _serde::__private::Ok(__Field::__field3),
                                "mergedProducts" => _serde::__private::Ok(__Field::__field4),
                                "rerankVersion" => _serde::__private::Ok(__Field::__field5),
                                "rerankedProducts" => {
                                    _serde::__private::Ok(__Field::__field6)
                                }
                                "searchTerm" => _serde::__private::Ok(__Field::__field7),
                                "vectorProducts" => _serde::__private::Ok(__Field::__field8),
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
                                b"embeddingModel" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"filters" => _serde::__private::Ok(__Field::__field2),
                                b"indexId" => _serde::__private::Ok(__Field::__field3),
                                b"mergedProducts" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"rerankVersion" => _serde::__private::Ok(__Field::__field5),
                                b"rerankedProducts" => {
                                    _serde::__private::Ok(__Field::__field6)
                                }
                                b"searchTerm" => _serde::__private::Ok(__Field::__field7),
                                b"vectorProducts" => {
                                    _serde::__private::Ok(__Field::__field8)
                                }
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
                        marker: _serde::__private::PhantomData<ProductSearch>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductSearch;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductSearch",
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
                                            &"struct ProductSearch with 9 elements",
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
                                            &"struct ProductSearch with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                ProductSearchFilters,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ProductSearch with 9 elements",
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
                                            &"struct ProductSearch with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Vec<ProductSearchMergedProducts>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ProductSearch with 9 elements",
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
                                            &"struct ProductSearch with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                Vec<ProductSearchRerankedProducts>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct ProductSearch with 9 elements",
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
                                            &"struct ProductSearch with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                Vec<ProductSearchVectorProducts>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct ProductSearch with 9 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductSearch {
                                country_code: __field0,
                                embedding_model: __field1,
                                filters: __field2,
                                index_id: __field3,
                                merged_products: __field4,
                                rerank_version: __field5,
                                reranked_products: __field6,
                                search_term: __field7,
                                vector_products: __field8,
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
                            let mut __field2: _serde::__private::Option<
                                ProductSearchFilters,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Vec<ProductSearchMergedProducts>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                Vec<ProductSearchRerankedProducts>,
                            > = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<
                                Vec<ProductSearchVectorProducts>,
                            > = _serde::__private::None;
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
                                                    "embeddingModel",
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
                                                    "filters",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ProductSearchFilters,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "indexId",
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
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "mergedProducts",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<ProductSearchMergedProducts>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "rerankVersion",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "rerankedProducts",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<ProductSearchRerankedProducts>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "searchTerm",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "vectorProducts",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<ProductSearchVectorProducts>,
                                            >(&mut __map)?,
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
                                    _serde::__private::de::missing_field("embeddingModel")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("filters")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("indexId")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("mergedProducts")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("rerankVersion")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("rerankedProducts")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("searchTerm")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("vectorProducts")?
                                }
                            };
                            _serde::__private::Ok(ProductSearch {
                                country_code: __field0,
                                embedding_model: __field1,
                                filters: __field2,
                                index_id: __field3,
                                merged_products: __field4,
                                rerank_version: __field5,
                                reranked_products: __field6,
                                search_term: __field7,
                                vector_products: __field8,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "countryCode",
                        "embeddingModel",
                        "filters",
                        "indexId",
                        "mergedProducts",
                        "rerankVersion",
                        "rerankedProducts",
                        "searchTerm",
                        "vectorProducts",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductSearch",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ProductSearch>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductSearch {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductSearch {
            #[inline]
            fn eq(&self, other: &ProductSearch) -> bool {
                self.country_code == other.country_code
                    && self.embedding_model == other.embedding_model
                    && self.filters == other.filters && self.index_id == other.index_id
                    && self.merged_products == other.merged_products
                    && self.rerank_version == other.rerank_version
                    && self.reranked_products == other.reranked_products
                    && self.search_term == other.search_term
                    && self.vector_products == other.vector_products
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductSearch {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "country_code",
                    "embedding_model",
                    "filters",
                    "index_id",
                    "merged_products",
                    "rerank_version",
                    "reranked_products",
                    "search_term",
                    "vector_products",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.country_code,
                    &self.embedding_model,
                    &self.filters,
                    &self.index_id,
                    &self.merged_products,
                    &self.rerank_version,
                    &self.reranked_products,
                    &self.search_term,
                    &&self.vector_products,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ProductSearch",
                    names,
                    values,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for ProductSearch {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductSearch")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductSearch",
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
                                name: "embeddingModel".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 1usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "filters".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <ProductSearchFilters as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 2usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "indexId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 3usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "mergedProducts".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Vec<
                                    ProductSearchMergedProducts,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 4usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "rerankVersion".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 5usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "rerankedProducts".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Vec<
                                    ProductSearchRerankedProducts,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 6usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "searchTerm".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 7usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "vectorProducts".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: <Vec<
                                    ProductSearchVectorProducts,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 8usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("ProductSearch")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductSearch",
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
    pub use product_search::ProductSearch;
    pub mod product_updated {
        pub struct ProductUpdated {
            #[avro(rename = "notificationType")]
            #[serde(rename = "notificationType")]
            pub notification_type: String,
            #[avro(rename = "productId")]
            #[serde(rename = "productId")]
            pub product_id: String,
            #[avro(rename = "timestamp")]
            #[serde(rename = "timestamp")]
            pub timestamp: String,
            #[avro(rename = "userId")]
            #[serde(rename = "userId")]
            pub user_id: String,
            #[avro(rename = "wishId")]
            #[serde(rename = "wishId")]
            pub wish_id: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductUpdated {
            #[inline]
            fn clone(&self) -> ProductUpdated {
                ProductUpdated {
                    notification_type: ::core::clone::Clone::clone(
                        &self.notification_type,
                    ),
                    product_id: ::core::clone::Clone::clone(&self.product_id),
                    timestamp: ::core::clone::Clone::clone(&self.timestamp),
                    user_id: ::core::clone::Clone::clone(&self.user_id),
                    wish_id: ::core::clone::Clone::clone(&self.wish_id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductUpdated {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductUpdated",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "notificationType",
                        &self.notification_type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "productId",
                        &self.product_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "timestamp",
                        &self.timestamp,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "userId",
                        &self.user_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "wishId",
                        &self.wish_id,
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
            impl<'de> _serde::Deserialize<'de> for ProductUpdated {
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
                                "notificationType" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "productId" => _serde::__private::Ok(__Field::__field1),
                                "timestamp" => _serde::__private::Ok(__Field::__field2),
                                "userId" => _serde::__private::Ok(__Field::__field3),
                                "wishId" => _serde::__private::Ok(__Field::__field4),
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
                                b"notificationType" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"productId" => _serde::__private::Ok(__Field::__field1),
                                b"timestamp" => _serde::__private::Ok(__Field::__field2),
                                b"userId" => _serde::__private::Ok(__Field::__field3),
                                b"wishId" => _serde::__private::Ok(__Field::__field4),
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
                        marker: _serde::__private::PhantomData<ProductUpdated>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductUpdated;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductUpdated",
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
                                            &"struct ProductUpdated with 5 elements",
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
                                            &"struct ProductUpdated with 5 elements",
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
                                            &"struct ProductUpdated with 5 elements",
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
                                            &"struct ProductUpdated with 5 elements",
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
                                            &"struct ProductUpdated with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductUpdated {
                                notification_type: __field0,
                                product_id: __field1,
                                timestamp: __field2,
                                user_id: __field3,
                                wish_id: __field4,
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
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "notificationType",
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
                                                    "productId",
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
                                                    "timestamp",
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
                                                <__A::Error as _serde::de::Error>::duplicate_field("userId"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("wishId"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("notificationType")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("productId")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("timestamp")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("userId")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("wishId")?
                                }
                            };
                            _serde::__private::Ok(ProductUpdated {
                                notification_type: __field0,
                                product_id: __field1,
                                timestamp: __field2,
                                user_id: __field3,
                                wish_id: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "notificationType",
                        "productId",
                        "timestamp",
                        "userId",
                        "wishId",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductUpdated",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ProductUpdated>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProductUpdated {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProductUpdated {
            #[inline]
            fn eq(&self, other: &ProductUpdated) -> bool {
                self.notification_type == other.notification_type
                    && self.product_id == other.product_id
                    && self.timestamp == other.timestamp && self.user_id == other.user_id
                    && self.wish_id == other.wish_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductUpdated {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ProductUpdated",
                    "notification_type",
                    &self.notification_type,
                    "product_id",
                    &self.product_id,
                    "timestamp",
                    &self.timestamp,
                    "user_id",
                    &self.user_id,
                    "wish_id",
                    &&self.wish_id,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for ProductUpdated {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("ProductUpdated")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "ProductUpdated",
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
                                name: "notificationType".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 0usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "productId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 1usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "timestamp".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 2usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "userId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 3usize,
                                custom_attributes: Default::default(),
                            },
                            apache_avro::schema::RecordField {
                                name: "wishId".to_string(),
                                doc: None,
                                default: None,
                                aliases: None,
                                schema: apache_avro::schema::Schema::String,
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 4usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("ProductUpdated")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "ProductUpdated",
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
    pub use product_updated::ProductUpdated;
    pub mod promotion_click {
        pub struct PromotionClick {
            #[avro(rename = "countryCode")]
            #[serde(rename = "countryCode")]
            pub country_code: Option<String>,
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
            pub user_id: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PromotionClick {
            #[inline]
            fn clone(&self) -> PromotionClick {
                PromotionClick {
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
            impl _serde::Serialize for PromotionClick {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "PromotionClick",
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
            impl<'de> _serde::Deserialize<'de> for PromotionClick {
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
                        marker: _serde::__private::PhantomData<PromotionClick>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PromotionClick;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct PromotionClick",
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
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct PromotionClick with 8 elements",
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
                                            &"struct PromotionClick with 8 elements",
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
                                            &"struct PromotionClick with 8 elements",
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
                                            &"struct PromotionClick with 8 elements",
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
                                            &"struct PromotionClick with 8 elements",
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
                                            &"struct PromotionClick with 8 elements",
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
                                            &"struct PromotionClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct PromotionClick with 8 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(PromotionClick {
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
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
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
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
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
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
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
                            _serde::__private::Ok(PromotionClick {
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
                        "PromotionClick",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PromotionClick>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PromotionClick {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PromotionClick {
            #[inline]
            fn eq(&self, other: &PromotionClick) -> bool {
                self.country_code == other.country_code
                    && self.created_at == other.created_at
                    && self.entity_id == other.entity_id
                    && self.entity_type == other.entity_type && self.id == other.id
                    && self.r#type == other.r#type && self.url == other.url
                    && self.user_id == other.user_id
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PromotionClick {
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
                    "PromotionClick",
                    names,
                    values,
                )
            }
        }
        impl apache_avro::schema::derive::AvroSchemaComponent for PromotionClick {
            fn get_schema_in_ctxt(
                named_schemas: &mut std::collections::HashMap<
                    apache_avro::schema::Name,
                    apache_avro::schema::Schema,
                >,
                enclosing_namespace: &Option<String>,
            ) -> apache_avro::schema::Schema {
                let name = apache_avro::schema::Name::new("PromotionClick")
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unable to parse schema name {0}",
                                    "PromotionClick",
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
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
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
                                schema: <Option<
                                    String,
                                > as apache_avro::schema::derive::AvroSchemaComponent>::get_schema_in_ctxt(
                                    named_schemas,
                                    enclosing_namespace,
                                ),
                                order: apache_avro::schema::RecordFieldOrder::Ascending,
                                position: 7usize,
                                custom_attributes: Default::default(),
                            },
                        ]),
                    );
                    let name = apache_avro::schema::Name::new("PromotionClick")
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Unable to parse struct name for schema {0}",
                                        "PromotionClick",
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
    pub use promotion_click::PromotionClick;
}
fn main() {
    {
        ::std::io::_print(format_args!("OK\n"));
    };
}
