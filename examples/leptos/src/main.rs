use leptos::prelude::*;
use sushi_rs::common::{
    Ingredient, NORI_COLOR, SushiOuterSheet, SushiShape, SushiSize, SushiState, SushiView,
    TopEdgeDecoration, TopEdgeType, default_sushi_gallery,
};
use sushi_rs::leptos::{Sushi, SushiGallery};

#[component]
pub fn Example1() -> impl IntoView {
    view! {
        <Sushi
            id="ex1"
            name="Salmon Maki"
            shape=SushiShape::Circular
            state=SushiState::Rolled
            view=SushiView::Top
            ingredients=vec![Ingredient::Salmon]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
            size=SushiSize { width: 140.0, height: 140.0 }
        />
    }
}

#[component]
pub fn Example2() -> impl IntoView {
    view! {
        <Sushi
            id="ex2"
            name="Rainbow Roll"
            shape=SushiShape::Circular
            ingredients=vec![Ingredient::Salmon, Ingredient::Avocado, Ingredient::Tuna]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
            size=SushiSize { width: 140.0, height: 140.0 }
            top_edge=Some(TopEdgeDecoration { edge_type: TopEdgeType::Sesame, color: None })
        />
    }
}

#[component]
pub fn Example3() -> impl IntoView {
    view! {
        <Sushi
            id="ex3"
            name="Tamago Square"
            shape=SushiShape::Square
            ingredients=vec![Ingredient::Egg]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 8.0 }
            size=SushiSize { width: 130.0, height: 130.0 }
        />
    }
}

#[component]
pub fn Example4() -> impl IntoView {
    view! {
        <Sushi
            id="ex4"
            name="Tuna Onigiri"
            shape=SushiShape::Triangular
            ingredients=vec![Ingredient::Tuna]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 6.0 }
            size=SushiSize { width: 120.0, height: 130.0 }
        />
    }
}

#[component]
pub fn Example5() -> impl IntoView {
    view! {
        <Sushi
            id="ex5"
            name="Salmon Nigiri"
            shape=SushiShape::Oval
            ingredients=vec![Ingredient::Salmon]
            outer_sheet=SushiOuterSheet { color: "transparent".into(), thickness: 0.0 }
            size=SushiSize { width: 150.0, height: 100.0 }
        />
    }
}

#[component]
pub fn Example6() -> impl IntoView {
    view! {
        <Sushi
            id="ex6"
            name="Crab Maki Front"
            shape=SushiShape::Circular
            view=SushiView::Front
            ingredients=vec![Ingredient::Crab]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
            size=SushiSize { width: 140.0, height: 140.0 }
        />
    }
}

#[component]
pub fn Example7() -> impl IntoView {
    view! {
        <Sushi
            id="ex7"
            name="Shrimp Square Front"
            shape=SushiShape::Square
            view=SushiView::Front
            ingredients=vec![Ingredient::Shrimp]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 8.0 }
            size=SushiSize { width: 130.0, height: 130.0 }
        />
    }
}

#[component]
pub fn Example8() -> impl IntoView {
    view! {
        <Sushi
            id="ex8"
            name="Avocado Triangle Front"
            shape=SushiShape::Triangular
            view=SushiView::Front
            ingredients=vec![Ingredient::Avocado]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 6.0 }
            size=SushiSize { width: 120.0, height: 130.0 }
        />
    }
}

#[component]
pub fn Example9() -> impl IntoView {
    view! {
        <Sushi
            id="ex9"
            name="Tuna Nigiri Front"
            shape=SushiShape::Oval
            view=SushiView::Front
            ingredients=vec![Ingredient::Tuna]
            outer_sheet=SushiOuterSheet { color: "transparent".into(), thickness: 0.0 }
            size=SushiSize { width: 150.0, height: 100.0 }
        />
    }
}

#[component]
pub fn Example10() -> impl IntoView {
    view! {
        <Sushi
            id="ex10"
            name="Exploded Philadelphia Roll"
            shape=SushiShape::Circular
            state=SushiState::Exploded
            ingredients=vec![Ingredient::CreamCheese, Ingredient::Salmon]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
            size=SushiSize { width: 120.0, height: 120.0 }
        />
    }
}

#[component]
pub fn Example11() -> impl IntoView {
    view! {
        <Sushi
            id="ex11"
            name="Red Caviar Roll"
            shape=SushiShape::Circular
            ingredients=vec![Ingredient::RedCaviar]
            outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
            size=SushiSize { width: 140.0, height: 140.0 }
            top_edge=Some(TopEdgeDecoration { edge_type: TopEdgeType::RedCaviar, color: None })
            rice_color=Some(String::from("#FFF9E8"))
            scale=1.1
        />
    }
}

#[component]
pub fn Example12() -> impl IntoView {
    view! {
        <SushiGallery items=default_sushi_gallery() />
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Sushi RS Leptos Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Circular Top: Single Ingredient" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, SushiView, NORI_COLOR,
};

#[component]
pub fn Example1() -> impl IntoView {
    view! {
        <Sushi
            id="salmon-maki"
            name="Salmon Maki"
            shape=SushiShape::Circular
            state=SushiState::Rolled
            view=SushiView::Top
            ingredients=vec![Ingredient::Salmon]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 10.0
            }
            size=SushiSize { width: 140.0, height: 140.0 }
        />
    }
}"## }
                    </pre>
                    <Example1 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Circular Top: Multi-Ingredient + Sesame Edge" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
    TopEdgeDecoration, TopEdgeType, NORI_COLOR,
};

