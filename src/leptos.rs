#![doc = include_str!("../LEPTOS.md")]
#![allow(unused)]

use crate::common::{
    Ingredient, NORI_COLOR, RICE_COLOR, SushiData, SushiOuterSheet, SushiShape, SushiSize,
    SushiState, SushiView, TopEdgeDecoration,
};
use crate::svg::render_sushi;
use leptos::prelude::*;

/// An SVG sushi renderer for the Leptos framework.
///
/// Renders a fully accessible SVG sushi piece.
/// All shapes (`Circular`, `Square`, `Triangular`, `Oval`) are supported in
/// both `Top` and `Front` views, plus an `Exploded` ingredient view.
///
/// # Arguments
/// * `id` - Unique identifier for SVG namespace. Defaults to `"sushi"`.
/// * `name` - Display name shown in ARIA labels. Defaults to `"Custom Roll"`.
/// * `shape` - Outer silhouette shape. Defaults to `SushiShape::Circular`.
/// * `state` - Rolled or Exploded. Defaults to `SushiState::Rolled`.
/// * `view` - Top or Front perspective. Defaults to `SushiView::Top`.
/// * `ingredients` - Vec of [`Ingredient`]s. Defaults to Salmon.
/// * `outer_sheet` - [`SushiOuterSheet`] colour and thickness.
/// * `size` - [`SushiSize`] canvas dimensions in pixels.
/// * `rice_color` - Optional custom rice colour.
/// * `top_edge` - Optional [`TopEdgeDecoration`].
/// * `scale` - Uniform scale factor. Defaults to `1.0`.
/// * `class` - CSS class for the wrapper `<div>`.
/// * `style` - Inline style for the wrapper `<div>`.
///
/// # Returns
/// `impl IntoView`: the rendered sushi SVG embedded in a `<div>`.
///
/// # Examples
/// ```rust
/// use leptos::prelude::*;
/// use sushi_rs::leptos::Sushi;
/// use sushi_rs::common::{SushiShape, Ingredient, SushiOuterSheet, SushiSize, NORI_COLOR};
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Sushi
///             id="salmon-maki"
///             name="Salmon Maki"
///             shape=SushiShape::Circular
///             ingredients=vec![Ingredient::Salmon]
///             outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
///             size=SushiSize { width: 120.0, height: 120.0 }
///         />
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN SVG Element](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg)
#[component]
pub fn Sushi(
    /// Unique identifier used to namespace SVG element IDs.
    #[prop(into, default = "sushi".to_string())]
    id: String,

    /// Human-readable label used in ARIA attributes.
    #[prop(into, default = "Custom Roll".to_string())]
    name: String,

    /// Outer silhouette shape.
    #[prop(default = SushiShape::Circular)]
    shape: SushiShape,

    /// Rolled or Exploded display state.
    #[prop(default = SushiState::Rolled)]
    state: SushiState,

    /// Camera perspective: Top or Front.
    #[prop(default = SushiView::Top)]
    view: SushiView,

    /// Ordered list of inner ingredients.
    #[prop(default = vec![Ingredient::Salmon])]
    ingredients: Vec<Ingredient>,

    /// Nori sheet colour and thickness.
    #[prop(default = SushiOuterSheet::default())]
    outer_sheet: SushiOuterSheet,

    /// Canvas dimensions in pixels.
    #[prop(default = SushiSize::default())]
    size: SushiSize,

    /// Optional custom rice colour.
    #[prop(default = None)]
    rice_color: Option<String>,

    /// Optional top-rim decoration.
    #[prop(default = None)]
    top_edge: Option<TopEdgeDecoration>,

    /// Uniform scale applied to all pixel dimensions.
    #[prop(default = 1.0)]
    scale: f64,

    /// CSS class for the wrapper element.
    #[prop(default = "")]
    class: &'static str,

    /// Inline style for the wrapper element.
    #[prop(default = "display:inline-block;")]
    style: &'static str,

    /// Optional description used in ARIA labels.
    #[prop(default = None)]
    description: Option<String>,
) -> impl IntoView {
    let data = SushiData {
        id,
        name: name.clone(),
        shape,
        state,
        view,
        ingredients,
        outer_sheet,
        size,
        rice_color,
        top_edge,
        description,
    };

    let svg = render_sushi(&data, scale);
    let full_class = format!("sushi-wrapper {class}");
    let label = name;

    view! {
        <div
            class={full_class}
            style={style}
            role="figure"
            aria-label={label}
            tabindex="0"
            inner_html={svg}
        />
    }
}

/// A gallery that renders multiple sushi pieces in a flex layout.
///
/// # Arguments
/// * `items` - Vec of [`SushiData`] to display.
/// * `class` - CSS class for the container. Defaults to `"sushi-gallery"`.
/// * `style` - Inline style for the container.
/// * `aria_label` - Accessible label for the gallery region.
///
/// # Returns
/// `impl IntoView`: a `<section>` with one sushi card per item.
///
/// # Examples
/// ```rust
/// use leptos::prelude::*;
/// use sushi_rs::leptos::SushiGallery;
/// use sushi_rs::common::default_sushi_gallery;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! { <SushiGallery items=default_sushi_gallery() /> }
/// }
/// ```
#[component]
pub fn SushiGallery(
    /// Sushi pieces to display.
    #[prop(default = vec![])]
    items: Vec<SushiData>,

    /// CSS class for the gallery container.
    #[prop(default = "sushi-gallery")]
    class: &'static str,

    /// Inline style for the gallery container.
    #[prop(default = "display:flex;flex-wrap:wrap;gap:16px;align-items:center;")]
    style: &'static str,

    /// Accessible label for the gallery region.
    #[prop(default = "Sushi Gallery")]
    aria_label: &'static str,
) -> impl IntoView {
    view! {
        <section class={class} style={style} role="region" aria-label={aria_label}>
            <For
                each=move || items.clone()
                key=|item| item.id.clone()
                let:item
            >
                {move || {
                    let svg = render_sushi(&item, 1.0);
                    let name = item.name.clone();
                    let card = format!(
                        r#"<figure class="sushi-card" tabindex="0" role="figure" aria-label="{name}">{svg}<figcaption class="sushi-card-caption">{name}</figcaption></figure>"#,
                        name = name, svg = svg
                    );
                    view! { <div inner_html={card}/> }
                }}
            </For>
        </section>
    }
}
