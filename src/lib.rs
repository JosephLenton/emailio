//!
//! This crate is for creating `Email` objects.
//!
//!  * It allows you to have Email **as a type**. i.e. `let emails : Vec<Email> = vec![]`.
//!  * The `Email` type guarantees to be **structurally validated**. Once it is created, you can be confident it's safe to use as an email.
//!  * The `Email` type can also be **used as strings**. This allows interoptability with lots of connector functions which will take a String.
//!  * It **supports Serde** out of the box. For Serialisation with CLIs, requests, etc.
//!
//! (Note this library will not check if the Email address exists. It only validates that it looks correct.)
//!
//! ## Features
//!
//!  * `serde` **Default** - Enables serde serialisation and deserialisation.
//!  * `sea-orm` - Enables Sea Orm use with DB entities.
//!
//! ## Usage
//!
//! ### Building your own email addresses
//!
//! ```rust
//! use ::serde_email::Email;
//!
//! let email = Email::from_str("test@example.com").expect("A valid email address");
//! ```
//!
//! ### Validating the email address yourself
//!
//! ```rust
//! use ::serde_email::is_valid_email;
//!
//! if is_valid_email(&"test@example.com") {
//!   // do something
//! }
//! ```
//!
//! ### Serialisation / Deserialisation
//!
//! ```rust
//! use ::serde_email::Email;
//! use ::serde::Deserialize;
//! use ::serde::Serialize;
//! use ::serde_json;
//!
//! #[derive(Deserialize, Serialize)]
//! struct Person {
//!   name: String,
//!   email: Email,
//! }
//!
//! // Some JSON input data as a &str. Maybe this comes from the user.
//! let data = r#"
//!     {
//!         "name": "John Doe",
//!         "email": "john@example.com"
//!     }"#;
//!
//! // Parse the string of data into serde_json::Value.
//! let person: Person = serde_json::from_str(data).unwrap();
//!
//! // Access parts of the data by indexing with square brackets.
//! println!("Hello {} I'll email you are {}", person.name, person.email);
//! ```
//!
//! ### Sea Orm Entities
//!
//! You can use the `Email` type with Sea Orm, including using it to save data to the DB.
//! Underneath it will serialise to a `Text` type within the DB.
//!
//! **Required**, the `sea-orm` feature must be enabled for Sea Orm support.
//!
//! ```rust
//! use ::sea_orm::entity::prelude::*;
//! use ::serde::Deserialize;
//! use ::serde::Serialize;
//! use ::serde_email::Email;
//!
//! #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
//! #[sea_orm(table_name = "user")]
//! pub struct Model {
//!     #[sea_orm(primary_key)]
//!     pub id: i32,
//!     pub email: Email, // use as an email field
//! }
//!
//! #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
//! pub enum Relation {}
//!
//! impl ActiveModelBehavior for ActiveModel {}
//! ```
//!

mod email;
pub use self::email::*;

mod email_error;
pub use self::email_error::*;

mod is_valid_email;
pub use self::is_valid_email::*;
