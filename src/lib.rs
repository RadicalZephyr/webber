#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub mod utils;

/// A plugin
pub struct WebberPlugin;

impl Plugin for WebberPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(webber);
    }
}

fn webber() {
    println!("Hello, World!");
}
