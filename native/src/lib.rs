use neon::prelude::*;
use usvg;
use options::*;
use fonts::*;

mod options;
mod fonts;

fn parse_color(hex: Option<String>) -> Result<Option<usvg::Color>, svgtypes::Error> {
    hex.map(|p| p.parse::<usvg::Color>()).transpose()
}

macro_rules! jstry(
    ($cx:expr, $e:expr) => (match $e { Ok(e) => e, Err(e) => return $cx.throw_error(format!("{}", e)) })
);

fn copy_vec_to_buffer(mut cx: FunctionContext, vec: Vec<u8>) -> JsResult<JsBuffer> {
    let mut buf = cx.buffer(vec.len() as u32)?;
    cx.borrow_mut(&mut buf, |data| {
        data.as_mut_slice::<u8>().copy_from_slice(&vec)
    });
    Ok(buf)
}

fn render(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let svg_data = cx.argument::<JsString>(0)?;
    let svg_options_argument = cx.argument_opt(1);

    let js_options = match svg_options_argument {
        Some(options) => neon_serde::from_value(&mut cx, options)?,
        None => JsOptions::new(),
    };

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

    let svg_fit_to = match js_options.fit_to {
        JsFitTo::Original => usvg::FitTo::Original,
        JsFitTo::Height { value } => usvg::FitTo::Height(value),
        JsFitTo::Width { value } => usvg::FitTo::Width(value),
        JsFitTo::Zoom { value } => usvg::FitTo::Zoom(value),
    };

    let background = jstry!(cx, parse_color(js_options.background));

    let tree = jstry!(
        cx,
        usvg::Tree::from_str(svg_data.value().as_str(), &svg_options)
    );

    let image = resvg::render(&tree, svg_fit_to, background);

    match image {
        Some(image) => {
            let mut buffer: Vec<u8> = vec![];
            jstry!(cx, image.write_png(&mut buffer));
            copy_vec_to_buffer(cx, buffer)
        },
        None => cx.buffer(0)
    }
}

fn render_node(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("render", render)?;
    cx.export_function("render_node", render_node)?;
    Ok(())
});
