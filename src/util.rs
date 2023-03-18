use rhai::EvalAltResult;

pub fn convert_to_int(val: impl TryInto<rhai::INT>) -> Result<rhai::INT, Box<EvalAltResult>> {
    val.try_into()
        .map_err(|_| "Error converting number {new_pos} to rhai number type".into())
}
