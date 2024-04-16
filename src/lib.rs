use std::fs;
use std::str::FromStr;

//use proc_macro::{Span, TokenStream};

use apache_avro::schema::Name;
use apache_avro::Schema;
use proc_macro::LexError;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::{parse2, parse_macro_input, Field, Ident, Item, ItemEnum, ItemImpl, ItemStruct, Type, Variant};


#[derive(Debug, Clone)]
struct  Error {

}

impl From<LexError> for Error {
    fn from(error: LexError) -> Self {
        
        Error {}
    }
}


impl From<proc_macro2::LexError> for Error {
    fn from(error: proc_macro2::LexError) -> Self {
        
        Error {}
    }
}

impl  From<apache_avro::Error> for Error {
    fn from(error: apache_avro::Error) -> Self {
        
        Error {}
    }
}




impl From<syn::Error> for Error {
    fn from(error: syn::Error) -> Self {
        
        Error {}
    }
}




fn get_type(schema: &apache_avro::Schema, parent: Option<&apache_avro::Schema>, items: &mut Vec<Item>) -> std::result::Result<TokenStream,Error> {
    
    match schema.clone() {
       
        apache_avro::Schema::Record(record) => {
            let name = Ident::new(record.name.name.as_str(), Span::call_site());
          
            let mut item_struct =  syn::parse2::<ItemStruct>(quote! { 
                #[derive(Clone, Serialize, Deserialize, PartialEq, Debug, apache_avro::AvroSchema )]
                pub struct #name {}
            })?;
            if let syn::Fields::Named(ref mut fields) = item_struct.fields  {
                for field in record.fields {
                    let field_name =field.name.as_str();
                    
                    let field_name_sc =  Ident::new_raw(snake(field_name).as_str(), Span::call_site());
            
                    match  get_type(&field.schema, Some(&apache_avro::Schema::Ref {name: Name::new(field_name)? } ), items) {
                        Ok(field_type) => 
                        {
                            match Field::parse_named.parse2(
                                quote! {
                               #[avro(rename = #field_name )]
                               #[serde(rename = #field_name)]
                               pub #field_name_sc: #field_type
                            }) {
                                Ok(field) => fields.named.push(field),
                                Err(e) => {
                                    return Err(e.into());
                                }
                            }
                            
                        },

                        Err(e) => {
                            return Err(e);
                        }
                    };
                }
            }



            items.push(Item::Struct(item_struct));
            
            
           Ok(TokenStream::from_str(record.name.name.as_str())?)
          

        },
        apache_avro::Schema::Union(s) => {
            
            let nullable = s.is_nullable() ;//.variants().iter().any(|&x| x == apache_avro::Schema::Null );
            let variants = s.variants().iter().filter(|&x| *x != apache_avro::Schema::Null  ).collect::<Vec<&Schema>>();
            if variants.len() == 1 {
                let name =  get_type(variants.first().unwrap(),parent, items)?;
               return if nullable {
                    let t = syn::parse2::<Type>(name)?;
                    Ok(quote! { Option<#t>})
               } else {
                    Ok(name)
                }
            }
            
            
            let name = match parent {
                Some(parent) => Ident::new(capitalize(parent.name().unwrap().name.clone()).as_str(), Span::call_site()),
                None => Ident::new("TEST", Span::call_site()),
            };

            let mut item_enum: ItemEnum = syn::parse2::<ItemEnum>(quote! { 
                #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
                pub enum #name {}
            })?;
            
            //apache_avro::schema::derive::AvroSchemaComponent
            let imp = syn::parse2::<ItemImpl>(quote! {
                impl apache_avro::schema::derive::AvroSchemaComponent for #name {
                    fn get_schema_in_ctxt(named_schemas: &mut std::collections::HashMap<apache_avro::schema::Name, apache_avro::Schema>, enclosing_namespace: &apache_avro::schema::Namespace) -> apache_avro::Schema {
                        todo!()
                    }
                }
            })?;
            
            items.push(Item::Impl(imp));

            for variant in variants {
                let test = match variant {
                    Schema::Null => quote! { Null },
                    Schema::Boolean => quote! { Boolean(bool) },
                    Schema::Int => quote! { Int(i32) },
                    Schema::Long => quote! { Int(i64) },
                    Schema::Float => quote! { Float(i32) },
                    Schema::Double => quote! { Float(i64) },
                    Schema::Bytes => todo!(),
                    Schema::String => quote! { String(String )},
                    Schema::Array(schema) => {
                        
                        let ident = parse2::<Ident>(get_type(schema, parent, items)?)?;
                        quote! { Array(#ident)}
                    },
                    Schema::Map(_) => todo!(),
                    Schema::Union(_) => todo!(),
                    Schema::Record(_) => todo!(),
                    Schema::Enum(_) => todo!(),
                    Schema::Fixed(_) => todo!(),
                    Schema::Decimal(_) => todo!(),
                    Schema::Uuid => todo!(),
                    Schema::Date => todo!(),
                    Schema::TimeMillis => todo!(),
                    Schema::TimeMicros => todo!(),
                    Schema::TimestampMillis => todo!(),
                    Schema::TimestampMicros => todo!(),
                    Schema::LocalTimestampMillis => todo!(),
                    Schema::LocalTimestampMicros => todo!(),
                    Schema::Duration => todo!(),
                    Schema::Ref { name } => todo!(),
                };
                let v = syn::parse2::<Variant>(test)?;
                item_enum.variants.push(v);
            }
          
            let name = item_enum.ident.clone();

            items.push(Item::Enum(item_enum));
            
            if nullable {
                Ok(quote! { Option<#name> })
            } else {
                Ok(quote! { #name })
            }
            
    
        },
        apache_avro::Schema::Int => Ok(quote! { i64 }),
        apache_avro::Schema::Boolean => Ok(quote! { bool }),
        apache_avro::Schema::Decimal(_) => Ok(quote! { f64 }),
        apache_avro::Schema::Bytes => Ok(quote! { &[u8] }),
        apache_avro::Schema::Float => Ok(quote! { f64 }),
        apache_avro::Schema::Long => Ok(quote! { i64 }),
        apache_avro::Schema::String => Ok(quote! { String }),
        apache_avro::Schema::Map(m) => {
            let t = get_type(m.as_ref(), parent, items)?;

            return Ok(quote! {  std::collection::HashMap<String, #t> })
        },
        apache_avro::Schema::Null => Ok(quote! { None }),
        apache_avro::Schema::Double =>Ok(quote! { f32}),
        apache_avro::Schema::Array(a) => {
            let t = get_type(a.as_ref(), parent, items)?;
            let t = syn::parse2::<Type>(t)?;
            Ok(quote! { Vec<#t> })
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
    
  
    if let Some((b,  mut items)) = item_mod.content {
        let uses = quote! {
            use serde::{Deserialize, Serialize};
            /*use apache_avro::AvroSchema;
            use apache_avro::schema::derive::AvroSchemaComponent;*/
         };
         items.push(Item::Verbatim(uses));

        for entry in  glob::glob(p.value().as_str()).expect("INVALID PATTERN") {
            match entry {
                Ok(path) => {
                    let mut file = fs::File::open(path.clone()).unwrap();
                    let schema = apache_avro::Schema::parse_reader(&mut file).unwrap();
                    match get_type(&schema, None, &mut items) {
                        Ok(_) => {},
                        Err(e) => panic!("ERROR PARSING FILE {:?}", path)
                    }

                },
                Err(e) => {}
            }
        }
 
        item_mod.content = Some((b, items));
    }

    return quote! {
        #item_mod
    }
    .into();
}



fn snake(input: impl Into<String>) -> String {
    let mut out = String::default();
    let input = input.into();
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

fn capitalize(input: impl Into<String>) -> String {
    let s = input.into();
    let mut c = s.chars();
  
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}