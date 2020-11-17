extern crate neon_serde;
use serde::de::DeserializeOwned;
use serde_bytes;

use fonts::*;
use neon::prelude::*;
use options::*;
use usvg;

mod fonts;
mod options;

fn parse_color(hex: &Option<String>) -> Result<Option<usvg::Color>, svgtypes::Error> {
    hex.as_ref().map(|p| p.parse::<usvg::Color>()).transpose()
}

macro_rules! jstry(
    ($cx:expr, $e:expr) => (match $e { Ok(e) => e, Err(e) => return $cx.throw_error(format!("{}", e)) })
);

fn get_argument<'j, V>(cx: &mut FunctionContext, i: i32) -> Result<V, neon::result::Throw>
where
    V: DeserializeOwned + ?Sized
{
    let arg = cx.argument(i)?;
    let js_options_opt: V = neon_serde::from_value(cx, arg)?;
    Ok(js_options_opt)
}

fn get_argument_or_default<'j, V>(
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

fn render(mut cx: FunctionContext) -> JsResult<JsValue> {
    let svg_data: String = get_argument(&mut cx, 0)?;
    let js_options: JsOptions = get_argument_or_default(&mut cx, 1)?;

    let background = jstry!(cx, parse_color(&js_options.background));
    let fontdb = load_fonts(&js_options.font);

    let svg_options = usvg::Options {
        path: js_options.path.map(|p| p.into()),
        dpi: js_options.dpi,
        font_family: js_options.font.default_font_family,
        font_size: js_options.font.default_font_size,
        languages: js_options.languages,
        shape_rendering: js_options.shape_rendering,
        text_rendering: js_options.text_rendering,
        image_rendering: js_options.image_rendering,
        keep_named_groups: false,
        fontdb,
    };
    let tree = jstry!(cx, usvg::Tree::from_str(&svg_data, &svg_options));

    let image = resvg::render(&tree, js_options.fit_to, background);

    let mut buffer: Vec<u8> = vec![];

    match image {
        Some(image) => {
            jstry!(cx, image.write_png(&mut buffer));
        },
        _ => {}
    }

    let bytes = serde_bytes::ByteBuf::from(buffer);
    Ok(neon_serde::to_value(&mut cx, &bytes)?)
}

fn render_node(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("render", render)?;
    cx.export_function("render_node", render_node)?;
    Ok(())
});
