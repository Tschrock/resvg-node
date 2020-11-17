use neon::prelude::*;
use serde::{Deserialize, Deserializer};
use usvg::{self, SystemFontDB};

#[derive(Deserialize)]
pub struct JsFontOptions {
    pub load_system_fonts: bool,
    pub font_files: Vec<String>,
    pub font_dirs: Vec<String>,
    pub default_font_family: String,
    pub default_font_size: f64,
    pub serif_family: String,
    pub sans_serif_family: String,
    pub cursive_family: String,
    pub fantasy_family: String,
    pub monospace_family: String,
}

impl JsFontOptions {
    fn new() -> JsFontOptions {
        JsFontOptions {
            load_system_fonts: true,
            font_files: vec![],
            font_dirs: vec![],
            default_font_family: "Times New Roman".to_string(),
            default_font_size: 12.0,
            serif_family: "Times New Roman".to_string(),
            sans_serif_family: "Arial".to_string(),
            cursive_family: "Comic Sans MS".to_string(),
            fantasy_family: "Impact".to_string(),
            monospace_family: "Courier New".to_string(),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JsFitTo {
    Original,
    Width { value: u32 },
    Height { value: u32 },
    Zoom { value: f32 },
}

#[derive(Deserialize)]
pub struct JsOptions {
    pub path: Option<String>,
    pub font: JsFontOptions,
    pub dpi: f64,
    pub languages: Vec<String>,
    #[serde(deserialize_with = "deserialize_shape_rendering")]
    pub shape_rendering: usvg::ShapeRendering,
    #[serde(deserialize_with = "deserialize_text_rendering")]
    pub text_rendering: usvg::TextRendering,
    #[serde(deserialize_with = "deserialize_image_rendering")]
    pub image_rendering: usvg::ImageRendering,
    pub fit_to: JsFitTo,
    pub background: Option<String>,
}

impl JsOptions {
    fn new() -> JsOptions {
        JsOptions {
            path: None,
            font: JsFontOptions::new(),
            dpi: 96.0,
            languages: vec!["en".to_string()],
            shape_rendering: usvg::ShapeRendering::GeometricPrecision,
            text_rendering: usvg::TextRendering::OptimizeLegibility,
            image_rendering: usvg::ImageRendering::OptimizeQuality,
            fit_to: JsFitTo::Original,
            background: None,
        }
    }
}

fn deserialize_shape_rendering<'de, D>(deserializer: D) -> Result<usvg::ShapeRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::ShapeRendering::CrispEdges),
        1 => Ok(usvg::ShapeRendering::GeometricPrecision),
        2 => Ok(usvg::ShapeRendering::OptimizeSpeed),
        n => Err(serde::de::Error::custom(format_args!(
            "invalid shape_rendering value: {}, expected 0 through 2",
            n
        ))),
    }
}

fn deserialize_text_rendering<'de, D>(deserializer: D) -> Result<usvg::TextRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::TextRendering::GeometricPrecision),
        1 => Ok(usvg::TextRendering::OptimizeLegibility),
        2 => Ok(usvg::TextRendering::OptimizeSpeed),
        n => Err(serde::de::Error::custom(format_args!(
            "invalid shape_rendering value: {}, expected 0 through 2",
            n
        ))),
    }
}

fn deserialize_image_rendering<'de, D>(deserializer: D) -> Result<usvg::ImageRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::ImageRendering::OptimizeQuality),
        1 => Ok(usvg::ImageRendering::OptimizeSpeed),
        n => Err(serde::de::Error::custom(format_args!(
            "invalid shape_rendering value: {}, expected 0 through 1",
            n
        ))),
    }
}

fn load_fonts(font_options: &JsFontOptions) -> usvg::fontdb::Database {
    let mut fontdb = usvg::fontdb::Database::new();

    if font_options.load_system_fonts {
        fontdb.load_system_fonts();
    }

    for path in &font_options.font_files {
        if let Err(e) = fontdb.load_font_file(path) {
            log::warn!("Failed to load '{}' cause {}.", path, e);
        }
    }

    for path in &font_options.font_dirs {
        fontdb.load_fonts_dir(path);
    }

    fontdb.set_serif_family(&font_options.serif_family);
    fontdb.set_sans_serif_family(&font_options.sans_serif_family);
    fontdb.set_cursive_family(&font_options.cursive_family);
    fontdb.set_fantasy_family(&font_options.fantasy_family);
    fontdb.set_monospace_family(&font_options.monospace_family);

    fontdb
}

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
