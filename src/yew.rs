#![doc = include_str!("../YEW.md")]

use crate::common::{
    Ingredient, SushiData, SushiOuterSheet, SushiShape, SushiSize, SushiState, SushiView,
    TopEdgeDecoration,
};
use crate::svg::render_sushi;
use yew::prelude::*;

/// Props for the [`Sushi`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct SushiProps {
    /// Unique identifier used to namespace all SVG element IDs.
    #[prop_or("sushi".to_string())]
    pub id: String,

    /// Human-readable label shown in ARIA attributes and exploded-view captions.
    #[prop_or("Custom Roll".to_string())]
    pub name: String,

    /// Outer silhouette shape of the sushi piece.
    #[prop_or_default]
    pub shape: SushiShape,

    /// Whether the sushi is rolled or shown in an exploded/deconstructed view.
    #[prop_or_default]
    pub state: SushiState,

    /// Render perspective: top-down or isometric front.
    #[prop_or_default]
    pub view: SushiView,

    /// Ordered list of inner filling ingredients.
    #[prop_or(vec![Ingredient::Salmon])]
    pub ingredients: Vec<Ingredient>,

    /// Nori / outer sheet configuration (colour + thickness).
    #[prop_or_default]
    pub outer_sheet: SushiOuterSheet,

    /// Canvas dimensions in pixels.
    #[prop_or_default]
    pub size: SushiSize,

    /// Optional custom rice colour; defaults to the cream-white constant.
    #[prop_or_default]
    pub rice_color: Option<String>,

    /// Optional top-rim decoration (caviar ring, sesame, herbs, tobiko).
    #[prop_or_default]
    pub top_edge: Option<TopEdgeDecoration>,

    /// Uniform scale factor applied to all pixel dimensions.
    #[prop_or(1.0)]
    pub scale: f64,

    /// Additional CSS class applied to the wrapper `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline style applied to the wrapper `<div>`.
    #[prop_or("display:inline-block;")]
    pub style: &'static str,

    /// Optional short description used in ARIA labels.
    #[prop_or_default]
    pub description: Option<String>,
}

/// An SVG sushi renderer for the Yew framework.
///
/// Renders a fully accessible SVG sushi piece given a set of properties.  
/// Supports [`SushiShape::Circular`], [`SushiShape::Square`],
/// [`SushiShape::Triangular`], and [`SushiShape::Oval`] shapes in both
/// [`SushiView::Top`] and [`SushiView::Front`] views, plus an exploded
/// ingredient view when `state` is [`SushiState::Exploded`].
///
/// # Arguments
/// * `props` - A [`SushiProps`] struct controlling every visual aspect of the sushi.
///
/// # Returns
/// `Html`: the rendered SVG sushi component.
///
/// # Examples
/// ```rust
/// use yew::prelude::*;
/// use sushi_rs::yew::Sushi;
/// use sushi_rs::common::{SushiShape, SushiState, SushiView, Ingredient, SushiOuterSheet, SushiSize, NORI_COLOR};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Sushi
///             id="my-salmon-maki"
///             name="Salmon Maki"
///             shape={SushiShape::Circular}
///             state={SushiState::Rolled}
///             view={SushiView::Top}
///             ingredients={vec![Ingredient::Salmon]}
///             outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
///             size={SushiSize { width: 120.0, height: 120.0 }}
///         />
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN SVG Element](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg)
#[function_component(Sushi)]
pub fn sushi(props: &SushiProps) -> Html {
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

    Html::from_html_unchecked(
        format!(
            r#"<div class="{}" style="{}" role="figure" aria-label="{}" tabindex="0">{}</div>"#,
            full_class, props.style, props.name, svg
        )
        .into(),
    )
}

/// Props for the [`SushiGallery`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct SushiGalleryProps {
    /// Collection of sushi pieces to display.
    #[prop_or_default]
    pub items: Vec<SushiData>,

    /// CSS class for the gallery container.
    #[prop_or("sushi-gallery")]
    pub class: &'static str,

    /// Inline style for the gallery container.
    #[prop_or("display:flex;flex-wrap:wrap;gap:16px;align-items:center;")]
    pub style: &'static str,

    /// Accessible label for the entire gallery region.
    #[prop_or("Sushi Gallery")]
    pub aria_label: &'static str,
}

/// A responsive gallery that renders multiple [`Sushi`] components in a flex layout.
///
/// # Arguments
/// * `props` - [`SushiGalleryProps`] containing the list of sushi pieces and layout options.
///
/// # Returns
/// `Html`: a `<section>` containing one [`Sushi`] card per item.
///
/// # Examples
/// ```rust
/// use yew::prelude::*;
/// use sushi_rs::yew::SushiGallery;
/// use sushi_rs::common::default_sushi_gallery;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <SushiGallery items={default_sushi_gallery()} />
///     }
/// }
/// ```
#[function_component(SushiGallery)]
pub fn sushi_gallery(props: &SushiGalleryProps) -> Html {
    html! {
        <section
            class={props.class}
            style={props.style}
            role="region"
            aria-label={props.aria_label}
        >
            { for props.items.iter().map(|item| {
                let name = item.name.clone();
                let svg = render_sushi(item, 1.0);
                let card_html = format!(
                    r#"<figure class="sushi-card" tabindex="0" role="figure" aria-label="{name}">
  {svg}
  <figcaption class="sushi-card-caption">{name}</figcaption>
</figure>"#,
                    name = name, svg = svg
                );
                Html::from_html_unchecked(card_html.into())
            }) }
        </section>
    }
}
