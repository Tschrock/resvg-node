use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(
    tag = "mode",
    content = "value",
    rename_all = "lowercase",
    deny_unknown_fields,
    remote = "usvg::FitTo"
)]
pub enum FitToDef {
    Original,
    Width(u32),
    Height(u32),
    Zoom(f32),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
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
    #[serde(with = "FitToDef")]
    pub fit_to: usvg::FitTo,
    pub background: Option<String>,
}

impl Default for JsOptions {
    fn default() -> JsOptions {
        JsOptions {
            path: None,
            font: JsFontOptions::default(),
            dpi: 96.0,
            languages: vec!["en".to_string()],
            shape_rendering: usvg::ShapeRendering::default(),
            text_rendering: usvg::TextRendering::default(),
            image_rendering: usvg::ImageRendering::default(),
            fit_to: usvg::FitTo::Original,
            background: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
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

impl Default for JsFontOptions {
    fn default() -> JsFontOptions {
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

fn deserialize_shape_rendering<'de, D>(deserializer: D) -> Result<usvg::ShapeRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::ShapeRendering::OptimizeSpeed),
        1 => Ok(usvg::ShapeRendering::CrispEdges),
        2 => Ok(usvg::ShapeRendering::GeometricPrecision),
        n => Err(serde::de::Error::custom(format_args!(
            "Invalid ShapeRendering value: {}. Expected 0 (OptimizeSpeed), 1 (CrispEdges), or 2 (GeometricPrecision).",
            n
        ))),
    }
}

fn deserialize_text_rendering<'de, D>(deserializer: D) -> Result<usvg::TextRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::TextRendering::OptimizeSpeed),
        1 => Ok(usvg::TextRendering::OptimizeLegibility),
        2 => Ok(usvg::TextRendering::GeometricPrecision),
        n => Err(serde::de::Error::custom(format_args!(
            "Invalid TextRendering value: {}. Expected 0 (OptimizeSpeed), 1 (OptimizeLegibility), or 2 (GeometricPrecision).",
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
            "Invalid ImageRendering value: {}. Expected 0 (OptimizeQuality) or 1 (OptimizeSpeed).",
            n
        ))),
    }
}
