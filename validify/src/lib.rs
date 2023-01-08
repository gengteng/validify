//! ### Example
//!
//! ```
//! use validify::{validify, Validify};
//!
//! #[validify]
//! struct Testor {
//!     #[modify(lowercase, trim)]
//!     #[validate(length(equal = 8))]
//!     pub a: String,
//!     #[modify(trim, uppercase)]
//!     pub b: Option<String>,
//!     #[modify(custom = "do_something")]
//!     pub c: String,
//!     #[modify(custom = "do_something")]
//!     pub d: Option<String>,
//!     #[validify]
//!     pub nested: Nestor,
//! }
//!
//! #[validify]
//! struct Nestor {
//!     #[modify(trim, uppercase)]
//!     #[validate(length(equal = 12))]
//!     a: String,
//!     #[modify(capitalize)]
//!     #[validate(length(equal = 14))]
//!     b: String,
//! }
//!
//! fn do_something(input: &mut String) {
//!     *input = String::from("modified");
//! }
//!
//! let mut test = Testor {
//!   a: "   LOWER ME     ".to_string(),
//!   b: Some("  makemeshout   ".to_string()),
//!   c: "I'll never be the same".to_string(),
//!   d: Some("Me neither".to_string()),
//!   nested: Nestor {
//!     a: "   notsotinynow   ".to_string(),
//!       b: "capitalize me.".to_string(),
//!   },
//! };
//! // The magic line
//! let res = test.validate();
//!
//! assert!(matches!(res, Ok(())));
//!
//! // Parent
//! assert_eq!(test.a, "lower me");
//! assert_eq!(test.b, Some("MAKEMESHOUT".to_string()));
//! assert_eq!(test.c, "modified");
//! assert_eq!(test.d, Some("modified".to_string()));
//! // Nested
//! assert_eq!(test.nested.a, "NOTSOTINYNOW");
//! assert_eq!(test.nested.b, "Capitalize me.");
//! ```

pub use derive_validify::{validify, Validify};
pub use traits::{Modify, Validify};
