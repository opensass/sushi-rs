#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    Ingredient, SushiData, SushiOuterSheet, SushiShape, SushiSize, SushiState, SushiView,
    TopEdgeDecoration,
};
use crate::svg::render_sushi;
use dioxus::prelude::*;

/// Props for the [`Sushi`] Dioxus component.
#[derive(Props, PartialEq, Clone)]
pub struct SushiProps {
    /// Unique identifier used to namespace all SVG element IDs.
    #[props(default = "sushi".to_string())]
    pub id: String,

    /// Human-readable label shown in ARIA attributes and exploded-view captions.
    #[props(default = "Custom Roll".to_string())]
    pub name: String,

    /// Outer silhouette shape of the sushi piece.
    #[props(default)]
    pub shape: SushiShape,

    /// Whether the sushi is rolled or shown in an exploded/deconstructed view.
    #[props(default)]
    pub state: SushiState,

    /// Render perspective: top-down or isometric front.
    #[props(default)]
    pub view: SushiView,

    /// Ordered list of inner filling ingredients.
    #[props(default = vec![Ingredient::Salmon])]
    pub ingredients: Vec<Ingredient>,

    /// Nori / outer sheet configuration (colour + thickness).
    #[props(default)]
    pub outer_sheet: SushiOuterSheet,

    /// Canvas dimensions in pixels.
    #[props(default)]
    pub size: SushiSize,

    /// Optional custom rice colour; defaults to the cream-white constant.
    #[props(default)]
    pub rice_color: Option<String>,

    /// Optional top-rim decoration (caviar ring, sesame, herbs, tobiko).
    #[props(default)]
    pub top_edge: Option<TopEdgeDecoration>,

    /// Uniform scale factor applied to all pixel dimensions.
    #[props(default = 1.0)]
    pub scale: f64,

    /// Additional CSS class applied to the wrapper `<div>`.
    #[props(default = "")]
    pub class: &'static str,

    /// Inline style applied to the wrapper `<div>`.
    #[props(default = "display:inline-block;")]
    pub style: &'static str,

    /// Optional short description used in ARIA labels.
    #[props(default)]
    pub description: Option<String>,
}

/// An SVG sushi renderer for the Dioxus framework.
///
/// Renders a fully accessible SVG sushi piece from a [`SushiProps`] struct.
/// All shapes (`Circular`, `Square`, `Triangular`, `Oval`) are supported in
/// both `Top` and `Front` views, and the `Exploded` state renders each
/// ingredient separately with accessible labels.
///
/// # Arguments
/// * `props` - A [`SushiProps`] struct controlling every visual aspect of the sushi.
///
/// # Returns
/// `Element`: the rendered SVG sushi component.
///
/// # Examples
/// ```rust
/// use dioxus::prelude::*;
/// use sushi_rs::dioxus::Sushi;
/// use sushi_rs::common::{SushiShape, Ingredient, SushiOuterSheet, SushiSize, NORI_COLOR};
///
/// fn App() -> Element {
///     rsx! {
///         Sushi {
///             id: "salmon-maki",
///             name: "Salmon Maki",
///             shape: SushiShape::Circular,
///             ingredients: vec![Ingredient::Salmon],
///             outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 },
///             size: SushiSize { width: 120.0, height: 120.0 },
///         }
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN SVG Element](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg)
#[component]
pub fn Sushi(props: SushiProps) -> Element {
    let data = SushiData {
        id: props.id.clone(),
        name: props.name.clone(),
        shape: props.shape,
        state: props.state,
        view: props.view,
        ingredients: props.ingredients.clone(),
        outer_sheet: props.outer_sheet.clone(),
        size: props.size.clone(),
        rice_color: props.rice_color.clone(),
        top_edge: props.top_edge.clone(),
        description: props.description.clone(),
    };

    let svg = render_sushi(&data, props.scale);
    let full_class = format!("sushi-wrapper {}", props.class);

    rsx! {
        div {
            class: "{full_class}",
            style: "{props.style}",
            role: "figure",
            aria_label: "{props.name}",
            tabindex: "0",
            dangerous_inner_html: "{svg}",
        }
    }
}

/// Props for the [`SushiGallery`] Dioxus component.
#[derive(Props, PartialEq, Clone)]
pub struct SushiGalleryProps {
    /// Collection of sushi pieces to display.
    #[props(default)]
    pub items: Vec<SushiData>,

    /// CSS class for the gallery container.
    #[props(default = "sushi-gallery")]
    pub class: &'static str,

    /// Inline style for the gallery container.
    #[props(default = "display:flex;flex-wrap:wrap;gap:16px;align-items:center;")]
    pub style: &'static str,

    /// Accessible label for the entire gallery region.
    #[props(default = "Sushi Gallery")]
    pub aria_label: &'static str,
}

/// A responsive gallery that renders multiple [`Sushi`] components in a flex layout.
///
/// # Arguments
/// * `props` - [`SushiGalleryProps`] containing sushi items and layout configuration.
///
/// # Returns
/// `Element`: a `<section>` containing one sushi card per item.
///
/// # Examples
/// ```rust
/// use dioxus::prelude::*;
/// use sushi_rs::dioxus::SushiGallery;
/// use sushi_rs::common::default_sushi_gallery;
///
/// fn App() -> Element {
///     rsx! {
///         SushiGallery { items: default_sushi_gallery() }
///     }
/// }
/// ```
#[component]
pub fn SushiGallery(props: SushiGalleryProps) -> Element {
    rsx! {
        section {
            class: "{props.class}",
            style: "{props.style}",
            role: "region",
            aria_label: "{props.aria_label}",
            for item in props.items.iter() {
                {
                    let svg = render_sushi(item, 1.0);
                    let name = item.name.clone();
                    let card = format!(
                        r#"<figure class="sushi-card" tabindex="0" role="figure" aria-label="{name}">{svg}<figcaption class="sushi-card-caption">{name}</figcaption></figure>"#,
                        name = name, svg = svg
                    );
                    rsx! {
                        div {
                            key: "{item.id}",
                            dangerous_inner_html: "{card}",
                        }
                    }
                }
            }
        }
    }
}
