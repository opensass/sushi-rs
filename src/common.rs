use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

/// The outer shape of the sushi piece.
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum SushiShape {
    /// Classic round maki or gunkan roll.
    #[default]
    Circular,
    /// Square or rectangular tamago / oshi-style roll.
    Square,
    /// Triangular onigiri shape.
    Triangular,
    /// Oval / nigiri hand-pressed shape.
    Oval,
}

/// Whether the sushi is assembled or shown in layers.
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum SushiState {
    /// Fully assembled sushi roll.
    #[default]
    Rolled,
    /// Deconstructed: every component displayed side-by-side.
    Exploded,
}

/// Camera / render perspective.
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum SushiView {
    /// Bird's-eye view looking straight down.
    #[default]
    Top,
    /// Isometric frontal view showing depth.
    Front,
}

/// Decoration applied to the top rim of the sushi.
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum TopEdgeType {
    /// No edge decoration.
    #[default]
    None,
    /// Red salmon roe (ikura) ring.
    RedCaviar,
    /// Black caviar ring.
    BlackCaviar,
    /// Toasted sesame seed ring.
    Sesame,
    /// Fresh herb specks ring.
    Herbs,
    /// Flying-fish roe (tobiko) ring.
    Tobiko,
}

/// Inner filling / topping ingredient.
#[derive(EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Ingredient {
    Salmon,
    Tuna,
    Avocado,
    Egg,
    RedCaviar,
    BlackCaviar,
    Cucumber,
    Crab,
    Shrimp,
    Mango,
    CreamCheese,
    Wasabi,
}

/// Visual metadata for a single ingredient.
#[derive(Debug, Clone)]
pub struct IngredientMeta {
    /// Primary SVG fill colour.
    pub fill: &'static str,
    /// Border / stroke colour.
    pub stroke: &'static str,
    /// Human-readable display label.
    pub label: &'static str,
    /// Emoji representation.
    pub emoji: &'static str,
    /// Lighter highlight shade for texture shading.
    pub light: &'static str,
}

/// Width and height in pixels for the sushi canvas.
#[derive(Debug, Clone, PartialEq)]
pub struct SushiSize {
    /// Canvas width in pixels.
    pub width: f64,
    /// Canvas height in pixels.
    pub height: f64,
}

impl Default for SushiSize {
    fn default() -> Self {
        Self {
            width: 120.0,
            height: 120.0,
        }
    }
}

/// The outer nori / seaweed sheet properties.
#[derive(Debug, Clone, PartialEq)]
pub struct SushiOuterSheet {
    /// Hex colour of the wrapper. Use `"transparent"` for nigiri.
    pub color: String,
    /// Thickness of the wrapper band in pixels.
    pub thickness: f64,
}

impl Default for SushiOuterSheet {
    fn default() -> Self {
        Self {
            color: NORI_COLOR.to_string(),
            thickness: 10.0,
        }
    }
}

/// A decorative ring shown on the top rim of the sushi.
#[derive(Debug, Clone, PartialEq)]
pub struct TopEdgeDecoration {
    /// The kind of decoration.
    pub edge_type: TopEdgeType,
    /// Optional colour override for the decoration element.
    pub color: Option<String>,
}

impl Default for TopEdgeDecoration {
    fn default() -> Self {
        Self {
            edge_type: TopEdgeType::None,
            color: None,
        }
    }
}

/// Complete data model describing one sushi piece.
#[derive(Debug, Clone, PartialEq)]
pub struct SushiData {
    /// Unique identifier used to namespace SVG `id` attributes.
    pub id: String,
    /// Human-readable name displayed in labels and ARIA attributes.
    pub name: String,
    /// Outer silhouette shape.
    pub shape: SushiShape,
    /// Whether the sushi is rolled or deconstructed.
    pub state: SushiState,
    /// Render perspective (defaults to `Top`).
    pub view: SushiView,
    /// Ordered list of inner ingredients.
    pub ingredients: Vec<Ingredient>,
    /// Nori / outer sheet configuration.
    pub outer_sheet: SushiOuterSheet,
    /// Canvas dimensions in pixels.
    pub size: SushiSize,
    /// Optional custom rice colour (defaults to [`RICE_COLOR`]).
    pub rice_color: Option<String>,
    /// Optional top-rim decoration.
    pub top_edge: Option<TopEdgeDecoration>,
    /// Optional human-readable description.
    pub description: Option<String>,
}

