// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use neon::prelude::FunctionContext;
use serde::de::DeserializeOwned;

/// Unwraps a result or propagates its error.
macro_rules! jstry(
    ($cx:expr, $e:expr) => (match $e { Ok(e) => e, Err(e) => return $cx.throw_error(format!("{}", e)) })
);

/// Gets the argument at the specified index and attempts to convert it to the
/// appropriate type.
pub fn get_argument<'j, V>(cx: &mut FunctionContext, i: i32) -> Result<V, neon::result::Throw>
where
    V: DeserializeOwned + ?Sized,
{
    let arg = cx.argument(i)?;
    let js_options_opt: V = neon_serde::from_value(cx, arg)?;
    Ok(js_options_opt)
}

/// Gets the argument at the specified index and attempts to convert it to the
/// appropriate type. If it doesn't exist, returns the type's `::default()`.
pub fn get_argument_or_default<'j, V>(
    cx: &mut FunctionContext,
    i: i32,
) -> Result<V, neon::result::Throw>
where
    V: DeserializeOwned + ?Sized,
    V: std::default::Default,
{
    let arg = cx.argument_opt(i);
    let js_options_opt: Option<V> = neon_serde::from_value_opt(cx, arg)?;
    Ok(js_options_opt.unwrap_or_default())
}
