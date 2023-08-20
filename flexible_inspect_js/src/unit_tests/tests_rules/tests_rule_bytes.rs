use super::*;

#[wasm_bindgen_test]
pub fn test_new_t_0() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustBeFound).into();

    assert_eq!(rule, RuleBytes::new(r"\w+", MatchRequirement::MustBeFound));
}

#[wasm_bindgen_test]
pub fn test_new_t_1() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound).into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound)
    );
}

#[wasm_bindgen_test]
fn test_extend_t_0() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .extend(
                serde_wasm_bindgen::to_value::<Vec<WasmRuleBytes>>(&vec![WasmRuleBytes::new(
                    String::from(r"\w+"),
                    WasmMatchRequirement::MustNotBeFound,
                )])
                .unwrap(),
            )
            .unwrap()
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).extend(vec![RuleBytes::new(
            r"\w+",
            MatchRequirement::MustNotBeFound
        ),])
    );
}

#[wasm_bindgen_test]
fn test_mode_counter_t_0() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .counter_is_equal(1)
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).counter_is_equal(1)
    );
}

#[wasm_bindgen_test]
fn test_mode_counter_t_1() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .counter_less_than(1)
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).counter_less_than(1)
    );
}

#[wasm_bindgen_test]
fn test_mode_counter_t_2() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .counter_more_than(1)
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).counter_more_than(1)
    );
}

#[wasm_bindgen_test]
fn test_mode_match_t_0() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .all_r_for_any_m()
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).all_r_for_any_m()
    );
}

#[wasm_bindgen_test]
fn test_mode_match_t_1() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .any_r_for_all_m()
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).any_r_for_all_m()
    );
}

#[wasm_bindgen_test]
fn test_mode_match_t_2() {
    let rule: RuleBytes =
        WasmRuleBytes::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
            .any_r_for_any_m()
            .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).any_r_for_any_m()
    );
}
