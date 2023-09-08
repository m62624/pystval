/*
Here we implement traits for two types of `Rule`, they are string `Rule` and byte `Rule`.
They are necessary to avoid code duplicates. Especially in context_match, where there are several modes
*/

// =======================================================
use super::common_elements::range::*;
use super::{CaptureData, Counter, ModeMatch, RegexRaw};
use crate::prelude::MatchRequirement;
use indexmap::IndexSet;
use std::{fmt::Debug, hash::Hash};
// =======================================================

/// This trait requires implementations of the most basic methods for any `Rule`.
pub trait RuleBase: Hash + PartialEq + Eq {
    type TakeRuleType;
    type SubRulesType;
    type RuleType;
    type RegexSet;

    fn _new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self;
    fn get_str_type(&self) -> &RegexRaw;
    fn get_subrules(&self) -> Option<&Self::SubRulesType>;
    // fn get_simple_rules(&self) -> Option<(&IndexSet<Self::RuleType>, &Self::RegexSet)>;
    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>>;
    /// We do not specify a reference, since it implements `Copy`
    fn get_requirement(&self) -> MatchRequirement;
    /// We do not specify a reference, since it implements `Copy`
    fn get_counter(&self) -> Option<Counter>;
    fn get_mode_match(&self) -> &ModeMatch;
    fn get_str(&self) -> &str;
    fn get_range(&self) -> Option<&RangeFormat>;
    fn get_save_duplicates(&self) -> bool;
}

/// The main trait for `context_match`, that is,
/// the implementation of the modifier nesting logic will be common for two different rule structures.
/// That is, `next` + `mode matching` will be common for them.
/// The main thing is to implement separately `Captures` for `&str` and `&[u8]`
/// the rest will be the same

pub trait CalculateValueRules<'a, C: IntoSpecificCaptureType<'a>>: Debug {
    type RuleType: RuleBase<RuleType = Self::RuleType, RegexSet = Self::RegexSet>;
    type RegexSet: 'a;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &C) -> Vec<usize>;
    fn find_captures(rule: &Self::RuleType, capture: &C) -> CaptureData<'a, C>;
}

/// This trait requires modifier implementations for any `Rules`
pub trait RuleModifiers {
    /// The type of the rule that will be returned after applying the modifier
    type RuleType;

    /// modifier for extending the rule with nested rules
    ///
    /// ( **by default, `all_rules_for_all_matches`** )\
    /// In this mode, all rules must be tested for all matches
    fn extend<R: IntoIterator<Item = Self::RuleType>>(self, nested_rules: R) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter == match`
    fn counter_is_equal(self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter >= match`
    fn counter_more_than(self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter <= match`
    fn counter_less_than(self, count: usize) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, all rules must pass the test for at least one match
    fn all_r_for_any_m(self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass the test for all matches.
    fn any_r_for_all_m(self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass at least one match check
    fn any_r_for_any_m(self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// Save all matches even if they are duplicates
    /// # Notes
    /// - By default, duplicates are not saved, but duplicate counting for `counter_*` methods is always kept regardless of the duplicate saving mode
    /// - Automatic activated if `number_range` for `RuleBytes` is used
    fn save_duplicates(self) -> Self::RuleType;
}
pub trait RangeType {
    fn get_range(self) -> RangeBoundaries;
}

pub trait IntoSpecificCaptureType<'a>: Copy + Hash + PartialEq + Eq + Debug {
    fn as_str(&self) -> Option<&'a str>;
    fn as_bytes(&self) -> Option<&'a [u8]>;
}