#[component]
pub fn Example2() -> impl IntoView {
    view! {
        <Sushi
            id="rainbow-roll"
            name="Rainbow Roll"
            shape=SushiShape::Circular
            ingredients=vec![
                Ingredient::Salmon,
                Ingredient::Avocado,
                Ingredient::Tuna,
            ]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 10.0
            }
            size=SushiSize { width: 140.0, height: 140.0 }
            top_edge=Some(TopEdgeDecoration {
                edge_type: TopEdgeType::Sesame, color: None
            })
        />
    }
}"## }
                    </pre>
                    <Example2 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Square Top: Tamago Egg" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, NORI_COLOR,
};

#[component]
pub fn Example3() -> impl IntoView {
    view! {
        <Sushi
            id="tamago-square"
            name="Tamago Square"
            shape=SushiShape::Square
            ingredients=vec![Ingredient::Egg]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 8.0
            }
            size=SushiSize { width: 130.0, height: 130.0 }
        />
    }
}"## }
                    </pre>
                    <Example3 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Triangular Top: Tuna Onigiri" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, NORI_COLOR,
};

#[component]
pub fn Example4() -> impl IntoView {
    view! {
        <Sushi
            id="tuna-onigiri"
            name="Tuna Onigiri"
            shape=SushiShape::Triangular
            ingredients=vec![Ingredient::Tuna]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 6.0
            }
            size=SushiSize { width: 120.0, height: 130.0 }
        />
    }
}"## }
                    </pre>
                    <Example4 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Oval Top: Salmon Nigiri (no nori)" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
};

#[component]
pub fn Example5() -> impl IntoView {
    view! {
        <Sushi
            id="salmon-nigiri"
            name="Salmon Nigiri"
            shape=SushiShape::Oval
            ingredients=vec![Ingredient::Salmon]
            outer_sheet=SushiOuterSheet {
                color: "transparent".into(), thickness: 0.0
            }
            size=SushiSize { width: 150.0, height: 100.0 }
        />
    }
}"## }
                    </pre>
                    <Example5 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Circular Front View: Crab" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
};

#[component]
pub fn Example6() -> impl IntoView {
    view! {
        <Sushi
            id="crab-front"
            name="Crab Maki Front"
            shape=SushiShape::Circular
            view=SushiView::Front
            ingredients=vec![Ingredient::Crab]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 10.0
            }
            size=SushiSize { width: 140.0, height: 140.0 }
        />
    }
}"## }
                    </pre>
                    <Example6 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Square Front View: Shrimp" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
};

#[component]
pub fn Example7() -> impl IntoView {
    view! {
        <Sushi
            id="shrimp-sq-front"
            name="Shrimp Square Front"
            shape=SushiShape::Square
            view=SushiView::Front
            ingredients=vec![Ingredient::Shrimp]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 8.0
            }
            size=SushiSize { width: 130.0, height: 130.0 }
        />
    }
}"## }
                    </pre>
                    <Example7 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Triangular Front View: Avocado" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
};

#[component]
pub fn Example8() -> impl IntoView {
    view! {
        <Sushi
            id="avocado-tri-front"
            name="Avocado Triangle Front"
            shape=SushiShape::Triangular
            view=SushiView::Front
            ingredients=vec![Ingredient::Avocado]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 6.0
            }
            size=SushiSize { width: 120.0, height: 130.0 }
        />
    }
}"## }
                    </pre>
                    <Example8 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Oval Front View: Tuna Nigiri" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView,
};

#[component]
pub fn Example9() -> impl IntoView {
    view! {
        <Sushi
            id="tuna-nigiri-front"
            name="Tuna Nigiri Front"
            shape=SushiShape::Oval
            view=SushiView::Front
            ingredients=vec![Ingredient::Tuna]
            outer_sheet=SushiOuterSheet {
                color: "transparent".into(), thickness: 0.0
            }
            size=SushiSize { width: 150.0, height: 100.0 }
        />
    }
}"## }
                    </pre>
                    <Example9 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Exploded State: Philadelphia Roll" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, NORI_COLOR,
};

#[component]
pub fn Example10() -> impl IntoView {
    view! {
        <Sushi
            id="philly-exploded"
            name="Exploded Philadelphia Roll"
            shape=SushiShape::Circular
            state=SushiState::Exploded
            ingredients=vec![
                Ingredient::CreamCheese,
                Ingredient::Salmon,
            ]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 10.0
            }
            size=SushiSize { width: 120.0, height: 120.0 }
        />
    }
}"## }
                    </pre>
                    <Example10 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Red Caviar Edge + Custom Rice + Scale" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
    TopEdgeDecoration, TopEdgeType, NORI_COLOR,
};

#[component]
pub fn Example11() -> impl IntoView {
    view! {
        <Sushi
            id="caviar-roll"
            name="Red Caviar Roll"
            shape=SushiShape::Circular
            ingredients=vec![Ingredient::RedCaviar]
            outer_sheet=SushiOuterSheet {
                color: NORI_COLOR.into(), thickness: 10.0
            }
            size=SushiSize { width: 140.0, height: 140.0 }
            top_edge=Some(TopEdgeDecoration {
                edge_type: TopEdgeType::RedCaviar, color: None
            })
            rice_color=Some(String::from("#FFF9E8"))
            scale=1.1
        />
    }
}"## }
                    </pre>
                    <Example11 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "SushiGallery: Built-in Gallery" }</h2>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto">
                        { r##"use leptos::prelude::*;
use sushi_rs::leptos::SushiGallery;
use sushi_rs::common::default_sushi_gallery;

#[component]
pub fn Example12() -> impl IntoView {
    view! {
        <SushiGallery items=default_sushi_gallery() />
    }
}"## }
                    </pre>
                    <Example12 />
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App/> })
}
