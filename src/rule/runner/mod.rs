use super::captures::CaptureData;
use super::next::NextStep;
use super::*;
use log::{info, trace};
use std::collections::VecDeque;
mod context_match;

impl Rule {
    /// Главный метод для итерационного запуска правила
    /// Проходит по всем подправилам, и так до самого конца для каждого `Rule`
    ///
    /// **Этап 1**\
    /// У каждого правила, сначала идет проверка по `RegexSet`, наша задача
    /// вернуть ошибку валидаций, если хоть одно правило не прошло.
    /// А значит, в наших же интересах, чтобы сразу проверить, то что уже найдено
    /// через `RegexSet` (доп. инфа в `./rule/mod.rs`)
    /// Тем самым, мы стараемся максимально быстро  узнать резульат от самых простых правил
    /// (под простыми, имееются ввиду правила, которые сразу попали в `RegexSet`)
    ///
    /// **Этап 2**\
    /// Дальше идут те правила которые не попали в `RegexSet`, но так и должно быть.
    /// Ведь `RegexSet` отбирает те, что точно нашлись и только лишь потом проверяются их модификаторы.
    /// А не отобранные, могут иметь те же модификтаоры `MustBeFound` и MustNotBefound. Ведь если какой-то паттерн не найден и указан `MustBeFound` то это ошибка, потому мы их проверяем во втором этапе.
    ///
    /// **Этап 3**\
    /// Дальше мы проверяем `FancyRegex`, в котором есть опережающие и ретроспективные проверки (lookaround & backreferences), такие правила могут дольше обрабатываться.
    /// Конечно, зависит от самого паттерна, но в целом, они могут быть дольше, чем обычные правила.
    /// Поэтому мы их оствавляем под конец, чтобы сразу попытаться отсеять долгие вычисления в начале очереди
    pub fn run(rule: &Rule, text: &str) -> NextStep {
        let mut stack = VecDeque::from([(rule, CaptureData::find_captures(rule, text))]);
        while let Some(frame) = stack.front() {
            trace!(
                "loading rules from top-level stack : `{}`, mode `{:#?}`",
                frame.0.as_ref(),
                frame.0.content_unchecked().mod_match
            );
            match frame.0.content_unchecked().mod_match {
                ModeMatch::AllRulesForAllMatches => {
                    trace!(
                        "loading mode AllRulesForAllMatches for the rule `{}`",
                        frame.0.as_ref()
                    );
                    if let NextStep::Error(v) = Self::all_rules_for_all_matches(&mut stack) {
                        return NextStep::Error(v);
                    }
                }
                ModeMatch::AllRulesForAtLeastOneMatch => {
                    trace!(
                        "loading mode AllRulesForAtLeastOneMatch for the rule `{}`",
                        frame.0.as_ref()
                    );
                    if let NextStep::Error(v) = Self::all_rules_for_at_least_one_match(&mut stack) {
                        return NextStep::Error(v);
                    }
                }
                ModeMatch::AtLeastOneRuleForAllMatches => {
                    trace!(
                        "loading mode AtLeastOneRuleForAllMatches for the rule `{}`",
                        frame.0.as_ref()
                    );
                    if let NextStep::Error(v) = Self::at_least_one_rule_for_all_matches(&mut stack)
                    {
                        return NextStep::Error(v);
                    }
                }
                ModeMatch::AtLeastOneRuleForAtLeastOneMatch => {
                    trace!(
                        "loading mode AtLeastOneRuleForAtLeastOneMatch for the rule `{}`",
                        frame.0.as_ref()
                    );
                    if let NextStep::Error(v) =
                        Self::at_least_one_rule_for_at_least_one_match(&mut stack)
                    {
                        return NextStep::Error(v);
                    }
                }
            }
        }
        NextStep::Finish
    }
}
