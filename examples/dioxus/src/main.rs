use dioxus::prelude::*;
use dioxus_logger::tracing;
use sushi_rs::common::{
    Ingredient, NORI_COLOR, SushiOuterSheet, SushiShape, SushiSize, SushiState, SushiView,
    TopEdgeDecoration, TopEdgeType, default_sushi_gallery,
};
use sushi_rs::dioxus::{Sushi, SushiGallery};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        Examples {}
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        div {
            class: "p-6 w-full min-h-screen flex flex-col items-center justify-center",
            style: "color: #5e5c7f; background-color: #303030; font-family: 'Rubik', sans-serif; overflow-x: hidden;",
            h1 { class: "text-3xl font-bold mb-8 text-white", "Sushi RS Dioxus Examples" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Circular Top: Single Ingredient" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, SushiView, NORI_COLOR,
}};

#[component]
fn Example1() -> Element {{
    rsx! {{
        Sushi {{
            id: "salmon-maki",
            name: "Salmon Maki",
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }},
            size: SushiSize {{ width: 140.0, height: 140.0 }},
        }}
    }}
}}"##
                    }
                    Example1 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Circular Top: Multi-Ingredient + Sesame Edge" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
    TopEdgeDecoration, TopEdgeType, NORI_COLOR,
}};

#[component]
fn Example2() -> Element {{
    rsx! {{
        Sushi {{
            id: "rainbow-roll",
            name: "Rainbow Roll",
            shape: SushiShape::Circular,
            ingredients: vec![
                Ingredient::Salmon,
                Ingredient::Avocado,
                Ingredient::Tuna,
            ],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }},
            size: SushiSize {{ width: 140.0, height: 140.0 }},
            top_edge: Some(TopEdgeDecoration {{
                edge_type: TopEdgeType::Sesame,
                color: None,
            }}),
        }}
    }}
}}"##
                    }
                    Example2 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Square Top: Tamago Egg" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, NORI_COLOR,
}};

#[component]
fn Example3() -> Element {{
    rsx! {{
        Sushi {{
            id: "tamago-square",
            name: "Tamago Square",
            shape: SushiShape::Square,
            ingredients: vec![Ingredient::Egg],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 8.0,
            }},
            size: SushiSize {{ width: 130.0, height: 130.0 }},
        }}
    }}
}}"##
                    }
                    Example3 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Triangular Top: Tuna Onigiri" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, NORI_COLOR,
}};

#[component]
fn Example4() -> Element {{
    rsx! {{
        Sushi {{
            id: "tuna-onigiri",
            name: "Tuna Onigiri",
            shape: SushiShape::Triangular,
            ingredients: vec![Ingredient::Tuna],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 6.0,
            }},
            size: SushiSize {{ width: 120.0, height: 130.0 }},
        }}
    }}
}}"##
                    }
                    Example4 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Oval Top: Salmon Nigiri (no nori)" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
}};

#[component]
fn Example5() -> Element {{
    rsx! {{
        Sushi {{
            id: "salmon-nigiri",
            name: "Salmon Nigiri",
            shape: SushiShape::Oval,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet {{
                color: "transparent".into(),
                thickness: 0.0,
            }},
            size: SushiSize {{ width: 150.0, height: 100.0 }},
        }}
    }}
}}"##
                    }
                    Example5 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Circular Front View: Crab" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
}};

#[component]
fn Example6() -> Element {{
    rsx! {{
        Sushi {{
            id: "crab-front",
            name: "Crab Maki Front",
            shape: SushiShape::Circular,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Crab],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }},
            size: SushiSize {{ width: 140.0, height: 140.0 }},
        }}
    }}
}}"##
                    }
                    Example6 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Square Front View: Shrimp" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
}};

#[component]
fn Example7() -> Element {{
    rsx! {{
        Sushi {{
            id: "shrimp-sq-front",
            name: "Shrimp Square Front",
            shape: SushiShape::Square,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Shrimp],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 8.0,
            }},
            size: SushiSize {{ width: 130.0, height: 130.0 }},
        }}
    }}
}}"##
                    }
                    Example7 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Triangular Front View: Avocado" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
}};

#[component]
fn Example8() -> Element {{
    rsx! {{
        Sushi {{
            id: "avocado-tri-front",
            name: "Avocado Triangle Front",
            shape: SushiShape::Triangular,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Avocado],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 6.0,
            }},
            size: SushiSize {{ width: 120.0, height: 130.0 }},
        }}
    }}
}}"##
                    }
                    Example8 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Oval Front View: Tuna Nigiri" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView,
}};

#[component]
fn Example9() -> Element {{
    rsx! {{
        Sushi {{
            id: "tuna-nigiri-front",
            name: "Tuna Nigiri Front",
            shape: SushiShape::Oval,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Tuna],
            outer_sheet: SushiOuterSheet {{
                color: "transparent".into(),
                thickness: 0.0,
            }},
            size: SushiSize {{ width: 150.0, height: 100.0 }},
        }}
    }}
}}"##
                    }
                    Example9 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Exploded State: Philadelphia Roll" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, NORI_COLOR,
}};

#[component]
fn Example10() -> Element {{
    rsx! {{
        Sushi {{
            id: "philly-exploded",
            name: "Exploded Philadelphia Roll",
            shape: SushiShape::Circular,
            state: SushiState::Exploded,
            ingredients: vec![
                Ingredient::CreamCheese,
                Ingredient::Salmon,
            ],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }},
            size: SushiSize {{ width: 120.0, height: 120.0 }},
        }}
    }}
}}"##
                    }
                    Example10 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Red Caviar Edge + Custom Rice + Scale" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::Sushi;
