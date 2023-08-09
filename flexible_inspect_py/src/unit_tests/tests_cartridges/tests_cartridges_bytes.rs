use super::*;
#[test]
fn test_new_t_0() {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustBeFound)],
    )
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        ),
    )
}