#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod binary;
mod packets;


// #[packet(0x01)]
// #[napi(object)]
// pub struct Test {
//   pub test: String,
// }

// #[napi]
// #[derive(Debug)]
// pub enum PacketType {
//   Test = 0x01,
// }


// #[napi]
// pub fn serialize(id: PacketType, data: Object) -> Result<Buffer> {
//   match id {
//     PacketType::Test => {
//       let test: String = data.get_named_property("test")?;

//       println!("test: {}", test);

//       Ok(vec![id as u8].into())
//     },
//   }
//   // let test: String = data.get_named_property("test")?;

//   // println!("test: {}", test);

//   // Ok(vec![id as u8].into())
// }



// type ZigZag32 = i32;

// #[napi]
// pub struct Test {
//   pub a: ZigZag32,
// }


// lets not do this. time to earn our good noodle stars and use Result in BinaryStream
// then we can use ? and ?? and all the other goodies. If we make the errors napi to begin
// with then we can just let them propagate.
// #[napi]
// pub fn testing_avoiding_panic() -> napi::Result<bool> {
//   let result = std::panic::catch_unwind(|| {
//     let temp = vec![1];

//     temp[1];

//     false
//   });

//   match result {
//     Ok(result) => Ok(result),
//     // Otherwise emit a napi error
//     Err(_) => Err(napi::Error::new(
//       napi::Status::GenericFailure,
//       "Panic occured",
//     )),
//   }
// }
