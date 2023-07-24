use super::*;
use crate::core::rules::traits::{RuleBase, RuleExtendBase};

impl RuleBase for Rule {
    type TakeRuleType = TakeRuleForExtend;
    type SubRulesType = Subrules;
    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &Self::TakeRuleType {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType {
        self.0.as_mut().expect(ERR_OPTION)
    }
    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.content_unchecked().subrules.as_ref()
    }
}

impl<'a> RuleExtendBase<'a> for Rule {
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect()
    }

    fn find_captures(rule: &Rule, text: &'a str) -> CaptureData<'a> {
        captures::find_captures(rule, text)
    }
}
