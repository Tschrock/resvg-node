// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # resvg-node
//!
//! Provides a Node.js api for rendering SVGs using resvg.

mod fonts;
#[macro_use]
mod neon_utils;
mod options;

extern crate neon_serde;
use neon::prelude::*;

/// Trys to parse an `Option<String>` into an `Option<usvg::Color>`
fn parse_color(value: &Option<String>) -> Result<Option<usvg::Color>, svgtypes::Error> {
    value.as_ref().map(|p| p.parse::<usvg::Color>()).transpose()
}

/// Renders an SVG
fn render(mut cx: FunctionContext) -> JsResult<JsValue> {
    let svg_data: String = neon_utils::get_argument(&mut cx, 0)?;
    let js_options: options::JsOptions = neon_utils::get_argument_or_default(&mut cx, 1)?;

    // Parse the background
    let background = jstry!(cx, parse_color(&js_options.background));

    // Load fonts
    let fontdb = fonts::load_fonts(&js_options.font);

    // Build the SVG options
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

    // Parse the SVG string into a tree.
    let tree = jstry!(cx, usvg::Tree::from_str(&svg_data, &svg_options));

    // Render the tree
    let image = resvg::render(&tree, js_options.fit_to, background);

    // Write the image data to a buffer
    let mut buffer: Vec<u8> = vec![];
    if let Some(image) = image {
        jstry!(cx, image.write_png(&mut buffer));
    }

    // Convert to a JsBuffer and return
    let bytes = serde_bytes::ByteBuf::from(buffer);
    Ok(neon_serde::to_value(&mut cx, &bytes)?)
}

register_module!(mut cx, {
    cx.export_function("render", render)?;
    Ok(())
});
