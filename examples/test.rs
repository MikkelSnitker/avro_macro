use apache_avro::AvroSchema;
use avro_macro::schema;
use avro_macro::load_schema;



#[schema("*.avro")]
mod Test {

   
}
/* 
pub fn process(a: Test::WishBuyClick){
    a.payload.country_code
}*/
fn main() {
    let schema =Test::foo::CountryCode::get_schema();
    
    println!("{}", schema.canonical_form());
  //  process(Blah::Hmm{ test: todo!(), foo: todo!() });
 /*Test::CountryCode::get_schema();
  let schema = Test::Employee::get_schema();
  println!("{}", schema.canonical_form());
*/
let a = mod1::struct1 {
    f: Some(mod1::mod2::Foo::A)
};

}


enum TestEnum {
    String(String),
    Vec(String)
}


pub mod mod1 {
    pub use mod2::struct1;
    pub mod mod2 {
       pub enum  Foo {
            A,
            B
        }
        pub struct struct1 {
            pub f: Option<Foo>
        }
    }
}