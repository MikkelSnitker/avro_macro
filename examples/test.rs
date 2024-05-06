use apache_avro::AvroSchema;
use avro_macro::{schema};


#[schema("*.avro", exclude = ["eventName"] )]
mod Test {

  
}

/* 
pub fn process(a: Test::WishBuyClick){
    a.payload.country_code
}*/
fn main() {
    
    let schema = Test::WishlistWishRemoved::get_schema();
    
    println!("{}", schema.canonical_form());
  //  process(Blah::Hmm{ test: todo!(), foo: todo!() });
 /*Test::CountryCode::get_schema();
  let schema = Test::Employee::get_schema();
  println!("{}", schema.canonical_form());
*/

}

