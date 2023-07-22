extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, ItemStruct, DataEnum, Ident, Data};
use convert_case::{Case, Casing};

// This is not optimal, children constructors this is applied to will not be references in the parent
// they will be cloned. This is a bit of a stinky hack, should probably try to get references working.
#[proc_macro_derive(UseConstructorCloning)]
pub fn use_constructor_cloning_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let struct_name = &ast.ident;

  // Gets all the fields in the struct.
  let fields = match ast.data { 
    syn::Data::Struct(ref data_struct) => &data_struct.fields,
    _ => panic!("FixNapi derive macro can only be used on structs"),
  };

  // Gets all the field names as idents for later use.
  let field_name_indentifiers = fields.iter().map(|field| {
    field.ident.as_ref().unwrap()
  });

  // Loops through all fields and generates a serialize function
  let mut gens = Vec::new();
  fields.iter().for_each(|field| {
    let field_name = field.ident.as_ref().unwrap();
    // let field_type = &field.ty;

    // This will extract the field from the object
    let gen = quote! {
      let #field_name = cloned.#field_name;
    };

    gens.push(gen);
  });

  // Implements it all into out impl
  let gen = quote! {
    impl napi::bindgen_prelude::FromNapiValue for #struct_name {
      unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
        let object: napi::bindgen_prelude::ClassInstance<#struct_name> = napi::bindgen_prelude::ClassInstance::from_napi_value(env, value)?;
        let cloned = object.clone();

        #(#gens)*

        Ok(#struct_name {
          #(#field_name_indentifiers),*
        })
      }
    }
  };

  gen.into()
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, item: TokenStream) -> TokenStream {
  // format is packet(0x00) parse the hex id out of args
  let id = args.to_string().replace("0x", "");
  let id = i32::from_str_radix(&id, 16).unwrap();

  let ast = parse_macro_input!(item as ItemStruct);
  let struct_name = &ast.ident;

  let gen = quote! {
    #ast
    // #[napi]
    impl #struct_name {
      // #[napi]
      // pub fn id() -> i32 {
      //   #id
      // }
      pub const ID: crate::binary::VarInt = #id;
    }
  };

  gen.into()
}

// Generates the serialize and deserialize functions for packet enum
#[proc_macro_attribute]
pub fn enum_serializer(_args: TokenStream, input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  // Extract the name of the enum
  let enum_name = &input.ident;

  // Extract the data of the enum
  let data = match &input.data {
    Data::Enum(data) => data,
    _ => panic!("Only enums are supported."),
  };

  // Generate match arms for serialize and deserialize
  let match_serialize = generate_match_arms(enum_name, &data, true);
  let match_deserialize = generate_match_arms(enum_name, &data, false);

  let expanded = quote! {
    #input

    #[napi]
    pub fn serialize(id: #enum_name, data: Object) -> Result<Buffer> {
      match id {
        #match_serialize
      }
    }

    #[napi]
    pub fn deserialize(env: Env, id: #enum_name, data: Buffer) -> Result<Object> {
      match id {
        #match_deserialize
      }
    }
  };

  expanded.into()
}

fn generate_match_arms(enum_name: &Ident, data: &DataEnum, is_serialize: bool) -> TokenStream2 {
  let match_arms = data.variants.iter().map(|variant| {
    let ident = &variant.ident;
    let variant_name = format_ident!("{}Packet", ident);

    // convert ident to snake case ident
    let variant_name_snake = format_ident!("{}", ident.to_string().from_case(Case::UpperCamel).to_case(Case::Snake));

    if is_serialize {
      quote! {
        #enum_name::#ident => {
          let packet = #variant_name_snake::#variant_name::from_object(data)?;
          packet.serialize()
        }
      }
    } else {
      quote! {
        #enum_name::#ident => {
          let packet = #variant_name_snake::#variant_name::deserialize(data)?;
          Ok(packet.to_object(env)?)
        }
      }
    }
  });

  quote! {
    #(#match_arms),*
  }
}
