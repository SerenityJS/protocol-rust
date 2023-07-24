extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, DataEnum, Ident, Data, Fields, Field, Type};
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
  let ast = parse_macro_input!(item as DeriveInput);
  let struct_name = &ast.ident;

  // Get the fields and types of the struct
  let fields = match ast.data { 
    Data::Struct(ref data_struct) => &data_struct.fields,
    _ => panic!("packet derive macro can only be used on structs"),
  };

  // Generate the from_object function
  let from_object = generate_packet_conversion_from_object(fields);
  let to_object = generate_packet_conversion_to_object(fields);

  // Generate the serialize and deserialize functions
  let serialize = generate_packet_serialization(fields);
  let deserialize = generate_packet_deserialization(fields);

  // If there are args then it is a root packet struct
  if !args.is_empty() {
    // format is packet(0x00) parse the hex id out of args
    let id = args.to_string().replace("0x", "");
    let id = i32::from_str_radix(&id, 16).unwrap();

    if !struct_name.to_string().ends_with("Packet") {
      panic!("Root packet structs must end with 'Packet' in their name.");
    }

    let gen = quote! {
      #ast
  
      impl #struct_name {
        pub const ID: crate::binary::VarInt = #id;
      }
  
      impl crate::packets::prelude::PacketConversion for #struct_name {
        fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self> {
          #from_object
        }
        fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object> {
          let mut object = env.create_object()?;
  
          #to_object
  
          Ok(object)
        }
      }
  
      impl crate::packets::prelude::PacketSerialization for #struct_name {
        fn serialize(&self) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Buffer> {
          let mut stream = crate::binary::BinaryStream::new();
  
          stream.write_varint(#struct_name::ID)?;
          #serialize
  
          Ok(stream.data.into())
        }
        fn deserialize(data: napi::bindgen_prelude::Buffer) -> napi::bindgen_prelude::Result<Self> {
          let mut stream = crate::binary::BinaryStream::from(data.into());
  
          let _id = stream.read_varint()?;
          #deserialize
        }
      }
    };
  
    return gen.into();
  }

  // Otherwise it is a nested structure and needs to be handled differently
  let gen = quote! {
    #ast

    impl crate::packets::prelude::PacketChildConversion for #struct_name {
      fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self> {
        #from_object
      }
      fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object> {
        let mut object = env.create_object()?;

        #to_object

        Ok(object)
      }
    }

    impl crate::packets::prelude::PacketChildSerialization for #struct_name {
      fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream> {
        let mut stream = crate::binary::BinaryStream::new();
    
        #serialize

        Ok(stream)
      }
    
      fn deserialize(stream: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self> {
        #deserialize
      }
    }
  };
  
  gen.into()
}

fn generate_packet_conversion_from_object(fields: &Fields) -> TokenStream2 {
  let field_names = fields.iter().map(|field| {
    field.ident.as_ref().unwrap()
  });

  let getters = fields.iter().map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    // let field_type_string = quote!(#field_type).to_string();

    let field_name_camel = field_name.to_string().from_case(Case::Snake).to_case(Case::Camel);

    if is_not_managed_type(field) {
      // panic!("'{}' is not supported in packets yet!", field_type_string);

      return quote! {
        let #field_name: #field_type = match #field_type::from_object(data.get_named_property(#field_name_camel)?) {
          Ok(value) => value,
          Err(err) => {
            return Err(napi::Error::new(
              err.status,
              format!("Expected property '{}' to be of type '{}'\n{}", #field_name_camel, stringify!(#field_type), err)
            ));
          }
        };
      }
    }

    quote! {
      // let #field_name: #field_type = data.get_named_property(stringify!(#field_name))?;
      // above but with custom napi error
      let #field_name: #field_type = match data.get_named_property(#field_name_camel) {
        Ok(value) => value,
        Err(err) => {
          return Err(napi::Error::new(
            err.status,
            format!("Expected property '{}' to be of type '{}'\n{}", #field_name_camel, stringify!(#field_type), err)
          ));
        }
      };
      
    }
  });

  quote! {
    #(#getters)*

    Ok(Self {
      #(#field_names),*
    })
  }
}

fn generate_packet_conversion_to_object(fields: &Fields) -> TokenStream2 {
  let setters = fields.iter().map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let field_name_camel = field_name.to_string().from_case(Case::Snake).to_case(Case::Camel);

    let field_type = &field.ty;
    let field_type_string = quote!(#field_type).to_string();

    if is_not_managed_type(field) {
      // panic!("'{}' is not supported in packets yet!", field_type_string);

      return quote! {
        object.set_named_property(#field_name_camel, self.#field_name.to_object(env)?)?;
      }
    }

    // Handle ownership of strings
    if field_type_string == "String" {
      return quote! {
        object.set_named_property(#field_name_camel, self.#field_name.to_owned())?;
      }
    }

    quote! {
      object.set_named_property(#field_name_camel, self.#field_name)?;
    }
  });

  quote! {
    #(#setters)*
  }
}

fn generate_packet_deserialization(fields: &Fields) -> TokenStream2 {
  let field_names = fields.iter().map(|field| {
    field.ident.as_ref().unwrap()
  });

  let setters = fields.iter().map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    let field_type_string = quote!(#field_type).to_string();

    let field_method = format_ident!("read_{}", field_type_string.to_lowercase());

    if is_not_managed_type(field) {
      // panic!("'{}' is not supported in packets yet!", field_type_string);

      return quote! {
        let #field_name = #field_type::deserialize(&mut stream)?;
      }
    }

    quote! {
      let #field_name = stream.#field_method()?;
    }

  });

  quote! {
    #(#setters)*

    Ok(Self {
      #(#field_names),*
    })
  }
}

fn generate_packet_serialization(fields: &Fields) -> TokenStream2 {
  let setters = fields.iter().map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    let field_type_string = quote!(#field_type).to_string();

    let field_method = format_ident!("write_{}", field_type_string.to_lowercase());

    // Types that are not manaually managed will be required to 
    if is_not_managed_type(field) {
      // panic!("'{}' is not supported in packets yet!", field_type_string);

      return quote! {
        let mut #field_method = self.#field_name.serialize()?;
        stream.append(&mut #field_method);
      }
    }

    // Handle ownership of strings
    if field_type_string == "String" {
      return quote! {
        stream.#field_method(self.#field_name.to_owned())?;
      }
    }

    quote! {
      stream.#field_method(self.#field_name)?;
    }
  });

  quote! {
    #(#setters)*
  }
}

fn is_not_managed_type(field: &Field) -> bool {
  if let Type::Path(path) = &field.ty {
    if let Some(segment) = path.path.segments.last() {
        let ident = &segment.ident.to_string();
        return !is_managed_type(ident);
    }
  }
  false
}

fn is_managed_type(ident: &str) -> bool {
  let managed_types = [
    "String",
    "bool",
    "i8",
    "i16",
    "i32",
    "i64",
    "u8",
    "u16",
    "u32",
    "u64",
    "i128",
    "u128",
    "f32",
    "f64",
    "VarInt"
  ];

  managed_types.contains(&ident)
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

  data.variants.iter().for_each(|varint| {
    if varint.ident.to_string().ends_with("Packet") {
      panic!("Enum variant names should match their associated packet name without 'Packet'!");
    }
  });

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
