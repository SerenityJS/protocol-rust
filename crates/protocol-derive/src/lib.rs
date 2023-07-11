extern crate proc_macro;

use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Packet, attributes(packet, r#use))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);

  let struct_name = &ast.ident;

  let packet_attributes = PacketAttributes::from(&ast);

  let id = packet_attributes.id;

  // Gets all fields from the struct

  let fields = match ast.data { 
    syn::Data::Struct(ref data_struct) => &data_struct.fields,
    _ => panic!("Packet derive macro can only be used on structs"),
  };

  // Loops through all fields and generates a serialize function
  let mut gens = Vec::new();

  fields.iter().for_each(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    let gen = quote! {
      println!("{}: {}", stringify!(#field_name), stringify!(#field_type));
    };

    gens.push(gen);
  });

  let gen = quote! {
    #[napi]
    impl #struct_name {
      // Adds above tokensteams to this quote!
      #[napi]
      pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        #(#gens)*

        buffer
      }

      // #[napi]
      // Loops throgh all field and serializes them for raknet spec
      // pub fn serialize(&self) -> Vec<u8> {
      //   let mut buffer = Vec::new();

      //   #(
      //     buffer.extend(self.#r#use.serialize());
      //   )*

      //   buffer
      // }

      #[napi]
      pub fn packet_id() -> u8 {
        #id
      }
    }
  };

  gen.into()
}

struct PacketAttributes {
  id: u8,
}

// TODO: make this less gay
impl PacketAttributes {
  fn from(ast: &DeriveInput) -> Self {
    let mut id: u8 = 0;

    for attr in &ast.attrs {
      if attr.path().is_ident("packet") {
        let out = attr.parse_nested_meta(|meta| {
          
          if meta.path.is_ident("id") {
            let value = meta.value()
              .expect("Cannot parse packet id!");
            if let syn::Lit::Int(lit_int) = value.parse()
              .expect("Cannot parse packet id!") {
              id = lit_int.base10_parse::<u8>()
                .expect("Packet id is invalid!");
            }
          }

          Ok(())
        });

        match out {
          Ok(_) => (),
          Err(err) => panic!("{}", err),
        }
      }
    }

    Self {
      id,
    }
  } 
}

// #[proc_macro_attribute]
// pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
//   // Parse the input tokens into a syntax tree
//   let input = parse_macro_input!(input as DeriveInput);

//   let mut attributes: HashMap<String, String> = HashMap::new();

//   let attr_string = attr.to_string();
//   let seperated: Vec<&str> = attr_string.split(',').collect();
  
//   // Add attributes to hashmap
//   for attr in seperated {
//     let seperated_attr: Vec<&str> = attr.split('=').collect();
//     attributes.insert(seperated_attr[0].trim().to_string(), seperated_attr[1].trim().to_string());
//   }

//   let id: String = unwrap_or_panic(attributes, "id");

//   // Extract the struct name
//   let struct_name = &input.ident;

//   // Generate implementations
//   // Normally we want to use traits here 
//   // but napi doesnt like trait implementations
//   let expanded = quote! {
//     #[napi]
//     impl #struct_name {
//       #[napi]
//       pub fn packet_id() -> String {
//         String::from(#id)
//       }
//     }
//   };

//   // Return the generated implementation as a token stream
//   expanded.into()
// }

// // Will attempt to unwrap a value from a hashmap
// // If the value is not found, it will panic
// fn unwrap_or_panic<T>(map: HashMap<String, String>, key: &str) -> T
// where
//   T: FromStr,
//   <T as FromStr>::Err: Debug,
// {
//   match map.get(key) {
//     Some(value) => match value.parse::<T>() {
//       Ok(value) => value,
//       Err(_) => panic!("Invalid `{}` attribute value: `{}`", key, value),
//     },
//     None => panic!("Missing `{}` attribute", key),
//   }
// }

