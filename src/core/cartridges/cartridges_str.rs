use super::*;

impl CartridgeBase<Rule, &str> for Cartridge<Rule> {
    fn id(&self) -> i64 {
        self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn root_rule(&self) -> &Rule {
        &self.root_rule
    }

    fn run(&mut self, data: &str) -> NextStep {
        rules::runner::run::<Rule, &str>(
            &self.root_rule,
            CaptureData {
                text_for_capture: HashSet::from([data]),
                hashmap_for_error: Default::default(),
                counter_value: Default::default(),
            },
        )
    }
}
