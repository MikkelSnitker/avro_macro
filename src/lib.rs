use std::fs;
use std::str::FromStr;

//use proc_macro::{Span, TokenStream};

use apache_avro::Schema;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, Field, Ident, Item, ItemEnum, ItemStruct, Type};

fn get_type(schema: &apache_avro::Schema, parent: Option<apache_avro::Schema>, items: &mut Vec<Item>) -> TokenStream {
    
    match schema.clone() {
       
        apache_avro::Schema::Record(record) => {
            let name = Ident::new(record.name.name.as_str(), Span::call_site());
            
            let mut item_struct =  syn::parse2::<ItemStruct>(quote! { 
                #[derive(Clone, Serialize, Deserialize, PartialEq, Debug, AvroSchema)]
                pub struct #name {}}).unwrap();
            if let syn::Fields::Named(ref mut fields) = item_struct.fields  {
                for field in record.fields {
                    let field_name =field.name.as_str();
                    let field_name_sc =  Ident::new(snake(field_name).as_str(), Span::call_site());
                    let field_type = get_type(&field.schema, Some(schema.clone()), items);    
                    fields.named.push( Field::parse_named.parse2(
                        quote! {
                       #[avro(rename = #field_name )]
                       #[serde(rename = #field_name)]
                       pub #field_name_sc: #field_type
                    }).unwrap());
                }
            }



            items.push(Item::Struct(item_struct));
            
            TokenStream::from_str(record.name.name.as_str()).unwrap()
            

            


        },
        apache_avro::Schema::Union(s) => {
            
            let nullable = s.is_nullable() ;//.variants().iter().any(|&x| x == apache_avro::Schema::Null );
            let variants = s.variants().iter().filter(|&x| *x != apache_avro::Schema::Null  ).collect::<Vec<&Schema>>();
            if variants.len() == 1 {
                let name =  get_type(variants.first().unwrap(),parent, items);
               return if nullable {
                let t = syn::parse2::<Type>(name).unwrap();
                    quote! { Option<#t>}
               } else {
                    name
                }
            }
            
            let item_enum = syn::parse2::<ItemEnum>(quote! { pub enum Name {}}).unwrap();
            items.push(Item::Enum(item_enum));
            if nullable {
                TokenStream::from_str("Option<i32>").unwrap()    
            } else {
                TokenStream::from_str("i32").unwrap()
            }
            
    
        },
        apache_avro::Schema::Int => quote! { i64 },
        apache_avro::Schema::Boolean => quote! { bool },
        apache_avro::Schema::Decimal(_) => quote! { f64 },
        apache_avro::Schema::Bytes => quote! { &[u8] },
        apache_avro::Schema::Float => quote! { f64 },
        apache_avro::Schema::Long => quote! { i64 },
        apache_avro::Schema::String => quote! { String },
        apache_avro::Schema::Map(m) => {
            let t = get_type(m.as_ref(), parent, items);

            return quote! {  std::collection::HashMap<String, #t> }.into()
        },
        apache_avro::Schema::Null => quote! { None },
        apache_avro::Schema::Double => quote! { f32},
        apache_avro::Schema::Array(a) => {
            let t = get_type(a.as_ref(), parent, items);
            let t = syn::parse2::<Type>(t).unwrap();
            quote! { Vec<#t> }
        },
        e => {
            panic!(r#"Unsupported type "{:?}""#, e.name())
        }
    }
}


#[proc_macro]
pub fn load_schema(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let module = quote! { pub mod Schema {} };
    schema(item, module.into())
}


#[proc_macro_attribute]
pub fn schema(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item_mod = parse_macro_input!(input as syn::ItemMod);
    let p = parse_macro_input!(args as syn::LitStr);
    
    let mut file = fs::File::open(p.value()).unwrap();
    let schema = apache_avro::Schema::parse_reader(&mut file).unwrap();
  
    if let Some((b,  mut items)) = item_mod.content {
        let uses = quote! {
            use serde::{Deserialize, Serialize};
            use apache_avro::AvroSchema;
         };
        
        items.push(Item::Verbatim(uses));
        let _ =  get_type(&schema, None, &mut items);
        item_mod.content = Some((b, items));
    }
   
    
    

    return quote! {
        #item_mod
    }
    .into();
}


fn snake(input: &str) -> String {
    let mut out = String::default();
    let mut chars = input.chars();
    out.push(chars.next().unwrap().to_ascii_lowercase());
    while let Some(ch) = chars.next() {
        match ch.is_ascii_uppercase() {
            true => {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            }
            false => out.push(ch),
        }
    }
    out
}