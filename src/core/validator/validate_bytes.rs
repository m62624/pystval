use super::*;

#[async_trait]
impl<'a, C, IC> ValidatorBase<RuleBytes, C, IC, &'a [u8]> for TemplateValidator<IC, &'a [u8]>
where
    C: CartridgeBase<RuleBytes, &'a [u8]> + Debug + Sync,
    IC: IntoIterator<Item = C> + AsRef<[C]> + Sync,
{
    fn new(cartridges: IC) -> Self {
        Self {
            cartridges,
            _phantom: PhantomData,
        }
    }

    fn validate(&self, data: &'a [u8]) -> Result<(), Vec<PystvalError>> {
        trace!("{:#?}", &self.cartridges.as_ref().iter());
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                error.push(PystvalError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                ));
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }

    async fn async_validate(&self, data: &'a [u8]) -> Result<(), Vec<PystvalError>> {
        self.validate(data)
    }
}
