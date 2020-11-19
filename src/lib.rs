// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # resvg-node
//!
//! Provides a Node.js api for rendering SVGs using resvg.

mod fonts;
mod options;

#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;
use napi::Module;

/// Trys to parse an `Option<String>` into an `Option<usvg::Color>`
fn parse_color(value: &Option<String>) -> Result<Option<usvg::Color>, svgtypes::Error> {
    value.as_ref().map(|p| p.parse::<usvg::Color>()).transpose()
}

/// Renders an SVG
#[js_function(2)]
fn render(ctx: napi::CallContext) -> napi::Result<napi::JsBuffer> {
    let svg_data: String = ctx.get::<napi::JsString>(0)?.into_utf8()?.try_into()?;

    let js_options: options::JsOptions = if ctx.length > 1 {
        ctx.env.from_js_value(ctx.get::<napi::JsUnknown>(1)?)?
    } else {
        options::JsOptions::default()
    };

    // Parse the background
    let background = parse_color(&js_options.background)
        .map_err(|e| napi::Error::from_reason(format!("{}", e)))?;

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
    let tree = usvg::Tree::from_str(&svg_data, &svg_options)
        .map_err(|e| napi::Error::from_reason(format!("{}", e)))?;

    // Render the tree
    let image = resvg::render(&tree, js_options.fit_to, background);

    // Write the image data to a buffer
    let mut buffer: Vec<u8> = vec![];
    if let Some(image) = image {
        image
            .write_png(&mut buffer)
            .map_err(|e| napi::Error::from_reason(format!("{}", e)))?;
    }

    ctx.env
        .create_buffer_with_data(buffer)
        .map(|v| v.into_raw())
}

register_module!(resvg, init);

fn init(module: &mut napi::Module) -> napi::Result<()> {
    module.create_named_method("render", render)?;
    Ok(())
}
