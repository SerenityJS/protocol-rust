extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, format_ident};
use syn::{
  Data::{Struct, Enum},
  DataStruct,
  Fields::Named,
  Result,
  parse_macro_input,
  DeriveInput,
  FieldsNamed,
  Field,
  Ident, DataEnum, Type, PathArguments, GenericArgument
};
use convert_case::{Case, Casing};

struct PacketMeta {
  id: Option<u32>,
  attrs: Vec<String>
}

impl PacketMeta {
  fn parse(input: TokenStream) -> Result<Self> {
    let attrs: Vec<String> = input.to_string().split(',')
      .map(|arg| arg.trim().to_string())
      .filter(|arg| !arg.is_empty())
      .collect();

    // Attempt to get id attribute. Will be a literal attribute starting with 0x
    let id = attrs.iter().find_map(|attr| {
      let attr = attr;
      if attr.starts_with("0x") {
        let id = attr.replace("0x", "");
        let id = u32::from_str_radix(&id, 16).unwrap();
        
        Some(id)
      } else {
        None
      }
    });

    Ok(PacketMeta {
      id,
      attrs,
    })
  }
}

struct PacketField {
  field: Field,
  attr: Option<String>
}

impl PacketField {
  fn parse(ast: &DeriveInput) -> syn::Result<Vec<Self>> {
    let ast_fields = match ast.data {
      Struct(DataStruct { fields: Named(FieldsNamed { ref named, .. }), .. }) => named,
      _ => panic!("Only structs with named fields are supported"),
    };

    let fields = ast_fields.iter()
      .map(|field| {
        let attrs: Vec<String> = field.attrs.iter()
          .map(|attr| attr.path().to_token_stream().to_string())
          .collect();

        // Filter out attributes that start with "napi"
        let attrs: Vec<String> = attrs.into_iter()
          .filter(|attr| !attr.starts_with("napi"))
          .collect();

        let attr = attrs.first().cloned();

        PacketField {
          field: field.clone(),
          attr
        }
      }).collect();

    Ok(fields)
  }
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;

  // Parse packet attributes
  let meta = PacketMeta::parse(args)
    .expect("Failed to parse packet attributes");
  let id = meta.id;

  let manual_serialize = meta.attrs.iter().any(|attr| attr == "manual_serialize");

  // Root packet structs must end with "Packet" for post build type manipulation
  if let Some(_) = id {
    if !name.to_string().ends_with("Packet") {
      panic!("Root packet structs must end with 'Packet' in their name.");
    }
  }

  // Generate the packet field data
  let fields = PacketField::parse(&ast)
    .expect("Failed to parse packet fields");

  let serialize = object_serialize_gen(&fields, false);
  let deserialize = object_serialize_gen(&fields, true);
  let to_object = object_conversion_gen(&fields, false);
  let from_object = object_conversion_gen(&fields, true);

  let impls = match id {
    Some(id) => quote! {
      impl #name {
        pub const ID: crate::binary::prelude::VarInt = #id;
      }

      impl crate::packets::prelude::PacketSerialization for #name {
        fn serialize(&self) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Buffer> {
          let mut stream = crate::binary::BinaryStream::new();
  
          stream.write_varint(#name::ID)?;
          #serialize
  
          Ok(stream.data.into())
        }
        fn deserialize(data: napi::bindgen_prelude::Buffer) -> napi::bindgen_prelude::Result<Self> {
          let mut stream = crate::binary::BinaryStream::from(data.into());
  
          let _id = stream.read_varint()?;
          #deserialize
        }
      }
    },
    None => quote! {
      impl crate::packets::prelude::PacketChildSerialization for #name {
        fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream> {
          let mut stream = crate::binary::BinaryStream::new();
      
          #serialize
  
          Ok(stream)
        }
      
        fn deserialize(stream: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self> {
          #deserialize
        }
      }
    }
  };

  let impls = match manual_serialize {
    true => quote! {},
    false => impls
  };

  let gen = quote! {
    #[napi(object)]
    #[derive(protocol_derive::PacketFieldAttributes)]
    #ast

    #impls

    impl crate::packets::prelude::PacketConversion for #name {
      fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object> {
        let mut object = env.create_object()?;

        #to_object

        Ok(object)
      }

      fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self> {
        #from_object
      }
    }
  };

  // println!("{}", gen.to_string());

  gen.into()
}

fn object_conversion_gen(fields: &Vec<PacketField>, from: bool) -> TokenStream2 {
  let field_names = fields.iter().map(|field| {
    field.field.ident.clone().unwrap()
  });

  let actions = fields.iter().map(|field| {
    let field_name = field.field.ident.clone().unwrap();
    let field_type = field.field.ty.clone();
    
    conversion_actions_gen(field_name, field_type, from)
  });

  match from {
    true => quote! {
      #(#actions)*

      Ok(Self {
        #(#field_names),*
      })
    },
    false => quote! {
      #(#actions)*
    }
  }
}

