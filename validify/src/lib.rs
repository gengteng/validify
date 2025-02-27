#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

mod error;
mod traits;
mod validation;

pub use error::{ValidationError, ValidationErrors};
pub use validation::time;
pub use validation::{
    cards::validate_credit_card,
    contains::validate_contains,
    email::validate_email,
    ip::{validate_ip, validate_ip_v4, validate_ip_v6},
    length::validate_length,
    must_match::validate_must_match,
    non_control_char::validate_non_control_character,
    phone::validate_phone,
    r#in::validate_in,
    range::validate_range,
    required::validate_required,
    urls::validate_url,
};
pub use validify_derive::{schema_err, schema_validation, Payload, Validate, Validify};

/// Deriving [Validate] allows you to specify schema and field validations on structs.
/// See the [repository](https://github.com/biblius/validify) for a full list of possible validations.
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

/// Modifies the struct based on the provided `modify` parameters. Automatically implemented when deriving Validify.
/// See the [repository](https://github.com/biblius/validify) for a full list of possible modifiers.
pub trait Modify {
    /// Apply the provided modifiers to self
    fn modify(&mut self);
}

/// Deriving [Validify] allows you to modify structs before they are validated by providing a out of the box validation implementations
/// as well as the ability to write custom ones.
///
/// ### Example
///
/// ```
/// use validify::Validify;
///
/// #[derive(Debug, Clone, serde::Deserialize, Validify)]
/// struct Testor {
///     #[modify(lowercase, trim)]
///     #[validate(length(equal = 8))]
///     pub a: String,
///     #[modify(trim, uppercase)]
///     pub b: Option<String>,
///     #[modify(custom(do_something))]
///     pub c: String,
///     #[modify(custom(do_something))]
///     pub d: Option<String>,
///     #[validify]
///     pub nested: Nestor,
/// }
///
/// #[derive(Debug, Clone, serde::Deserialize, Validify)]
/// struct Nestor {
///     #[modify(trim, uppercase)]
///     #[validate(length(equal = 12))]
///     a: String,
///     #[modify(capitalize)]
///     #[validate(length(equal = 14))]
///     b: String,
/// }
///
/// fn do_something(input: &mut String) {
///     *input = String::from("modified");
/// }
///
/// let mut test = Testor {
///   a: "   LOWER ME     ".to_string(),
///   b: Some("  makemeshout   ".to_string()),
///   c: "I'll never be the same".to_string(),
///   d: Some("Me neither".to_string()),
///   nested: Nestor {
///     a: "   notsotinynow   ".to_string(),
///       b: "capitalize me.".to_string(),
///   },
/// };
///
/// // The magic line
/// let res = test.validify();
///
/// assert!(matches!(res, Ok(_)));
///
/// // Parent
/// assert_eq!(test.a, "lower me");
/// assert_eq!(test.b, Some("MAKEMESHOUT".to_string()));
/// assert_eq!(test.c, "modified");
/// assert_eq!(test.d, Some("modified".to_string()));
/// // Nested
/// assert_eq!(test.nested.a, "NOTSOTINYNOW");
/// assert_eq!(test.nested.b, "Capitalize me.");
/// ```
pub trait Validify: Modify + Validate {
    /// Apply the provided modifiers to self and run validations.
    fn validify(&mut self) -> Result<(), ValidationErrors>;
}

/// Creates a new field validation error.
/// Serves as a shorthand for writing out errors for custom functions
/// and schema validations.
/// Accepts:
///
/// - `("code")`
/// - `("code", "message")`
/// - `("field_name", "code", "custom message")`
#[macro_export]
macro_rules! field_err {
    ($code:literal) => {
        ::validify::ValidationError::new_field($code)
    };
    ($code:literal, $message:literal) => {
        ::validify::ValidationError::new_field($code).with_message($message.to_string())
    };
    ($field:literal, $code:literal, $message:literal) => {
        ::validify::ValidationError::new_field_named($field, $code)
            .with_message($message.to_string())
    };
}
