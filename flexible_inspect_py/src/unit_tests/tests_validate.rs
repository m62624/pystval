use crate::{
    PyCartridge, PyCartridgeBytes, PyMatchRequeriment, PyRule, PyRuleBytes, PyTemplateValidator,
    PyTemplateValidatorBytes,
};

#[test]
fn test_validate_t_0() {
    let cartrdige_1 = PyCartridge::new(
        1,
        "message_1".into(),
        vec![PyRule::new(
            r"ABC".into(),
            PyMatchRequeriment::MustNotBeFound,
        )],
    );

    let cartridge_2 = PyCartridge::new(
        2,
        "message_2".into(),
        vec![PyRule::new(r"\d+".into(), PyMatchRequeriment::MustBeFound)],
    );
    let validator = PyTemplateValidator::new(vec![cartrdige_1, cartridge_2]);
    assert!(validator.validate("123".to_string()).unwrap().0.is_empty());
    assert!(!validator.validate("ABC 123".to_string()).unwrap().0.is_empty());
    assert_eq!(validator.validate("ABC".to_string()).unwrap().0.len(), 2);
}

#[test]
fn test_validate_t_1() {
    let cartrdige_1 = PyCartridgeBytes::new(
        1,
        "message_1".into(),
        vec![PyRuleBytes::new(
            r"ABC".into(),
            PyMatchRequeriment::MustNotBeFound,
        )],
    );

    let cartridge_2 = PyCartridgeBytes::new(
        2,
        "message_2".into(),
        vec![PyRuleBytes::new(
            r"\d+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    );
    let validator = PyTemplateValidatorBytes::new(vec![cartrdige_1, cartridge_2]);
    assert!(validator.validate("123".as_bytes()).unwrap().0.is_empty());
    assert!(!validator
        .validate("ABC 123".as_bytes())
        .unwrap()
        .0
        .is_empty());
    assert_eq!(validator.validate("ABC".as_bytes()).unwrap().0.len(), 2);
}
