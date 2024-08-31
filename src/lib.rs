
use std::fs;
use std::path::Path;
use std::str::FromStr;

use apache_avro::schema::{Name, UnionSchema};
use apache_avro::Schema;
use proc_macro::{LexError, TokenTree};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Enum, Pub};
use syn::{parse2, parse_macro_input, Field, Fields, FieldsUnnamed, Generics, Ident, Item, ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemUse, LitStr, Token, Type, UseTree, Variant};

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





struct AvroInput {
    pub schema: String,
    pub exclude: Vec<String>,
}

impl Parse for AvroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    
        let mut exclude = vec![];
        let path = input.parse::<syn::LitStr>()?;

        while  input.peek(syn::Token![,]) {
            let _: syn::Token![,] = input.parse()?;
            if let Ok(ident) =  input.parse::<syn::Ident>() {
                if ident == "exclude" {
                    let _: syn::Token![=] = input.parse()?;
                    if input.peek(syn::token::Bracket) {
                        let content;
                        
                        let _ = syn::bracketed!(content in input);
                        while let Ok(val) = content.parse::<LitStr>() {
                            exclude.push(val.value());

                            content.parse::<syn::Token![,]>();
                        }
        
                    } else {
                        let val = input.parse::<LitStr>()?;
                        exclude.push(val.value())
                    }                     
                }
            }
        }

        Ok(AvroInput {
            schema:  path.value(),
            exclude
        }) 
        
    
    }
}


