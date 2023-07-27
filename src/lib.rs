/*
                                                                  tttt                                               lllllll
                                                               ttt:::t                                               l:::::l
                                                               t:::::t                                               l:::::l
                                                               t:::::t                                               l:::::l
ppppp   pppppppppyyyyyyy           yyyyyyy  ssssssssss   ttttttt:::::tttttttvvvvvvv           vvvvvvvaaaaaaaaaaaaa    l::::l
p::::ppp:::::::::py:::::y         y:::::y ss::::::::::s  t:::::::::::::::::t v:::::v         v:::::v a::::::::::::a   l::::l
p:::::::::::::::::py:::::y       y:::::yss:::::::::::::s t:::::::::::::::::t  v:::::v       v:::::v  aaaaaaaaa:::::a  l::::l
pp::::::ppppp::::::py:::::y     y:::::y s::::::ssss:::::stttttt:::::::tttttt   v:::::v     v:::::v            a::::a  l::::l
 p:::::p     p:::::p y:::::y   y:::::y   s:::::s  ssssss       t:::::t          v:::::v   v:::::v      aaaaaaa:::::a  l::::l
 p:::::p     p:::::p  y:::::y y:::::y      s::::::s            t:::::t           v:::::v v:::::v     aa::::::::::::a  l::::l
 p:::::p     p:::::p   y:::::y:::::y          s::::::s         t:::::t            v:::::v:::::v     a::::aaaa::::::a  l::::l
 p:::::p    p::::::p    y:::::::::y     ssssss   s:::::s       t:::::t    tttttt   v:::::::::v     a::::a    a:::::a  l::::l
 p:::::ppppp:::::::p     y:::::::y      s:::::ssss::::::s      t::::::tttt:::::t    v:::::::v      a::::a    a:::::a l::::::l
 p::::::::::::::::p       y:::::y       s::::::::::::::s       tt::::::::::::::t     v:::::v       a:::::aaaa::::::a l::::::l
 p::::::::::::::pp       y:::::y         s:::::::::::ss          tt:::::::::::tt      v:::v         a::::::::::aa:::al::::::l
 p::::::pppppppp        y:::::y           sssssssssss              ttttttttttt         vvv           aaaaaaaaaa  aaaallllllll
 p:::::p               y:::::y
 p:::::p              y:::::y
p:::::::p            y:::::y
p:::::::p           y:::::y
p:::::::p          yyyyyyy
ppppppppp

:D
*/
//! Library for data validation based on custom regular expressions with modifiers

// =====================================================================
mod core;
mod export_lang;
#[cfg(test)]
mod unit_tests;
// =====================================================================
use std::sync::Once;
// =====================================================================
pub mod prelude {
    pub use crate::core::cartridges::*;
    pub use crate::core::rules::rule_bytes::RuleBytes;
    pub use crate::core::rules::rule_str::Rule;
    pub use crate::core::rules::traits::RuleModifiers;
    pub use crate::core::rules::MatchRequirement;
    pub use crate::core::validator::*;
}
// =====================================================================
/// For one-time initialization to the logger
static INIT: Once = Once::new();
// =====================================================================
/// Initialization of the logger (disable backtrace)
#[cfg(not(tarpaulin_include))]
pub fn init_logger() {
    // env_logger is called only once
    INIT.call_once(|| {
        env_logger::init();
    });
}