fn conversion_actions_gen(field_name: Ident, field_type: Type, from: bool) -> TokenStream2 {
  let field_type_string = quote!(#field_type).to_string();
  let field_name_camel = field_name.to_string().from_case(Case::Snake).to_case(Case::Camel);

  let match_arms = quote! {
    Ok(value) => value,
    Err(err) => return Err(napi::Error::new(
      err.status,
      format!("Expected property '{}' to be of type '{}'\n{}", #field_name_camel, stringify!(#field_type), err),
    ))
  };

  match from {
    true => {
      // Handles Vecs
      if is_vec(&field_type) {
        let vec_type_string = get_vec_type(&field_type);
        let vec_type = format_ident!("{}", vec_type_string);

        if vec_type_string == "vec" {
          panic!("Nested Vecs are not supported! Please use a struct.");
        }

        return match is_managed_type(&vec_type_string) {
          // If its managed we can return default pretty much.
          true => quote! {
            let #field_name: #field_type = match data.get_named_property(#field_name_camel) {
              #match_arms
            };
          },
          // If its not managed then we need to serialize all the structs into objects
          // and add them to a vec.
          false => {
            let napi_raw = format_ident!("{}_napi", field_name);

            quote! {
              let #napi_raw: Vec<napi::bindgen_prelude::Object> = match data.get_named_property(#field_name_camel) {
                #match_arms
              };
              let mut #field_name: #field_type = Vec::new();
              for item in #napi_raw {
                #field_name.push(#vec_type::from_object(item)?);
              }
            }
          }
        };
      }

      // Handles everything that isn't a Vec
      let object_accessor = match is_managed_type(&field_type_string) {
        true => quote!(data.get_named_property(#field_name_camel)),
        false => quote!(#field_type::from_object(data.get_named_property(#field_name_camel)?))
      };

      quote! {
        let #field_name: #field_type = match #object_accessor {
          #match_arms
        };
      }
    },
    false => {
      // Handles Vecs
      if is_vec(&field_type) {
        let vec_type_string = get_vec_type(&field_type);
        // let vec_type = format_ident!("{}", vec_type_string);

        if vec_type_string == "vec" {
          panic!("Nested Vecs are not supported! Please use a struct.");
        }

        return match is_managed_type(&vec_type_string) {
          true => quote! {
            object.set_named_property(#field_name_camel, self.#field_name.to_owned())?;
          },
          false => quote! {
            let mut #field_name = env.create_array_with_length(self.#field_name.len())?;
            for (i, item) in self.#field_name.iter().enumerate() {
              let obj = item.to_object(env)?;
              #field_name.set_element(i as u32, obj)?;
            }
            object.set_named_property(#field_name_camel, #field_name)?;
          }
        }
      }

      // Handles everything that isn't a Vec
      let object_accessor = match is_managed_type(&field_type_string) {
        true => quote!(self.#field_name.to_owned()),
        false => quote!(self.#field_name.to_object(env)?)
      };

      quote! {
        object.set_named_property(#field_name_camel, #object_accessor)?;
      }
    }
  }
}

fn object_serialize_gen(fields: &Vec<PacketField>, from: bool) -> TokenStream2 {
  let field_names = fields.iter().map(|field| {
    field.field.ident.clone().unwrap()
  });

  let actions = fields.iter().map(|field| {
    let field_name = field.field.ident.clone().unwrap();
    let field_type = field.field.ty.clone();

    serialize_actions_gen(field_name, field_type, &field.attr, from)
  });

  match from {
    true => quote! {
      #(#actions)*

      Ok(Self {
        #(#field_names),*
      })
    },
    false => quote! {
      #(#actions)*
    }
  }
}

fn serialize_actions_gen(field_name: Ident, field_type: Type, attr: &Option<String>, from: bool) -> TokenStream2 {
  let field_type_string = quote!(#field_type).to_string();
  let field_method = match is_vec(&field_type) {
    true => format_ident!("noop"),
    false => match from {
      true => format_ident!("read_{}", field_type_string.to_lowercase()),
      false => format_ident!("write_{}", field_type_string.to_lowercase()),
    }
  };

  match from {
    true => {
      // Handles Vecs
      if is_vec(&field_type) {
        let vec_type_string = get_vec_type(&field_type);
        let vec_type = format_ident!("{}", vec_type_string);

        if vec_type_string == "vec" {
          panic!("Nested Vecs are not supported! Please use a struct.");
        }

        let size_type = match attr {
          Some(attr) => attr.to_string(),
          None => panic!("Vecs must have a size attribute!"),
        };  
        let size_method = match from {
          true => format_ident!("read_{}", size_type.to_lowercase()),
          false => format_ident!("write_{}", size_type.to_lowercase()),
        };

        let len = format_ident!("{}_len", field_name);
        return match is_managed_type(&vec_type_string) {
          true => {
            let field_method = format_ident!("read_{}", vec_type_string.to_lowercase());

            quote! {
              let #len = stream.#size_method()?;
              let mut #field_name: #field_type = Vec::new();
              for _ in 0..#len {
                #field_name.push(stream.#field_method()?);
              }
            }
          },
          false => quote! {
            let #len = stream.#size_method()?;
            let mut #field_name: #field_type = Vec::new();
            for _ in 0..#len {
              #field_name.push(#vec_type::deserialize(&mut stream)?);
            }
          }
        };
      }

      let size_method = match attr {
        Some(attr) => match is_managed_type(&field_type_string) {
          true => panic!("Managed types cannot have a size attribute!"),
          false => {
            let size_type = attr.to_string();
            let size_method = format_ident!("read_{}", size_type.to_lowercase());
  
            quote! {
              // We currently do not verify the size is correct
              stream.#size_method()?;
            }
          }
        },
        None => quote! {},
      };

      // Handle everything that isn't a Vec
      let object_accessor = match is_managed_type(&field_type_string) {
        true => quote!(stream.#field_method()?),
        false => quote!(#field_type::deserialize(&mut stream)?)
      };

      quote! {
        #size_method
        let #field_name: #field_type = #object_accessor;
      }
    },
    false => {// Handles Vecs
      if is_vec(&field_type) {
        let vec_type_string = get_vec_type(&field_type);
        // let vec_type = format_ident!("{}", vec_type_string);

        if vec_type_string == "vec" {
          panic!("Nested Vecs are not supported! Please use a struct.");
        }

        let size_type = match attr {
          Some(attr) => attr.to_string(),
          None => panic!("Vecs must have a size attribute!"),
        };  
        let size_ident = format_ident!("{}", size_type);
        let size_method = match from {
          true => format_ident!("read_{}", size_type.to_lowercase()),
          false => format_ident!("write_{}", size_type.to_lowercase()),
        };

        let len = format_ident!("{}_len", field_name);
        return match is_managed_type(&vec_type_string) {
          true => {
            let field_method = format_ident!("write_{}", vec_type_string.to_lowercase());

            quote! {
              let #len = self.#field_name.len();
              stream.#size_method(#len as #size_ident)?;
              for i in 0..#len {
                stream.#field_method(self.#field_name[i].to_owned())?;
              }
            }
          },
          false => quote! {
            let #len = self.#field_name.len();
            stream.#size_method(#len as #size_ident)?;
            for i in 0..#len {
              let mut bin = self.#field_name[i].serialize()?;
              stream.append(&mut bin);
            }
          }
        };
      }

      // if attr and is not a managed type
      if attr.is_some() && !is_managed_type(&field_type_string) {
        let size_type = attr.as_ref().unwrap().to_string();
        let size_type_ident = format_ident!("{}", size_type);
        let size_method = format_ident!("write_{}", size_type.to_lowercase());

        return quote! {
          let mut #field_name = self.#field_name.serialize()?;
          stream.#size_method(#field_name.len() as #size_type_ident)?;
          stream.append(&mut #field_name);
        };
      }

      // Handle everything that isn't a Vec
      let object_accessor = match is_managed_type(&field_type_string) {
        true => quote!{ stream.#field_method(self.#field_name.to_owned())? },
        false => quote!{ stream.append(&mut self.#field_name.serialize()?) }
      };

      quote! {
        #object_accessor;
      }
    }
  }
}

fn is_managed_type(ident: &str) -> bool {
  match ident {
      "i8" 
    | "i16" 
    | "i32" 
    | "i64" 
    | "u8" 
    | "u16" 
    | "u32" 
    | "u64"  
    | "f32" 
    | "f64" 
    | "bool" 
    | "String"
    // | "Value"
    // | "Vec"
    // Custom Types
    | "LittleString"
    | "LU16"
    | "LI16"
    | "LF32"
    | "LU64"
    | "VarInt"
    | "VarLong"
    | "ZigZag"
    | "ZigZong" 
      => true,
    _ => false
  }
}

fn is_vec(ty: &Type) -> bool {
  if let Type::Path(path) = ty {
    if let Some(segment) = path.path.segments.last() {
        let ident = &segment.ident.to_string();
        return ident == "Vec";
    }
  }
  false
}

fn get_vec_type(ty: &Type) -> String {
  if let Type::Path(path) = ty {
    if let Some(segment) = path.path.segments.last() {
      if segment.ident.to_string() == "Vec" {
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
          if let Some(GenericArgument::Type(type_arg)) = args.args.first() {
            let type_name = quote::quote! { #type_arg }.to_string();

            return type_name;
          }
        }
      }
    }
  }
  panic!("Field is not a Vec!");
}

// Has no significant function just allow sizing attributes for properties
// namely used for arrays. Attributes MUST follow binary naming convention
#[proc_macro_derive(PacketFieldAttributes, attributes(
  LI16,
  LI32,
  VarInt,
))]
pub fn derive_packet_field_attributes(_item: TokenStream) -> TokenStream {
  TokenStream::new()
}

// Generates the serialize and deserialize functions for packet enum
#[proc_macro_attribute]
pub fn packet_enum(_args: TokenStream, input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  // Extract the name of the enum
  let enum_name = &input.ident;

  // Extract the data of the enum
  let data = match &input.data {
    Enum(data) => data,
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
    #[napi]
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
