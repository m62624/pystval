use crate::{prelude::*, rules::traits::RuleBase, rules::ModeMatch};

#[test]
fn test_mode_t_0() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    );
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AllRulesForAllMatches
    );
}

#[test]
fn test_mode_t_1() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    )
    .mode_all_rules_for_at_least_one_match();
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AllRulesForAtLeastOneMatch
    );
}

#[test]
fn test_mode_t_2() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    )
    .mode_at_least_one_rule_for_all_matches();
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AtLeastOneRuleForAllMatches
    );
}

#[test]
fn test_mode_t_3() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    )
    .mode_at_least_one_rule_for_at_least_one_match();
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AtLeastOneRuleForAtLeastOneMatch
    );
}

#[test]
fn test_mode_t_4() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
    );
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AllRulesForAllMatches
    );
}

#[test]
fn test_mode_t_5() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
    )
    .mode_all_rules_for_at_least_one_match();
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AllRulesForAtLeastOneMatch
    );
}

#[test]
fn test_mode_t_6() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
    )
    .mode_at_least_one_rule_for_all_matches();
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AtLeastOneRuleForAllMatches
    );
}

#[test]
fn test_mode_t_7() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
    )
    .mode_at_least_one_rule_for_at_least_one_match();
    assert_eq!(
        cartridge
            .root_rule
            .content_unchecked()
            .general_modifiers
            .mod_match,
        ModeMatch::AtLeastOneRuleForAtLeastOneMatch
    );
}