use sushi_rs::common::{{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
    TopEdgeDecoration, TopEdgeType, NORI_COLOR,
}};

#[component]
fn Example11() -> Element {{
    rsx! {{
        Sushi {{
            id: "caviar-roll",
            name: "Red Caviar Roll",
            shape: SushiShape::Circular,
            ingredients: vec![Ingredient::RedCaviar],
            outer_sheet: SushiOuterSheet {{
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }},
            size: SushiSize {{ width: 140.0, height: 140.0 }},
            top_edge: Some(TopEdgeDecoration {{
                edge_type: TopEdgeType::RedCaviar,
                color: None,
            }}),
            rice_color: Some("#FFF9E8".into()),
            scale: 1.1,
        }}
    }}
}}"##
                    }
                    Example11 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "SushiGallery: Built-in Gallery" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use sushi_rs::dioxus::SushiGallery;
use sushi_rs::common::default_sushi_gallery;

#[component]
fn Example12() -> Element {{
    rsx! {{
        SushiGallery {{ items: default_sushi_gallery() }}
    }}
}}"##
                    }
                    Example12 {}
                }
            }
        }
    }
}

#[component]
fn Example1() -> Element {
    rsx! {
        Sushi {
            id: "ex1",
            name: "Salmon Maki",
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 },
            size: SushiSize { width: 140.0, height: 140.0 },
        }
    }
}

#[component]
fn Example2() -> Element {
    rsx! {
        Sushi {
            id: "ex2",
            name: "Rainbow Roll",
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon, Ingredient::Avocado, Ingredient::Tuna],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 },
            size: SushiSize { width: 140.0, height: 140.0 },
            top_edge: Some(TopEdgeDecoration { edge_type: TopEdgeType::Sesame, color: None }),
        }
    }
}

#[component]
fn Example3() -> Element {
    rsx! {
        Sushi {
            id: "ex3",
            name: "Tamago Square",
            shape: SushiShape::Square,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Egg],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 8.0 },
            size: SushiSize { width: 130.0, height: 130.0 },
        }
    }
}

#[component]
fn Example4() -> Element {
    rsx! {
        Sushi {
            id: "ex4",
            name: "Tuna Onigiri",
            shape: SushiShape::Triangular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Tuna],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 6.0 },
            size: SushiSize { width: 120.0, height: 130.0 },
        }
    }
}

#[component]
fn Example5() -> Element {
    rsx! {
        Sushi {
            id: "ex5",
            name: "Salmon Nigiri",
            shape: SushiShape::Oval,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::Salmon],
            outer_sheet: SushiOuterSheet { color: "transparent".into(), thickness: 0.0 },
            size: SushiSize { width: 150.0, height: 100.0 },
        }
    }
}

#[component]
fn Example6() -> Element {
    rsx! {
        Sushi {
            id: "ex6",
            name: "Crab Maki Front",
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Crab],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 },
            size: SushiSize { width: 140.0, height: 140.0 },
        }
    }
}

#[component]
fn Example7() -> Element {
    rsx! {
        Sushi {
            id: "ex7",
            name: "Shrimp Square Front",
            shape: SushiShape::Square,
            state: SushiState::Rolled,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Shrimp],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 8.0 },
            size: SushiSize { width: 130.0, height: 130.0 },
        }
    }
}

#[component]
fn Example8() -> Element {
    rsx! {
        Sushi {
            id: "ex8",
            name: "Avocado Triangle Front",
            shape: SushiShape::Triangular,
            state: SushiState::Rolled,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Avocado],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 6.0 },
            size: SushiSize { width: 120.0, height: 130.0 },
        }
    }
}

#[component]
fn Example9() -> Element {
    rsx! {
        Sushi {
            id: "ex9",
            name: "Tuna Nigiri Front",
            shape: SushiShape::Oval,
            state: SushiState::Rolled,
            view: SushiView::Front,
            ingredients: vec![Ingredient::Tuna],
            outer_sheet: SushiOuterSheet { color: "transparent".into(), thickness: 0.0 },
            size: SushiSize { width: 150.0, height: 100.0 },
        }
    }
}

#[component]
fn Example10() -> Element {
    rsx! {
        Sushi {
            id: "ex10",
            name: "Exploded Philadelphia Roll",
            shape: SushiShape::Circular,
            state: SushiState::Exploded,
            view: SushiView::Top,
            ingredients: vec![Ingredient::CreamCheese, Ingredient::Salmon],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 },
            size: SushiSize { width: 120.0, height: 120.0 },
        }
    }
}

#[component]
fn Example11() -> Element {
    rsx! {
        Sushi {
            id: "ex11",
            name: "Red Caviar Roll",
            shape: SushiShape::Circular,
            state: SushiState::Rolled,
            view: SushiView::Top,
            ingredients: vec![Ingredient::RedCaviar],
            outer_sheet: SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 },
            size: SushiSize { width: 140.0, height: 140.0 },
            top_edge: Some(TopEdgeDecoration { edge_type: TopEdgeType::RedCaviar, color: None }),
            rice_color: Some("#FFF9E8".into()),
            scale: 1.1,
        }
    }
}

#[component]
fn Example12() -> Element {
    rsx! {
        SushiGallery { items: default_sushi_gallery() }
    }
}
