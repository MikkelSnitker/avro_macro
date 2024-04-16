use apache_avro::AvroSchema;
use avro_macro::schema;
use avro_macro::load_schema;


#[schema("test.avro")]
mod Test {

   
}

pub fn process(a: Test::Employee){
   println!("{}", a.department.id);

}
fn main() {
  //  process(Blah::Hmm{ test: todo!(), foo: todo!() });
  
  let schema = Test::Employee::get_schema();
  println!("{}", schema.canonical_form());
}

