# 🌱 Sushi RS Leptos Usage

Adding Sushi RS to your Leptos project:

1. Set up Leptos using the [Getting Started Guide](https://book.leptos.dev/getting_started/index.html).

1. Add the dependency:

   ```sh
   cargo add sushi-rs --features=lep
   ```

## 🛠️ Usage

```rust
use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{SushiShape, Ingredient, SushiOuterSheet, SushiSize, NORI_COLOR};

#[component]
fn App() -> impl IntoView {
    view! {
        <Sushi
            id="salmon-maki"
            name="Salmon Maki"
            shape=SushiShape::Circular
            ingredients=vec![Ingredient::Salmon]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
            size=SushiSize { width: 120.0, height: 120.0 }
        />
    }
}
```

### Gallery

```rust
use leptos::prelude::*;
use sushi_rs::leptos::SushiGallery;
use sushi_rs::common::default_sushi_gallery;

#[component]
fn App() -> impl IntoView {
    view! { <SushiGallery items=default_sushi_gallery() /> }
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

## 💡 Notes

- Uses `inner_html` prop to embed the shared SVG string engine.
- All `<g>` groups carry `role="img"` and `<title>` for screen readers.
- `SushiState::Exploded` renders all layers side by side.
