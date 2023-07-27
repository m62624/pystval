// =======================================================
mod init;
pub mod next;
pub mod rule_bytes;
pub mod rule_str;
pub mod runner;
pub mod traits;
// =======================================================
use super::*;
use crate::prelude::Rule;
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
// =======================================================

/// The struct for sorting all nested rules
pub struct SlisedRules {
    /// The rules are in the `IndexSet` collection to preserve
    /// the order of the rules during index retrieval from the `RegexSet` and to avoid duplicate rules
    pub simple_rules: IndexSet<Rule>,
    pub complex_rules: IndexSet<Rule>,
}

/// A Structure for common `Rule` modifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralModifiers {
    pub requirement: MatchRequirement,
    pub counter: Option<Counter>,
    pub mod_match: ModeMatch,
}

/// A structure that defines what action is required when finding regular expression matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

/// A structure defining the operation mode of the validator subrules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModeMatch {
    AllRulesForAllMatches,
    AllRulesForAtLeastOneMatch,
    AtLeastOneRuleForAllMatches,
    AtLeastOneRuleForAtLeastOneMatch,
}

/// A structure for realization of modifier-counters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Counter {
    Only(usize),
    MoreThan(usize),
    LessThan(usize),
}

/// A structure that stores all the data for processing the capture
#[derive(Debug)]
pub struct CaptureData<T: PartialEq + Eq + Hash> {
    pub text_for_capture: HashSet<T>,
    pub hashmap_for_error: HashMap<String, String>,
    pub counter_value: usize,
}
