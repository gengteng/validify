use validify::Validate;

#[test]
fn can_validate_contains_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains("he"))]
        val: String,
    }

    let s = TestStruct {
        val: "hello".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn value_not_containing_needle_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains("he"))]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code(), "contains");
}

#[test]
fn can_specify_code_for_contains() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains(value = "he", code = "dis dont have he yo"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code(), "dis dont have he yo");
}

#[test]
fn can_specify_message_for_contains() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(contains(value = "he", message = "oops"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].clone().message().unwrap(), "oops");
}
