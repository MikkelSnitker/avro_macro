use std::{
    fmt::format, io::{Read, Write}, time
};

use apache_avro::{schema, AvroSchema};
use avro_macro::{schema, TaggedEnum};
use base64::prelude::*;
use serde::{de::{self, value}, ser::{self, SerializeSeq, SerializeTupleVariant}, Deserialize, Serialize};

#[avro_macro::schema("./Test.avro")]
mod Test {}

#[derive(Debug)]
#[derive(Deserialize)]

#[derive(Serialize)]
#[avro_macro::foobar]
//#[serde(untagged)]
enum Foo {
    String(String),
    Int(i8),
    Array(Vec<String>),
}
/*
impl serde::Serialize for Foo {
    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    
    where

        S: serde::Serializer {
            serializer.serialize_newtype_variant("Foo", 0, "String", "Hello")
        //serializer.serialize_unit_variant("FOO", 0, &"Foobar")
        
    }
}
 */
impl apache_avro::schema::derive::AvroSchemaComponent for Foo {
    fn get_schema_in_ctxt(named_schemas: &mut std::collections::HashMap<apache_avro::schema::Name, apache_avro::Schema>, enclosing_namespace: &apache_avro::schema::Namespace) -> apache_avro::Schema {
   
      apache_avro::schema::Schema::parse_str(r#"
      [
            "string",
            "int",
            {
                "type": "array",
                "items": "string"
            }
        ]
    
      "#).unwrap()

    }
}

#[derive(Serialize, Deserialize, Debug, AvroSchema)]
struct Test1 {
    age: u8,
    hmm: Foo,
}

#[derive(Serialize, AvroSchema)]
struct Test2 {
    lines: Vec<String>
}

#[derive(Debug)]
#[derive(Deserialize)]
//#[derive(AvroSchema)]
#[derive(Serialize)]
enum EnumTest1 {
    string(String),
    int(i8),
    array(Vec<u8>),
    

}

fn main1() {

    let schema = apache_avro::schema::Schema::parse_str(r#"["string","int", {"type": "array", "items": "int"}]"#).unwrap();

    println!("{:?}", schema);
    let a =apache_avro::types::Value::Union(1,Box::new(apache_avro::types::Value::Int(1)));
    let a =apache_avro::types::Value::Union(0,Box::new(apache_avro::types::Value::String("Hello World!".into())));
    let a =apache_avro::types::Value::Union(2,Box::new(apache_avro::types::Value::Array(vec![apache_avro::types::Value::Int(13)]) ));
    let a = apache_avro::to_value( EnumTest1::array(vec![12])).unwrap();

    println!("{:?}", &a);
    let mut b = apache_avro::to_avro_datum(&schema, a).unwrap();
    let a = apache_avro::from_avro_datum(&schema, &mut b.as_slice(), None).unwrap();
    let b = apache_avro::from_value::<EnumTest1>(&a).unwrap();
    println!("{:?}", &a);
    
    return;

    
    let schema = Test1::get_schema();

    println!("Schema\n{}", schema.canonical_form());

    let obj = Test1 {
        age: 1,
        hmm: Foo::Array(vec!["TEST".to_string()]),
      //  hmm: Foo::String("TEST".to_string())
  //  hmm: Foo::Int(123)
};

    println!("obj: {:?}", obj);
    
    let obj_json = serde_json::to_string(&obj).unwrap();
    println!("obj_json: {:?}", obj_json);
    let obj = serde_json::from_str::<Test1>(&obj_json).unwrap();
    println!("obj: {:?}", obj);
  //  let avro_value = apache_avro::to_value(obj).unwrap();
    /*
  
    
    
  //  
    
     */

    let avro_value = apache_avro::to_value(obj).unwrap();
    println!("avro_value: {:?}", avro_value);
/*
    let avro_value = apache_avro::types::Value::Record(vec![
        ("age".into(), apache_avro::types::Value::Int(4)),
        ("hmm".into(), apache_avro::types::Value::Union(0, Box::new(apache_avro::types::Value::String("fobar".into())))),
    ]);
 
    println!("avro_value: {:?}", avro_value);
 */
    let obj_avro = apache_avro::to_avro_datum(&schema, avro_value).unwrap();
    println!("obj_avro: {:?}", obj_avro);

    //let avro_value = apache_avro::to_value(obj_avro).unwrap();

    let obj_value = apache_avro::from_avro_datum(&schema, &mut obj_avro.as_slice(), None).unwrap();
    println!("obj_value: {:?}", obj_value);

    let obj = apache_avro::from_value::<Test1>(&obj_value).unwrap();
    println!("obj: {:?}", obj);
    
   
    /*
    let a = serde_json::from_str::<Test::WishCreated>(json);
    println!("{:?}", a);
    let b = a.unwrap().photos;
     */
}




fn main() {
    let json = r#"
    {
        "title": "Få Street food derhjemme af Katrine Klinken som Hæftet bog på dansk - 9788740058437",
        "currency": "DKK",
        "description": null,
        "index": -14,
        "price": 0,
        "photos": [
            "https://s.onskeskyen.dk/uploads/images/wish/c2/f9/7a/f4/9b/ba/c2f97af49bba.jpeg"
        ],
        "id": "v4et1tBnoqLOfDbA",
        "userId": "wV7RDQipYLf7a7NI",
        "wishlistId": "PDHZnhF8KRouKzWa",
        "url": "https://www.saxo.com/dk/street-food-derhjemme_katrine-klinken_haeftet_9788740058437?dfw_tracker=13098-78452492&gclid=CjwKCAiAuoqABhAsEiwAdSkVVMnXYO8ArHlA-E8nlIbPB_Br4dEbyY29ytCzsjG90Iv9h8FkQPW78RoC0EQQAvD_BwE",
        "createdAt": "2021-01-16T14:29:20.000Z",
        "quantity": 1,
        "productRef": {
            "id": "c2F4by5jb20vZGsvc3RyZWV0LWZvb2QtZGVyaGplbW1lX2thdHJpbmUta2xpbmtlbl9oYWVmdGV0Xzk3ODg3NDAwNTg0Mzd7aWQ6OnYxfURL",
            "uurl": "saxo.com/dk/street-food-derhjemme_katrine-klinken_haeftet_9788740058437",
            "countryCode": "DK",
            "originalUrl": "https://www.saxo.com/dk/street-food-derhjemme_katrine-klinken_haeftet_9788740058437?dfw_tracker=13098-78452492&gclid=CjwKCAiAuoqABhAsEiwAdSkVVMnXYO8ArHlA-E8nlIbPB_Br4dEbyY29ytCzsjG90Iv9h8FkQPW78RoC0EQQAvD_BwE",
            "domainName": "saxo.com",
            "createdAt": "2023-02-17T01:55:34.057Z",
            "updatedAt": "2023-02-17T01:55:34.057Z"
        }
    }
    "#;

    let data = serde_json::from_str::<Test::WishCreated>(json).unwrap();

    let schema = Test::WishCreated::get_schema();
    /*let data = Test::WishCreated {
        created_at: "2023".into(),
        currency: "DKK".into(),
        description: Some("".into()),
        id: "".into(),
        index: 1.0,
        metadata: None,
        photos: Test::test::Photos::Array(vec![]),
        price: 1337f64,
        product_ref: None,
        quantity: 1.0,
        title: "".into(),
        url: Some("".into()),
        user_id: "213".into(),
        wishlist_id: "123".into(),
    };*/

    let data = apache_avro::to_value(data).unwrap();
    let mut data = apache_avro::to_avro_datum(&schema, data).unwrap();
    let data = apache_avro::from_avro_datum(&schema,&mut data.as_slice(),None).unwrap();
    let data = apache_avro::from_value::<Test::WishCreated>(&data).unwrap();
    print!("Data {:?}", data);
}