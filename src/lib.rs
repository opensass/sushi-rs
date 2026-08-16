#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/sushi-rs/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/sushi-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod common;
pub mod svg;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

#[cfg(feature = "lep")]
pub mod leptos;

pub use common::{
    Ingredient, IngredientMeta, NORI_COLOR, RICE_COLOR, SushiData, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, SushiView, TopEdgeDecoration, TopEdgeType, default_sushi_gallery,
    describe_arc, ingredient_meta, ingredient_texture_id, polar_to_cart,
};
pub use svg::render_sushi;
