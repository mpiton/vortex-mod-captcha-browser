use extism_pdk::*;

use crate::{handle_can_solve, interaction_required_response};

#[plugin_fn]
pub fn can_solve(input: String) -> FnResult<String> {
    handle_can_solve(&input).map_err(plugin_error)
}

#[plugin_fn]
pub fn solve(input: String) -> FnResult<String> {
    interaction_required_response(&input).map_err(plugin_error)
}

fn plugin_error(message: String) -> WithReturnCode<extism_pdk::Error> {
    extism_pdk::Error::msg(message).into()
}