impl AvroInput {

pub fn get_type(&self, schema: &apache_avro::Schema, parent: Option<&apache_avro::Schema>, items: &mut Vec<Item>) -> std::result::Result<TokenStream,Error> {
    let schema_str = schema.canonical_form();
    let namespace = schema.namespace();
    
    match schema.clone() {
       
        apache_avro::Schema::Record(record) => {
            let name = Ident::new(record.name.name.as_str(), Span::call_site());
            
            
            let mut item_struct =  if namespace.is_none() {
                syn::parse2::<ItemStruct>(quote! { 
                    #[derive(Clone,  serde::Serialize, serde::Deserialize, PartialEq, Debug, apache_avro::AvroSchema )]
                    pub struct #name {}
                })
            } else {
                syn::parse2::<ItemStruct>(quote! { 
                    #[derive(Clone,  serde::Serialize, serde::Deserialize, PartialEq, Debug, apache_avro::AvroSchema )]
                    #[avro(namespace = #namespace )]
                    pub struct #name {}
                })
            }?;

            if let syn::Fields::Named(ref mut fields) = item_struct.fields  {

                for field in record.fields {

                    let field_name =field.name.as_str();
                    if parent == None && self.exclude.contains(&&field_name.to_string()) {
                        continue;
                    }
                    let field_name_sc =  Ident::new_raw(snake(field_name).as_str(), Span::call_site());
                  
                    match  self.get_type(&field.schema, Some(&apache_avro::Schema::Ref {name: Name::new(field_name)? } ), items) {
                        Ok(field_type) => 
                        {
                            match Field::parse_named.parse2(
                                quote! {
                               #[avro(rename = #field_name)]
                               #[serde(rename = #field_name)]
                               pub #field_name_sc: #field_type
                            }) {
                                Ok(field) => {
                                    if  field_type.to_string() != (quote! { None }).to_string() {
                                        fields.named.push(field)   
                                    }
                                },
                                Err(e) => {
                                    return Err(e.into());
                                }
                            }
                            
                        },

                        Err(e) => {
                            panic!("ERRR {:?}", e);
                            return Err(e);
                        }
                    };
                }
            }



            items.push(Item::Struct(item_struct));
            
            
           Ok(TokenStream::from_str(record.name.name.as_str())?)
          

        },
        apache_avro::Schema::Union(s) => {
         
            let schema = apache_avro::Schema::Union(s.clone());
            let schema = schema.canonical_form();

            let nullable = s.is_nullable() ;//.variants().iter().any(|&x| x == apache_avro::Schema::Null );
            let variants = s.variants().iter().filter(|&x| *x != apache_avro::Schema::Null  ).collect::<Vec<&Schema>>();
            if variants.len() == 1 {
                let name =  self.get_type(variants.first().unwrap(),parent, items)?;
               return if nullable {
                    let t = syn::parse2::<Type>(name)?;
                    Ok(quote! { Option<#t>})
               } else {
                    Ok(name)
                }
            }
            
            let name = match parent {
                Some(parent) => Ident::new(capitalize(parent.name().unwrap().name.clone() ).as_str(), Span::call_site()),
                None => Ident::new("TEST", Span::call_site()),
            };

            let mut item_enum: ItemEnum = syn::parse2::<ItemEnum>(quote! { 
                #[derive(Clone, /* serde::Serialize,*/  serde::Deserialize, PartialEq, Debug)]
                #[serde(untagged)]
                pub enum #name {}
   
                
            })?;

            
            let imp = syn::parse2::<ItemImpl>(quote! {



                impl apache_avro::schema::derive::AvroSchemaComponent for #name {
                    fn get_schema_in_ctxt(named_schemas: &mut std::collections::HashMap<apache_avro::schema::Name, apache_avro::Schema>, enclosing_namespace: &apache_avro::schema::Namespace) -> apache_avro::Schema {
                        apache_avro::Schema::parse_str(#schema).unwrap()
                    }
                }
            })?;
            
            let variant = variants.iter().enumerate().map(|(index, v)| {
                let variant = Ident::new(format!("{:?}",v).as_str(), Span::call_site()); //Ident::new( v.name().expect(format!("Name missing {:?}", v).as_str()).name.as_str(), Span::call_site());
                quote! { #name::#variant(val) => serializer.serialize_newtype_variant("", #index as u32, stringify!(#variant), val) }
            });
          
        
            let ser = syn::parse2::<ItemImpl>(quote!{
                impl<'a> serde::Serialize for #name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                      where
                          S: serde::Serializer,
                      {
                       
                      match self {
                        #(#variant),*,
                        _ => panic!("NO MATCH")
                      } 
                

                        
                      }
                    }
            }).expect("FÅÅÅK");
            items.push(Item::Impl(ser));
          
/*
            let de_serailize_imp = syn::parse2::<ItemImpl>(quote! {
               
                impl<'de> serde::Deserialize<'de> for #name {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D:  serde::Deserializer<'de>,
                    {
                        let value = serde_json::Value::deserialize(deserializer)?;

                        Ok(Self::String("Hello".to_string()))
                    }
                }
            })?;
         */
            for variant in variants {
                let variant = syn::parse2::<Variant>(match variant {
                    Schema::Null => quote! { Null },
                    Schema::Boolean => quote! { Boolean(bool) },
                    Schema::Int => quote! { Int(i32) },
                    Schema::Long  => quote! { Int(i64) },
                    Schema::Float => quote! { Float(f32) },
                    Schema::Double => quote! { Float(f64) },
                    Schema::Bytes => todo!(),
                    Schema::String => quote! { String(String )},
                    Schema::Array(schema) => {
                        
                        let ident = parse2::<Ident>(self.get_type(&schema.items, parent, items)?)?;
                        quote! { Array(Vec<#ident>)}
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
                    Schema::BigDecimal => todo!(),
                    Schema::TimestampNanos => todo!(),
                    Schema::LocalTimestampNanos => todo!(),
                })?;
                item_enum.variants.push(variant);
            }
          
            let name = item_enum.ident.clone();
           
            items.push(Item::Impl(imp));
            items.push(Item::Enum(item_enum));

           
      //      items.push(Item::Impl(de_serailize_imp));
        
            
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
            
            let t = self.get_type(m.types.as_ref(), parent, items)?;

            return Ok(quote! {  std::collection::HashMap<String, #t> })
        },
        apache_avro::Schema::Null => Ok(quote! { None }),
        apache_avro::Schema::Double =>Ok(quote! { f32}),
        apache_avro::Schema::Array(a) => {
            
            let t = self.get_type(a.items.as_ref(), parent, items)?;
            let t = syn::parse2::<Type>(t)?;
            Ok(quote! { Vec<#t> })
        },
        e => {
            panic!(r#"Unsupported type "{:?}""#, e.name())
        }
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
    let p = parse_macro_input!(args as AvroInput);
    
    if let Some((b,  mut items)) = item_mod.content {
        let uses = quote! {
            use serde::{Deserialize, Serialize};
            use serde::de::Error;
            
         };
         let mut variants: Vec<Variant> = vec![];
       

         //let p = p.value().as_str()
        for entry in  glob::glob(p.schema.as_str()).expect("INVALID PATTERN") {
           let root = match entry {
                Ok(path) => {
                   let file_name = path.file_stem().unwrap();
                   let file_name = file_name.to_str().unwrap();
                    let name = Ident::new(snake(file_name).as_str(), Span::call_site()); 
                    
                    let mut module = syn::parse2::<ItemMod>(quote! { pub mod #name {
                        use serde::{Deserialize, Serialize};
                    } }).unwrap();
                    
                    let mut mod_items: Vec<syn::Item> = Vec::new();

                    let mut file = fs::File::open(path.clone()).unwrap();
                    
                    let schema = apache_avro::Schema::parse_reader(&mut file).unwrap();
                    
                    match p.get_type(&schema, None, &mut mod_items) {
                        Ok(root) => {
                            
                            module.content = Some((b, mod_items));
                            items.push(Item::Mod(module));
                           
                           variants.push(syn::parse2::<Variant>(quote! {  #root(#root) }).unwrap());
                           if let Ok(ident) = syn::parse2::<Ident>(root){
                             Some( quote! {  #name::#ident } )
                           } else {
                            None
                           }
                           
                           
                        },
                        Err(e) => panic!("ERROR PARSING FILE {:?}", path)
                    }
 
                },
                Err(e) => {
                    None
                }
            };
            
            if let Some(root) = root {
                match syn::parse2::<ItemUse>(quote! { pub use #root; } ) {
                    Ok(use_item) =>  {
                        items.push(Item::Use(use_item));
                    },
                    Err(e ) => panic!("FOO {}", e)
                }
            }
        }
        
        items.push(Item::Verbatim(uses));
  
        let mut types = Punctuated::<Variant, Token![,]>::new();
        for variant in variants {
            types.push(variant)
        }
        
      

        let types_token_stream = types.to_token_stream();
        items.push(Item::Verbatim( quote! {
            use avro_macro::create_events; 

            create_events!(Events { #types_token_stream }); }));

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



#[proc_macro_derive(TaggedEnum, attributes(tag))]
pub fn register_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    
    let attrs = &input.attrs;

    

    let a = quote! {
        match tag {}
    };

    let mut a = syn::parse2::<syn::ExprMatch>(a).unwrap();
    


    if let syn::Data::Enum(e)  = &input.data {
        for var in &e.variants {
            
            let ident = &var.ident;
            for  a in &var.fields {
                println!("{:?}", a);
            }
            let event_type = ident.to_string();
            println!("{:?}", &var);
            a.arms.push(syn::parse2::<syn::Arm>(quote! { #event_type => {
               let value = serde_json::from_str::<#ident>(value)?; 
            Ok(#name :: #ident (value))
             
            }  }).unwrap());

          /*  println!("Variant {}", var.ident.to_string());
            if let syn::Fields::Unnamed(f) = &var.fields {
                if let Some(field) = f.unnamed.first() {
                 println!("{:?}",field.ty);
                }
            };
            let fields = &var.fields;
            for field in fields {
                println!("Field {:?}", &field.ident);
            }
             */
        }
    };

    a.arms.push(syn::parse2::<syn::Arm>(quote! { _ => panic!("Unsupported") }).unwrap());
   
    for attr in attrs {
        //if attr.path.is_ident("attach") {
        //    parse_attach_attribute(&attr);
       // }
    }
 //   println!("{:#?}", input);

    proc_macro::TokenStream::from(quote! {
        impl #name {
            fn from_str(tag: &str, value: &str) ->  Result<Self, serde_json::Error> {
               #a
            }
        }
    })

}



#[proc_macro]
pub fn create_events(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //let test = parse_macro_input!(item  as syn::ItemEnum);
    
    let input: proc_macro2::TokenStream = input.into();

    let event_enum = syn::parse2::<ItemEnum>(quote! { 
     

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        #[serde(untagged)]
        pub enum #input
    }).unwrap();


    let name = &event_enum.ident;
    let variants = &event_enum.variants;

    let variant_names = variants.iter().map(|v| {
        let name = &v.ident;
        quote! { stringify!(#name) }
    });

    let variant_name = variants.iter().map(|v| {
        let variant = &v.ident;
        quote! { #name::#variant(_) => stringify!(#variant) }
    });

    let parse = variants.iter().map(|v| {
        let variant = &v.ident;
        
        let field_types = match &v.fields {
            Fields::Named(fields) => {
                fields.named.iter().map(|f| &f.ty).collect::<Vec<_>>()
            },
            Fields::Unnamed(fields) => {
                fields.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>()
            }, 
            Fields::Unit => vec![] ,
        };

        let variant_type = field_types.first().map(|ty| quote! { #ty} ).unwrap_or(quote! {});
        
        quote! { stringify!(#variant) => {
            match serde_json::from_str::<#variant_type>(value) {
                Ok(val) => Ok(Self::#variant(val)),
                Err(e) => {
                    Err(e)
                }
            } 
        } }
    });
    
    let variant_schemas = variants.iter().map(|v| {
        let variant = &v.ident;
        
        let field_types = match &v.fields {
            Fields::Named(fields) => {
                fields.named.iter().map(|f| &f.ty).collect::<Vec<_>>()
            },
            Fields::Unnamed(fields) => {
                fields.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>()
            }, 
            Fields::Unit => vec![] ,
        };

        let variant_type = field_types.first().map(|ty| quote! { #ty} ).unwrap_or(quote! {});
        
        variant_type
    });
    proc_macro::TokenStream::from(quote!{
      

        #event_enum
        impl std::fmt::Display for  #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl #name {

            pub fn event_names() -> &'static[&'static str] {
                &[#(#variant_names),*]
            }

            pub fn event_name(&self) -> &'static str {
                match self {
                    #(#variant_name),*
                }
            }

            pub fn from_str(tag: &str, value: &str) -> Result<Self,serde_json::Error> {
                match tag {
                    #(#parse) *,
                    _ => Err(serde_json::Error::custom(format!("Unsupported tag \"{}\"", tag)))
                }
            }
        }


        impl apache_avro::schema::derive::AvroSchemaComponent for #name {
            fn get_schema_in_ctxt(named_schemas: &mut std::collections::HashMap<apache_avro::schema::Name, apache_avro::Schema>, enclosing_namespace: &apache_avro::schema::Namespace) -> apache_avro::Schema {
                let schemas = vec![ #(#variant_schemas::get_schema_in_ctxt(named_schemas, enclosing_namespace)),* ] ;
                apache_avro::schema::Schema::Union(
                    apache_avro::schema::UnionSchema::new(schemas.into_iter().map(|schema|{
                        apache_avro::schema::Schema::Ref { name: schema.name().unwrap().clone() }
                }).collect::<Vec<_>>()).unwrap()
       )
    }
}
    })
}
