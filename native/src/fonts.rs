use usvg::{self, SystemFontDB};

use crate::options::*;

pub fn load_fonts(font_options: &JsFontOptions) -> usvg::fontdb::Database {
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
