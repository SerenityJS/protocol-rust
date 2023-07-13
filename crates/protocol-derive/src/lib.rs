extern crate proc_macro;

use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

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
  let id = u8::from_str_radix(&id, 16).unwrap();

  let ast = parse_macro_input!(item as ItemStruct);
  let struct_name = &ast.ident;

  let gen = quote! {
    #ast
    #[napi]
    impl #struct_name {
      #[napi(js_name = "id")]
      pub fn pid() -> u8 {
        #id
      }

      #[napi]
      pub fn id(&self) -> u8 {
        #id
      }
    }
  };

  gen.into()
}