impl Default for SushiData {
    fn default() -> Self {
        Self {
            id: "sushi-default".into(),
            name: "Custom Roll".into(),
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet::default(),
            size: SushiSize::default(),
            rice_color: None,
            top_edge: None,
            description: None,
        }
    }
}

/// Default nori seaweed colour.
pub const NORI_COLOR: &str = "#0D1F08";

/// Default shari rice colour.
pub const RICE_COLOR: &str = "#F7F3E8";

/// Returns the display metadata for an ingredient.
pub fn ingredient_meta(ingredient: Ingredient) -> IngredientMeta {
    match ingredient {
        Ingredient::Salmon => IngredientMeta {
            fill: "#FA8072",
            stroke: "#E05A4A",
            light: "#FFAAA0",
            label: "Salmon",
            emoji: "🐟",
        },
        Ingredient::Tuna => IngredientMeta {
            fill: "#C0392B",
            stroke: "#992D22",
            light: "#D9574A",
            label: "Tuna",
            emoji: "🐠",
        },
        Ingredient::Avocado => IngredientMeta {
            fill: "#5D9E2F",
            stroke: "#4A7C25",
            light: "#7EC84A",
            label: "Avocado",
            emoji: "🥑",
        },
        Ingredient::Egg => IngredientMeta {
            fill: "#FFD700",
            stroke: "#E5BE00",
            light: "#FFED6A",
            label: "Tamago Egg",
            emoji: "🥚",
        },
        Ingredient::RedCaviar => IngredientMeta {
            fill: "#E74C3C",
            stroke: "#C0392B",
            light: "#FF7F6E",
            label: "Red Caviar",
            emoji: "🔴",
        },
        Ingredient::BlackCaviar => IngredientMeta {
            fill: "#2C2C2C",
            stroke: "#444444",
            light: "#555555",
            label: "Black Caviar",
            emoji: "⚫",
        },
        Ingredient::Cucumber => IngredientMeta {
            fill: "#7AC74C",
            stroke: "#5E9B37",
            light: "#A0E066",
            label: "Cucumber",
            emoji: "🥒",
        },
        Ingredient::Crab => IngredientMeta {
            fill: "#E8733A",
            stroke: "#C5562A",
            light: "#FF9A68",
            label: "Crab",
            emoji: "🦀",
        },
        Ingredient::Shrimp => IngredientMeta {
            fill: "#FFB5A0",
            stroke: "#E89080",
            light: "#FFD5C8",
            label: "Shrimp",
            emoji: "🍤",
        },
        Ingredient::Mango => IngredientMeta {
            fill: "#FF9800",
            stroke: "#E58800",
            light: "#FFB84D",
            label: "Mango",
            emoji: "🥭",
        },
        Ingredient::CreamCheese => IngredientMeta {
            fill: "#FFFCF0",
            stroke: "#E0D8C0",
            light: "#FFFFFF",
            label: "Cream Cheese",
            emoji: "🧀",
        },
        Ingredient::Wasabi => IngredientMeta {
            fill: "#5CB85C",
            stroke: "#4A9A4A",
            light: "#7ACF7A",
            label: "Wasabi",
            emoji: "🌿",
        },
    }
}

/// Returns the SVG pattern `url(#...)` fill reference for an ingredient's texture.
pub fn ingredient_texture_id(ingredient: Ingredient, uid: &str) -> String {
    match ingredient {
        Ingredient::Salmon => format!("url(#salmon-p-{uid})"),
        Ingredient::RedCaviar => format!("url(#rcav-p-{uid})"),
        Ingredient::BlackCaviar => format!("url(#bcav-p-{uid})"),
        _ => format!("url(#sheen-p-{uid})"),
    }
}

