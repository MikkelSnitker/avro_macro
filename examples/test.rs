use std::{
    fmt::format, io::{Read, Write}, time
};

use apache_avro::{schema::{self, UnionSchema}, types::Value, AvroSchema};
use avro_macro::{create_events, schema, TaggedEnum};
use serde::{de::Error, Serialize};


#[avro_macro::schema("./gowish_schema/base/*.avro")]
mod Test {}

create_events!(WishEvents {
  WishCreated(Test::WishCreated),

});


create_events!(TrackingEvents {
  Tracking(Test::Tracking),
  TrackingV2(Test::TrackingV2),
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
 
 
Ok(())
 
}

