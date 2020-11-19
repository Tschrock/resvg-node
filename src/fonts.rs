// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use usvg::{self, SystemFontDB};

use crate::options::*;

/// Loads fonts.
pub fn load_fonts(font_options: &JsFontOptions) -> usvg::fontdb::Database {
    // Create a new font database
    let mut fontdb = usvg::fontdb::Database::new();

    // Load system fonts
    if font_options.load_system_fonts {
        fontdb.load_system_fonts();
    }

    // Load font paths
    for path in &font_options.font_files {
        if let Err(e) = fontdb.load_font_file(path) {
            log::warn!("Failed to load '{}' cause {}.", path, e);
        }
    }

    // Load font directories
    for path in &font_options.font_dirs {
        fontdb.load_fonts_dir(path);
    }

    // Set generic font families
    fontdb.set_serif_family(&font_options.serif_family);
    fontdb.set_sans_serif_family(&font_options.sans_serif_family);
    fontdb.set_cursive_family(&font_options.cursive_family);
    fontdb.set_fantasy_family(&font_options.fantasy_family);
    fontdb.set_monospace_family(&font_options.monospace_family);

    fontdb
}
