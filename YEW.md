# 🍣 Sushi RS Yew Usage

Adding Sushi RS to your Yew project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Sushi RS component to your dependencies:

   ```sh
   cargo add sushi-rs --features=yew
   ```

1. Import and use the `Sushi` component.

## 🛠️ Usage

```rust
use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{SushiShape, SushiState, SushiView, Ingredient, SushiOuterSheet, SushiSize, NORI_COLOR};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Sushi
            id="salmon-maki"
            name="Salmon Maki"
            shape={SushiShape::Circular}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 120.0, height: 120.0 }}
        />
    }
}
```

### Gallery

```rust
use yew::prelude::*;
use sushi_rs::yew::SushiGallery;
use sushi_rs::common::default_sushi_gallery;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <SushiGallery items={default_sushi_gallery()} />
    }
}
```

## 🔧 Props

### `Sushi` Component

| Prop          | Type                        | Description               | Default         |
| ------------- | --------------------------- | ------------------------- | --------------- |
| `id`          | `String`                    | SVG namespace ID          | `"sushi"`       |
| `name`        | `String`                    | Display name / ARIA label | `"Custom Roll"` |
| `shape`       | `SushiShape`                | Silhouette shape          | `Circular`      |
| `state`       | `SushiState`                | `Rolled` or `Exploded`    | `Rolled`        |
| `view`        | `SushiView`                 | `Top` or `Front`          | `Top`           |
| `ingredients` | `Vec<Ingredient>`           | Inner fillings            | `[Salmon]`      |
| `outer_sheet` | `SushiOuterSheet`           | Nori colour + thickness   | nori defaults   |
| `size`        | `SushiSize`                 | Width × height in px      | `120×120`       |
| `rice_color`  | `Option<String>`            | Custom rice colour        | cream-white     |
| `top_edge`    | `Option<TopEdgeDecoration>` | Rim decoration            | `None`          |
| `scale`       | `f64`                       | Uniform scale factor      | `1.0`           |
| `class`       | `&'static str`              | Extra CSS class           | `""`            |
| `style`       | `&'static str`              | Inline wrapper style      | inline-block    |

### `SushiShape` Enum

| Variant      | Description               |
| ------------ | ------------------------- |
| `Circular`   | Round maki / gunkan roll  |
| `Square`     | Rectangular tamago / oshi |
| `Triangular` | Triangular onigiri        |
| `Oval`       | Nigiri hand-pressed shape |

### `Ingredient` Enum

| Variant       | Label        | Emoji |
| ------------- | ------------ | ----- |
| `Salmon`      | Salmon       | 🐟    |
| `Tuna`        | Tuna         | 🐠    |
| `Avocado`     | Avocado      | 🥑    |
| `Egg`         | Tamago Egg   | 🥚    |
| `RedCaviar`   | Red Caviar   | 🔴    |
| `BlackCaviar` | Black Caviar | ⚫    |
| `Cucumber`    | Cucumber     | 🥒    |
| `Crab`        | Crab         | 🦀    |
| `Shrimp`      | Shrimp       | 🍤    |
| `Mango`       | Mango        | 🥭    |
| `CreamCheese` | Cream Cheese | 🧀    |
| `Wasabi`      | Wasabi       | 🌿    |

### `TopEdgeDecoration`

| Field       | Type             | Description                                                     |
| ----------- | ---------------- | --------------------------------------------------------------- |
| `edge_type` | `TopEdgeType`    | `None`, `RedCaviar`, `BlackCaviar`, `Sesame`, `Herbs`, `Tobiko` |
| `color`     | `Option<String>` | Override colour for herbs                                       |

## 🧱 Layout Structure

```html
<div class="sushi-wrapper" role="figure">
  <svg ...>
    ← rolled view
    <defs>...</defs>
    <!-- nori + rice + ingredients -->
  </svg>
</div>
```

or in exploded state:

```html
<div class="sushi-exploded-row" role="img">
  <div class="sushi-exploded-part">Nori SVG</div>
  +
  <div class="sushi-exploded-part">Rice SVG</div>
  +
  <div class="sushi-exploded-part">Ingredient SVG ...</div>
</div>
```

## 💡 Notes

- Multiple ingredients are rendered as equal pie-slices with individual `<title>` elements.
- The `scale` prop multiplies both size and sheet thickness uniformly.
- `SushiState::Exploded` ignores `shape` and `view` and renders all layers side by side.
- All SVG groups carry `role="img"` and `aria-label` via `<title>`.
