// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use crate::{Attrs, Font};

/// Access system fonts
pub struct FontSystem {
    locale: String,
    db: fontdb::Database,
}

impl FontSystem {
    pub fn new() -> Self {
        let locale = "en-US".to_string();

        let mut db = fontdb::Database::new();
        {
            db.set_monospace_family("Fira Mono");
            db.set_sans_serif_family("Fira Sans");
            db.set_serif_family("DejaVu Serif");
        }

        Self { locale, db }
    }

    pub fn new_with_locale_and_db(locale: &str, db: fontdb::Database) -> Self {
        Self {
            locale: locale.to_string(),
            db,
        }
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn db(&self) -> &fontdb::Database {
        &self.db
    }

    pub fn db_mut(&mut self) -> &mut fontdb::Database {
        &mut self.db
    }

    pub fn get_font(&self, id: fontdb::ID) -> Option<Arc<Font>> {
        get_font(&self.db, id)
    }

    pub fn get_font_matches(&self, attrs: impl AsRef<Attrs>) -> Arc<Vec<Arc<Font>>> {
        let mut fonts = Vec::new();
        for face in self.db.faces() {
            if !attrs.as_ref().matches(face) {
                continue;
            }

            if let Some(font) = get_font(&self.db, face.id) {
                fonts.push(font);
            }
        }

        Arc::new(fonts)
    }
}

fn get_font(db: &fontdb::Database, id: fontdb::ID) -> Option<Arc<Font>> {
    let face = db.face(id)?;
    match Font::new(face) {
        Some(font) => Some(Arc::new(font)),
        None => {
            log::warn!("failed to load font '{}'", face.post_script_name);
            None
        }
    }
}
