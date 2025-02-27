use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validify::{Validate, ValidationError, ValidationErrors};

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        return Err(ValidationError::new_field("terrible_username"));
    }

    Ok(())
}

fn validate_signup(data: &SignupData) -> Result<(), ValidationErrors> {
    let mut errs = ValidationErrors::new();
    if data.mail.ends_with("gmail.com") && data.age == 18 {
        errs.add(ValidationError::new_schema("stupid_rule"));
    }
    if errs.is_empty() {
        return Ok(());
    }
    Err(errs)
}

#[derive(Debug, Validate, Deserialize)]
#[validate(validate_signup)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom(validate_unique_username))]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18., max = 20.))]
    age: u32,
    #[validate]
    phone: Phone,
    #[validate]
    card: Option<Card>,
    #[validate]
    preferences: Vec<Preference>,
}

#[derive(Debug, Validate, Deserialize)]
struct Phone {
    #[validate(phone)]
    number: String,
}

#[derive(Debug, Validate, Deserialize)]
struct Card {
    #[validate(credit_card)]
    number: String,
    #[validate(range(min = 100., max = 9999.))]
    cvv: u32,
}

#[allow(dead_code)]
#[derive(Debug, Validate, Deserialize)]
struct Preference {
    #[validate(length(min = 4))]
    name: String,
    value: bool,
}

#[test]
fn is_fine_with_many_valid_validations() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "Bob".to_string(),
        age: 18,
        phone: Phone {
            number: "+14152370800".to_string(),
        },
        card: Some(Card {
            number: "5236313877109142".to_string(),
            cvv: 123,
        }),
        preferences: vec![Preference {
            name: "marketing".to_string(),
            value: false,
        }],
    };

    assert!(signup.validate().is_ok());
}

#[test]
fn failed_validation_points_to_original_field_name() {
    let signup = SignupData {
        mail: "bob@bob.com".to_string(),
        site: "http://hello.com".to_string(),
        first_name: "".to_string(),
        age: 18,
        phone: Phone {
            number: "123 invalid".to_string(),
        },
        card: Some(Card {
            number: "1234567890123456".to_string(),
            cvv: 1,
        }),
        preferences: vec![Preference {
            name: "abc".to_string(),
            value: true,
        }],
    };
    let res = signup.validate().unwrap_err();
    assert_eq!(res.errors()[0].location(), "/firstName");
    assert_eq!(res.errors()[0].params()["actual"], "");
    assert_eq!(res.errors()[1].location(), "/phone/number");
    assert_eq!(res.errors()[1].params()["actual"], "123 invalid");
    assert_eq!(res.errors()[2].location(), "/card/number");
    assert_eq!(res.errors()[2].params()["actual"], "1234567890123456");
    assert_eq!(res.errors()[3].location(), "/card/cvv");
    assert_eq!(res.errors()[3].params()["actual"], 1);
    assert_eq!(res.errors()[3].params()["min"], 100.);
    assert_eq!(res.errors()[3].params()["max"], 9999.);
    assert_eq!(res.errors()[4].location(), "/preferences/0/name");
    assert_eq!(res.errors()[4].params()["actual"], "abc");
}

#[test]
fn test_can_validate_option_fields_with_lifetime() {
    lazy_static! {
        static ref RE2: Regex = Regex::new(r"[a-z]{2}").unwrap();
    }

    #[derive(Debug, Validate)]
    struct PutStruct<'a> {
        #[validate(length(min = 1, max = 10))]
        name: Option<&'a str>,
        #[validate(length(min = 1, max = 10))]
        address: Option<Option<&'a str>>,
        #[validate(range(min = 1., max = 100.))]
        age: Option<Option<usize>>,
        #[validate(range(min = 1., max = 10.))]
        range: Option<usize>,
        #[validate(email)]
        email: Option<&'a str>,
        #[validate(url)]
        url: Option<&'a str>,
        #[validate(contains("@"))]
        text: Option<&'a str>,
        #[validate(regex(RE2))]
        re: Option<&'a str>,
        #[validate(custom(check_str))]
        custom: Option<&'a str>,
    }

    fn check_str(_: &str) -> Result<(), ValidationError> {
        Ok(())
    }

    let s = PutStruct {
        name: Some("al"),
        address: Some(Some("gol")),
        age: Some(Some(20)),
        range: Some(2),
        email: Some("hi@gmail.com"),
        url: Some("http://google.com"),
        text: Some("@someone"),
        re: Some("hi"),
        custom: Some("hey"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn test_can_validate_option_fields_without_lifetime() {
    lazy_static! {
        static ref RE2: Regex = Regex::new(r"[a-z]{2}").unwrap();
    }

    #[derive(Debug, Validate)]
    struct PutStruct {
        #[validate(length(min = 1, max = 10))]
        name: Option<String>,
        #[validate(length(min = 1, max = 10))]
        address: Option<Option<String>>,
        #[validate(length(min = 1, max = 10))]
        ids: Option<Vec<usize>>,
        #[validate(length(min = 1, max = 10))]
        opt_ids: Option<Option<Vec<usize>>>,
        #[validate(range(min = 1., max = 100.))]
        age: Option<Option<usize>>,
        #[validate(range(min = 1., max = 10.))]
        range: Option<usize>,
        #[validate(email)]
        email: Option<String>,
        #[validate(url)]
        url: Option<String>,
        #[validate(contains("@"))]
        text: Option<String>,
        #[validate(regex(RE2))]
        re: Option<String>,
        #[validate(custom(check_str))]
        custom: Option<String>,
    }

    fn check_str(_: &str) -> Result<(), ValidationError> {
        Ok(())
    }

    let s = PutStruct {
        name: Some("al".to_string()),
        address: Some(Some("gol".to_string())),
        ids: Some(vec![1, 2, 3]),
        opt_ids: Some(Some(vec![1, 2, 3])),
        age: Some(Some(20)),
        range: Some(2),
        email: Some("hi@gmail.com".to_string()),
        url: Some("http://google.com".to_string()),
        text: Some("@someone".to_string()),
        re: Some("hi".to_string()),
        custom: Some("hey".to_string()),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn test_works_with_question_mark_operator() {
    fn some_fn() -> Result<(), ValidationErrors> {
        let signup = SignupData {
            mail: "invalid_email".to_string(),
            site: "http://hello.com".to_string(),
            first_name: "Bob".to_string(),
            age: 18,
            phone: Phone {
                number: "+14152370800".to_string(),
            },
            card: None,
            preferences: Vec::new(),
        };

        signup.validate()?;
        Ok(())
    }

    assert!(some_fn().is_err());
}

#[test]
fn test_works_with_none_values() {
    #[derive(Debug, Validate)]
    struct PutStruct {
        #[validate(length(min = 1, max = 10))]
        name: Option<String>,
        #[validate(length(min = 1, max = 10))]
        address: Option<Option<String>>,
        #[validate(range(min = 1., max = 100.))]
        age: Option<Option<usize>>,
        #[validate(range(min = 1., max = 10.))]
        range: Option<usize>,
    }

    let p = PutStruct {
        name: None,
        address: None,
        age: None,
        range: None,
    };

    let q = PutStruct {
        name: None,
        address: Some(None),
        age: Some(None),
        range: None,
    };

    assert!(p.validate().is_ok());
    assert!(q.validate().is_ok());
}
