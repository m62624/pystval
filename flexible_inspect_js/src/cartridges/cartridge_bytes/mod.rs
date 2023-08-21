mod modifiers;
use super::*;

#[wasm_bindgen(js_name = "CartridgeBytes")]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmCartridgeBytes(Cartridge<RuleBytes>);

#[wasm_bindgen(js_class = "CartridgeBytes")]
impl WasmCartridgeBytes {
    /// Constructs a new `CartridgeBytes`
    /// # Arguments:
    /// * `error_code` - error code
    /// * `message` - error message
    /// * `root_rules` - \[ `RuleBytes`, `RuleBytes`, `RuleBytes` \] (collection)
    /// # Notes:
    /// * **by default, all rules must pass every match check**
    /// In this mode, to which all additional rules apply (default mode for everyone).
    /// We check that for each match (text) all the rules will work.
    /// ## Operation scheme of the mode
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///      |   [123], [456], [789]
    ///      |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE
    ///      |                                      |       |        |
    ///      |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
    ///
    /// ```
    ///
    /// ## Fill in messages
    /// * Each cartridge supports filling the message with unwanted data, when specifying a message,
    /// you can specify a variable in the message in the format : **`{variable}`**.
    /// After specifying an identical group name in any rule along with the *`MustNotBeFound`* modifier
    #[wasm_bindgen(constructor)]
    pub fn new(
        error_code: i32,
        message: String,
        root_rules: JsValue,
    ) -> Result<WasmCartridgeBytes, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(
            Cartridge::new(error_code, message, serde_wasm_bindgen::from_value::<Vec<RuleBytes>>(root_rules)
        .map_err(|_| JsValue::from_str(" (CartridgeBytes) RuleBytes` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `RuleBytes` ( [ RuleBytes, RuleBytes, RuleBytes ] ) type for the `CartridgeBytes`"))?
        .into_iter()
        ))
    )
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmCartridgeBytes> for Cartridge<RuleBytes> {
    fn from(value: WasmCartridgeBytes) -> Self {
        value.0
    }
}
