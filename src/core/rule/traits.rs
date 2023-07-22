use super::*;

/// Реализация трейта по сравнению элементов
mod partial_eq_eq {

    use super::*;

    impl PartialEq for Subrules {
        fn eq(&self, other: &Self) -> bool {
            self.simple_rules == other.simple_rules && self.complex_rules == other.complex_rules
        }
    }

    impl PartialEq for SimpleRules {
        fn eq(&self, other: &Self) -> bool {
            self.all_rules == other.all_rules
        }
    }

    impl PartialEq for Counter {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Only(l0), Self::Only(r0)) => l0 == r0,
                (Self::MoreThan(l0), Self::MoreThan(r0)) => l0 == r0,
                (Self::LessThan(l0), Self::LessThan(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl PartialEq for ModeMatch {
        fn eq(&self, other: &Self) -> bool {
            mem::discriminant(self) == mem::discriminant(other)
        }
    }

    impl Eq for Subrules {}
    impl Eq for SimpleRules {}
    impl Eq for Counter {}
    impl Eq for ModeMatch {}
}

#[cfg(not(tarpaulin_include))]
/// Реализация трейта для получения ссылки
mod as_ref_str {

    use super::*;

    impl AsRef<str> for RegexRaw {
        fn as_ref(&self) -> &str {
            match self {
                RegexRaw::DefaultRegex(value) => value,
                RegexRaw::FancyRegex(value) => value,
                RegexRaw::DefaultBytes(value) => value,
                RegexRaw::FancyBytes(value) => value,
            }
        }
    }

    impl AsRef<str> for Rule {
        fn as_ref(&self) -> &str {
            self.content_unchecked().str_with_type.as_ref()
        }
    }

    impl AsRef<TakeRuleForExtend> for &TakeRuleForExtend {
        fn as_ref(&self) -> &TakeRuleForExtend {
            self
        }
    }
}

mod hash_set {
    use super::*;

    impl Hash for Rule {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.content_unchecked().hash(state);
        }
    }

    impl Hash for SimpleRules {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
            self.all_rules.hasher();
        }
    }
}
