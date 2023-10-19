# `serde_magnus`

`serde_magnus` converts between Rust and Ruby data structures using [Serde] and [Magnus].

[Serde]: https://github.com/serde-rs/serde
[Magnus]: https://github.com/matsadler/magnus

**Quick links**

* [API documentation](https://docs.rs/serde_magnus)
* [Releases/changelog](https://github.com/georgeclaghorn/serde-magnus/releases)
* [`serde_magnus` on crates.io](https://crates.io/crates/serde_magnus)

## Usage

The [`serde_magnus::serialize`] function converts from a Rust type implementing the
[`serde::Serialize`] trait into a Ruby equivalent.

```rust
use serde::{Serialize, Deserialize};
use magnus::{eval, Value};
use serde_magnus::serialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Post {
    title: String,
    content: String,
    author: Author,
    tags: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Author {
    name: String,
    email_address: String
}

let post = Post {
    title: "Spring carnival planning update".into(),
    content: "Here's what's new.".into(),
    author: Author {
        name: "Martha".into(),
        email_address: "martha@example.com".into()
    },
    tags: vec![
        "carnival".into(),
        "update".into()
    ]
};

let post: Value = serialize(&post)?;

assert!(eval!(
    r#"
    post == {
      title: "Spring carnival planning update",
      content: "Here's what's new.",
      author: {
        name: "Martha",
        email_address: "martha@example.com"
      },
      tags: ["carnival", "update"]
    }
    "#,
    post
)?);
```

[`serde_magnus::deserialize`] converts from a Ruby value to a Rust type implementing
[`serde::Deserialize`].

```rust
use magnus::RHash;
use serde_magnus::deserialize;

let post: RHash = eval!(r#"
  {
    title: "Spring carnival planning update",
    content: "Here's what's new.",
    author: {
      name: "Martha",
      email_address: "martha@example.com"
    },
    tags: ["carnival", "update"]
  }
"#)?;

let post: Post = deserialize(post)?;

assert_eq!(
    Post {
        title: "Spring carnival planning update".into(),
        content: "Here's what's new.".into(),
        author: Author {
            name: "Martha".into(),
            email_address: "martha@example.com".into(),
        },
        tags: vec![
            "carnival".into(),
            "update".into()
        ]
    },
    post
);
```

[`serde_magnus::serialize`]: https://docs.rs/serde_magnus/latest/serde_magnus/fn.serialize.html
[`serde::Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`serde_magnus::deserialize`]: https://docs.rs/serde_magnus/latest/serde_magnus/fn.deserialize.html
[`serde::Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html

## Requirements

`serde_magnus` requires Rust 1.65+ and Ruby 3.0+.

## License

`serde_magnus` is released under the terms of the MIT License. See `LICENSE` for details.
