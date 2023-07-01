use super::captures::MultiCapture;
use super::validator_templates::actions_from_the_requirement::next_or_error;
use super::*;
use std::collections::VecDeque;

impl Rule {
    pub fn run(text: &str, rule: &Rule, class_template: &PyObject) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            let mut stack = VecDeque::from([(rule, text)]);
            while let Some(stack_rule) = stack.pop_front() {
                let captures = MultiCapture::find_captures(stack_rule.0, stack_rule.1)?;
                dbg!(&stack_rule);
                // subrules
                if next_or_error(py, class_template, stack_rule.0, &captures)? {
                    let text_set: Vec<&str> = captures.into();
                    // Простые правила
                    if let Some(simple_rules) = &stack_rule
                        .0
                        .get_content()
                        .unwrap()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .simple_rules
                    {
                        text_set.iter().for_each(|txt| {
                            Rule::get_selected_rules(&simple_rules.regex_set, txt)
                                .iter()
                                .for_each(|index| {
                                    stack.push_back((&simple_rules.all_rules[*index], txt))
                                });
                        });
                        dbg!(":: правила которые были добавлены из regexset:");
                        dbg!(&stack);
                        text_set.iter().for_each(|txt| {
                            simple_rules.all_rules.iter().for_each(|rule| {
                                if !stack.contains(&(rule, txt)) {
                                    stack.push_back((rule, txt));
                                }
                            });
                        });
                        dbg!(":: правила которые были добавлены после regexset:");
                        dbg!(&stack);
                    }
                    if let Some(complex_rules) = &stack_rule
                        .0
                        .get_content()
                        .unwrap()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .complex_rules
                    {
                        text_set.iter().for_each(|txt| {
                            complex_rules
                                .iter()
                                .for_each(|rule| stack.push_back((rule, txt)))
                        });
                    }
                   dbg!(":: последние правила, которые были добавлены из complex rules:");
                    dbg!(&stack);
                }
            }
            Ok(())
        })
    }
}
