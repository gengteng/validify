use validify::Validate;

const MIN_CONST: u64 = 1;
const MAX_CONST: u64 = 10;

const MAX_CONST_I32: i32 = 2;
const NEGATIVE_CONST_I32: i32 = -10;

#[test]
fn can_validate_length_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let s = TestStruct {
        val: "hello".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn validate_length_with_ref_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = MIN_CONST, max = MAX_CONST))]
        val: String,
    }

    let s = TestStruct {
        val: "hello".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn validate_length_with_ref_fails() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = MIN_CONST, max = MAX_CONST))]
        val: String,
    }

    let s = TestStruct {
        val: "".to_string(),
    };

    assert!(s.validate().is_err());
}

#[test]
fn validate_length_with_ref_i32_fails() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(max = MAX_CONST_I32))]
        val: String,
    }

    let s = TestStruct {
        val: "TO_LONG_YAY".to_string(),
    };

    assert!(s.validate().is_err());
}

#[test]
fn validate_length_with_ref_negative_i32_fails() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(max = NEGATIVE_CONST_I32))]
        val: String,
    }

    let s = TestStruct {
        val: "TO_LONG_YAY".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn value_out_of_length_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code(), "length");
    assert_eq!(errs[0].params()["actual"], "");
    assert_eq!(errs[0].params()["min"], 5);
    assert_eq!(errs[0].params()["max"], 10);
}

#[test]
fn can_specify_code_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10, code = "oops"))]
        val: String,
    }
    let s = TestStruct { val: String::new() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code(), "oops");
}

#[test]
fn can_specify_message_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10, message = "oops"))]
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

#[test]
fn can_validate_array_for_length() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5, max = 10))]
        val: [String; 1],
    }

    let s = TestStruct {
        val: [String::new()],
    };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code(), "length");
    assert_eq!(errs[0].params()["min"], 5);
    assert_eq!(errs[0].params()["max"], 10);
}
