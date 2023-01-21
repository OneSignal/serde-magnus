//! `serde_magnus` converts between Rust and Ruby data structures using [Serde] and [Magnus].
//!
//! [Serde]: https://github.com/serde-rs/serde
//! [Magnus]: https://github.com/matsadler/magnus
//!
//! The [serialize] function converts from a Rust type implementing the [serde::Serialize] trait
//! into a Ruby equivalent.
//!
//! ```
//! use serde::{Serialize, Deserialize};
//! use magnus::{eval, RArray, RHash, RString, Symbol};
//! use serde_magnus::serialize;
//! # let _cleanup = unsafe { magnus::embed::init() };
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Post {
//!     title: String,
//!     content: String,
//!     author: Author,
//!     tags: Vec<String>
//! }
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Author {
//!     name: String,
//!     email_address: String
//! }
//!
//! let post = Post {
//!     title: "Spring carnival planning update".into(),
//!     content: "Here's what's new.".into(),
//!     author: Author {
//!         name: "Martha".into(),
//!         email_address: "martha@example.com".into()
//!     },
//!     tags: vec![
//!         "carnival".into(),
//!         "update".into()
//!     ]
//! };
//!
//! let post: RHash = serialize(&post)?;
//! let title: RString = post.lookup(Symbol::new("title"))?;
//! assert_eq!("Spring carnival planning update", title.to_string()?);
//!
//! let author: RHash = post.lookup(Symbol::new("author"))?;
//! let author_email_address: RString = author.lookup(Symbol::new("email_address"))?;
//! assert_eq!("martha@example.com", author_email_address.to_string()?);
//!
//! let tags: RArray = post.lookup(Symbol::new("tags"))?;
//! assert_eq!(2, tags.len());
//! assert_eq!("carnival", tags.entry::<RString>(0)?.to_string()?);
//! assert_eq!("update", tags.entry::<RString>(1)?.to_string()?);
//!
//! # Ok::<(), magnus::Error>(())
//! ```
//!
//! [deserialize] converts from a Ruby value to a Rust type implementing [serde::Deserialize].
//!
//! ```
//! # use serde::Deserialize;
//! # use magnus::{eval, RHash};
//! # let _cleanup = unsafe { magnus::embed::init() };
//! #
//! # #[derive(Deserialize, Debug)]
//! # struct Post {
//! #     title: String,
//! #     content: String,
//! #     author: Author,
//! #     tags: Vec<String>
//! # }
//! #
//! # #[derive(Deserialize, Debug)]
//! # struct Author {
//! #     name: String,
//! #     email_address: String
//! # }
//! use serde_magnus::deserialize;
//!
//! let post: RHash = eval!(r#"
//!   {
//!     title: "Spring carnival planning update",
//!     content: "Here's what's new.",
//!     author: {
//!       name: "Martha",
//!       email_address: "martha@example.com"
//!     },
//!     tags: ["carnival", "update"]
//!   }
//! "#)?;
//!
//! let post: Post = deserialize(post)?;
//! assert_eq!("Spring carnival planning update", post.title);
//! assert_eq!("martha@example.com", post.author.email_address);
//! assert_eq!(vec!["carnival", "update"], post.tags);
//!
//! # Ok::<(), magnus::Error>(())
//! ```

mod de;
mod error;
mod ser;

pub use de::deserialize;
pub use ser::serialize;
