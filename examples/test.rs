use std::{io::{Read, Write}, time};

use apache_avro::AvroSchema;
use avro_macro::{schema};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[schema("*.avro", exclude = ["eventName"] )]
mod Test {

  
}

struct Base64Writer {
  buf: Vec<u8>
}

impl std::io::Write for Base64Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.append(&mut buf.to_vec());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
         std::io::stdout().write_all( BASE64_STANDARD.encode(self.buf.as_slice()).as_bytes())

    }
}

/* 
pub fn process(a: Test::WishBuyClick){
    a.payload.country_code
}*/

#[derive(Serialize, AvroSchema)]
struct Payload {
  
  schema: String,
  schemaType: String,
}

fn insertAvroSchema(){
  let user_created_schema = Test::UserCreated::get_schema();
  let user_updated_schema =Test::UserUpdated::get_schema();
  
  let client = reqwest::blocking::Client::new();


  println!("{}", user_created_schema.canonical_form());
  println!("###");

let body = serde_json::to_string(&Payload {
 schemaType: "AVRO".to_string(),
 schema: user_created_schema.canonical_form(),
}).unwrap();
  let mut res = client.post("http://localhost:8080/subjects/msn_schema_test-com.messages.events.user.UserCreated/versions")
  .body(body.clone())
  .send().unwrap();

  println!("{}", body);
let mut buf: String = String::new();
  res.read_to_string(&mut buf);
  println!("{}", buf);
}

#[derive(Serialize, Deserialize, AvroSchema)]
struct EventOne {
  title: String,
  id: String,
}

#[derive(Serialize, Deserialize, AvroSchema)]
struct EventTwo {
  age: u8,
  id: String
}


#[derive(Serialize, Deserialize, AvroSchema)]
struct  Foo {
  event1: EventOne,
  event2: EventTwo,
}


#[derive(Serialize, Deserialize)]
enum Events {
  EventOne(EventOne),
  EventTwo(EventTwo),
}

fn main() {

  let schema = Foo::get_schema();

  let data = Foo {
    event1: EventOne {
      id: "user-1".to_string(),
      title: "Hello World".to_string(),
    },
    event2: EventTwo {
      id: "user-2".to_string(),
      age: 38

    }
  };

  let a = apache_avro::to_avro_datum(&schema, apache_avro::to_value(data).unwrap());

  println!("{}",  schema.canonical_form());
  println!("{:?}", a);

/*
    
    //let schema = Test::get_schema();
   */





  //  let key = "UserCreated/test-1".to_string();
/*
    let key = apache_avro::to_value(key)?;
    let key = apache_avro::to_avro_datum_schemata(&key_schema, vec![], key)?;
    */
    /*
    let value = apache_avro::to_value(Test::UserCreated {
      id: "mikkel@snitker.com".to_string(),
      first_name: "Mikkel".to_string(),
      last_name: "Snitker".to_string(),
      email_id: "mikkel@snitker.com".to_string(),   
     }).unwrap();

    let value = apache_avro::to_avro_datum_schemata(&user_created_schema, vec![],value).unwrap();

    println!("{:?}", BASE64_STANDARD.encode(value));

*/
/*
    let value = apache_avro::to_value(Test::UserUpdated {
      id: "mikkel@snitker.com".to_string(),
      first_name: "Kirk".to_string(),
      last_name: "Sweeney".to_string(),
      updated_at: 1715066778,
      email: "kirk@sweeney.com".to_string(),
     }).unwrap();

     let mut w = Base64Writer {buf: vec![]} ;
     let mut writer = apache_avro::Writer::new(&user_updated_schema,&mut w);
     let _ = writer.append(value);
     let _ = writer.flush();
     
     w.flush();
      */
    //let value = apache_avro::to_avro_datum_schemata(&user_updated_schema, vec![],value).unwrap();

  //  println!("{:?}", BASE64_STANDARD.encode(value));
  //  process(Blah::Hmm{ test: todo!(), foo: todo!() });
 /*Test::CountryCode::get_schema();
  let schema = Test::Employee::get_schema();
  println!("{}", schema.canonical_form());
*/

}


