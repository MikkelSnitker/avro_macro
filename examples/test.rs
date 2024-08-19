use std::{
    fmt::format, io::{Read, Write}, time
};

use apache_avro::{schema, AvroSchema};
use avro_macro::{schema, TaggedEnum};
use serde::{de::{self, value}, ser::{self, SerializeSeq, SerializeTupleVariant}, Deserialize, Serialize};


#[avro_macro::schema("./gowish_schema/base/*.avro")]
mod Test {}


fn main() {
  
}