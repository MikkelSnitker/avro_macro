use apache_avro::AvroSchema;
use avro_macro::schema;
use avro_macro::load_schema;


#[schema("test.avro")]
mod Test {}

fn main() {
   let schema = Test::Employee::get_schema();
   
    println!("{}", schema.canonical_form());


}