/// Converts polar coordinates (degrees, starting at 12 o'clock) to Cartesian.
pub fn polar_to_cart(cx: f64, cy: f64, r: f64, deg: f64) -> (f64, f64) {
    let rad = (deg - 90.0) * std::f64::consts::PI / 180.0;
    (cx + r * rad.cos(), cy + r * rad.sin())
}

/// Builds an SVG arc path from a centre, through a start angle to an end angle.
pub fn describe_arc(cx: f64, cy: f64, r: f64, start_deg: f64, end_deg: f64) -> String {
    let (x1, y1) = polar_to_cart(cx, cy, r, end_deg);
    let (x2, y2) = polar_to_cart(cx, cy, r, start_deg);
    let large = if end_deg - start_deg <= 180.0 { 0 } else { 1 };
    format!(
        "M {cx} {cy} L {x1} {y1} A {r} {r} 0 {large} 0 {x2} {y2} Z",
        cx = cx,
        cy = cy,
        x1 = x1,
        y1 = y1,
        r = r,
        large = large,
        x2 = x2,
        y2 = y2
    )
}

/// Pre-built gallery of sushi pieces used for demo renders.
pub fn default_sushi_gallery() -> Vec<SushiData> {
    vec![
        SushiData {
            id: "salmon-maki".into(),
            name: "Salmon Maki".into(),
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            },
            size: SushiSize {
                width: 120.0,
                height: 120.0,
            },
            rice_color: None,
            top_edge: None,
            description: Some("Classic salmon maki roll with fresh Atlantic salmon.".into()),
        },
        SushiData {
            id: "tuna-maki".into(),
            name: "Tuna Maki".into(),
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Tuna],
            outer_sheet: SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            },
            size: SushiSize {
                width: 120.0,
                height: 120.0,
            },
            rice_color: None,
            top_edge: None,
            description: Some("Bluefin tuna wrapped in nori seaweed.".into()),
        },
        SushiData {
            id: "tamago-square".into(),
            name: "Tamago Square".into(),
            shape: SushiShape::Square,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Egg],
            outer_sheet: SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 8.0,
            },
            size: SushiSize {
                width: 120.0,
                height: 120.0,
            },
            rice_color: None,
            top_edge: None,
            description: Some("Sweet Japanese tamago egg in a seaweed square wrap.".into()),
        },
        SushiData {
            id: "tuna-onigiri".into(),
            name: "Tuna Onigiri".into(),
            shape: SushiShape::Triangular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Tuna],
            outer_sheet: SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 6.0,
            },
            size: SushiSize {
                width: 110.0,
                height: 120.0,
            },
            rice_color: None,
            top_edge: None,
            description: Some("Traditional triangular rice ball filled with tuna.".into()),
        },
        SushiData {
            id: "salmon-nigiri".into(),
            name: "Salmon Nigiri".into(),
            shape: SushiShape::Oval,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet {
                color: "transparent".into(),
                thickness: 0.0,
            },
            size: SushiSize {
                width: 130.0,
                height: 100.0,
            },
            rice_color: None,
            top_edge: None,
            description: Some("Hand-pressed salmon draped over seasoned shari rice.".into()),
        },
        SushiData {
            id: "rainbow-roll".into(),
            name: "Rainbow Roll".into(),
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon, Ingredient::Avocado, Ingredient::Tuna],
            outer_sheet: SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            },
            size: SushiSize {
                width: 120.0,
                height: 120.0,
            },
            rice_color: None,
            top_edge: Some(TopEdgeDecoration {
                edge_type: TopEdgeType::Sesame,
                color: None,
            }),
            description: Some("Colorful rainbow roll with premium multi-topping.".into()),
        },
    ]
}
