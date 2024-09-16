//! This crate provides a convenient concise way to write unit tests for
//! implementations of [`Serialize`] and [`Deserialize`].
//!
//! [`Serialize`]: serde::ser::Serialize
//! [`Deserialize`]: serde::de::Deserialize
//!
//! The `Serialize` impl for a value can be characterized by the sequence of
//! [`Serializer`] calls that are made in the course of serializing the value,
//! so `serde_test` provides a [`Token`] abstraction which corresponds roughly
//! to `Serializer` method calls. There is an [`assert_ser_tokens`] function to
//! test that a value serializes to a particular sequence of method calls, an
//! [`assert_de_tokens`] function to test that a value can be deserialized from
//! a particular sequence of method calls, and an [`assert_tokens`] function to
//! test both directions. There are also functions to test expected failure
//! conditions.
//!
//! [`Serializer`]: serde::ser::Serializer
//!
//! Here is an example from the [`linked-hash-map`] crate.
//!
//! [`linked-hash-map`]: https://github.com/contain-rs/linked-hash-map
//!
//! ```edition2021
//! # const IGNORE: &str = stringify! {
//! use linked_hash_map::LinkedHashMap;
//! # };
//! use serde_test::{assert_tokens, Token};
//!
//! # use std::fmt;
//! # use std::marker::PhantomData;
//! #
//! # use serde::ser::{Serialize, Serializer, SerializeMap};
//! # use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
//! #
//! # // Dumb imitation of LinkedHashMap.
//! # #[derive(PartialEq, Debug)]
//! # struct LinkedHashMap<K, V>(Vec<(K, V)>);
//! #
//! # impl<K, V> LinkedHashMap<K, V> {
//! #     fn new() -> Self {
//! #         LinkedHashMap(Vec::new())
//! #     }
//! #
//! #     fn insert(&mut self, k: K, v: V) {
//! #         self.0.push((k, v));
//! #     }
//! # }
//! #
//! # impl<K, V> Serialize for LinkedHashMap<K, V>
//! # where
//! #     K: Serialize,
//! #     V: Serialize,
//! # {
//! #     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//! #     where
//! #         S: Serializer,
//! #     {
//! #         let mut map = serializer.serialize_map(Some(self.0.len()))?;
//! #         for &(ref k, ref v) in &self.0 {
//! #             map.serialize_entry(k, v)?;
//! #         }
//! #         map.end()
//! #     }
//! # }
//! #
//! # struct LinkedHashMapVisitor<K, V>(PhantomData<(K, V)>);
//! #
//! # impl<'de, K, V> Visitor<'de> for LinkedHashMapVisitor<K, V>
//! # where
//! #     K: Deserialize<'de>,
//! #     V: Deserialize<'de>,
//! # {
//! #     type Value = LinkedHashMap<K, V>;
//! #
//! #     fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
//! #     where
//! #         M: MapAccess<'de>,
//! #     {
//! #         let mut map = LinkedHashMap::new();
//! #         while let Some((key, value)) = access.next_entry()? {
//! #             map.insert(key, value);
//! #         }
//! #         Ok(map)
//! #     }
//! # }
//! #
//! # impl<'de, K, V> Deserialize<'de> for LinkedHashMap<K, V>
//! # where
//! #     K: Deserialize<'de>,
//! #     V: Deserialize<'de>,
//! # {
//! #     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//! #     where
//! #         D: Deserializer<'de>,
//! #     {
//! #         deserializer.deserialize_map(LinkedHashMapVisitor(PhantomData))
//! #     }
//! # }
//! #
//! #[test]
//! # fn not_a_test_ser_de_empty() {}
//! fn test_ser_de_empty() {
//!     let map = LinkedHashMap::<char, u32>::new();
//!
//!     assert_tokens(
//!         &map,
//!         &[
//!             Token::Map { len: Some(0) },
//!             Token::MapEnd,
//!         ],
//!     );
//! }
//!
//! #[test]
//! # fn not_a_test_ser_de() {}
//! fn test_ser_de() {
//!     let mut map = LinkedHashMap::new();
//!     map.insert('b', 20);
//!     map.insert('a', 10);
//!     map.insert('c', 30);
//!
//!     assert_tokens(
//!         &map,
//!         &[
//!             Token::Map { len: Some(3) },
//!             Token::Char('b'),
//!             Token::I32(20),
//!             Token::Char('a'),
//!             Token::I32(10),
//!             Token::Char('c'),
//!             Token::I32(30),
//!             Token::MapEnd,
//!         ],
//!     );
//! }
//! #
//! # fn main() {
//! #     test_ser_de_empty();
//! #     test_ser_de();
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/serde_test/1.0.177")]
// Ignored clippy lints
#![allow(clippy::float_cmp, clippy::needless_doctest_main)]
// Ignored clippy_pedantic lints
#![allow(
    clippy::manual_assert,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

mod assert;
mod configure;
mod de;
mod error;
mod ser;
mod token;

pub use crate::assert::{
    assert_de_tokens, assert_de_tokens_error, assert_ser_tokens, assert_ser_tokens_error,
    assert_tokens,
};
pub use crate::configure::{Compact, Configure, Readable};
pub use crate::token::Token;
