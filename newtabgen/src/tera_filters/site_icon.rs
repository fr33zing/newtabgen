// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the `site_icon` Tera filter.

use std::collections::HashMap;

use tera::{to_value, Filter};

use crate::builder;

/// Site icon CSS classname filter for use in Tera templates. Converts a string into a corresponding
/// CSS classname to add a website's icon to the background of the element the class is applied to.
pub struct SiteIcon;

impl Filter for SiteIcon {
    fn filter(
        &self,
        value: &tera::Value,
        _args: &HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        match value.as_str() {
            Some(url) => {
                let output = builder::site_icons::site_icon_class(url);
                to_value(output).map_err(|_| {
                    tera::Error::msg(
                        "formatting site icon class produced invalid value: '{output}'",
                    )
                })
            }
            None => Err(tera::Error::msg(
                "tried to get site icon class from non-string",
            )),
        }
    }
}
