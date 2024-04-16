use apache_avro::AvroSchema;
use avro_macro::schema;
use avro_macro::load_schema;
use Test::CountryCode;


#[schema("*.avro")]
mod Test {

   
}

pub fn process(a: Test::WishBuyClick){
    
}
fn main() {
  //  process(Blah::Hmm{ test: todo!(), foo: todo!() });
 /*Test::CountryCode::get_schema();
  let schema = Test::Employee::get_schema();
  println!("{}", schema.canonical_form());
*/

}


enum TestEnum {
    String(String),
    Vec(String)
}
