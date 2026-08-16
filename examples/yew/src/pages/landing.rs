use sushi_rs::common::{
    Ingredient, NORI_COLOR, SushiOuterSheet, SushiShape, SushiSize, SushiState, SushiView,
    TopEdgeDecoration, TopEdgeType, default_sushi_gallery,
};
use sushi_rs::yew::{Sushi, SushiGallery};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExampleCardProps {
    pub title: &'static str,
    pub code: &'static str,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ExampleCard)]
pub fn example_card(props: &ExampleCardProps) -> Html {
    html! {
        <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
            <h2 class="text-xl font-bold mb-2">{ props.title }</h2>
            <pre
                class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
            >
                { props.code }
            </pre>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(Example1)]
pub fn example1() -> Html {
    html! {
        <Sushi
            id="ex1"
            name="Salmon Maki"
            shape={SushiShape::Circular}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 140.0, height: 140.0 }}
        />
    }
}

#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Sushi
            id="ex2"
            name="Rainbow Roll"
            shape={SushiShape::Circular}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Salmon, Ingredient::Avocado, Ingredient::Tuna]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 140.0, height: 140.0 }}
            top_edge={Some(TopEdgeDecoration { edge_type: TopEdgeType::Sesame, color: None })}
        />
    }
}

#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <Sushi
            id="ex3"
            name="Tamago Square"
            shape={SushiShape::Square}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Egg]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 8.0 }}
            size={SushiSize { width: 130.0, height: 130.0 }}
        />
    }
}

#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <Sushi
            id="ex4"
            name="Tuna Onigiri"
            shape={SushiShape::Triangular}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Tuna]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 6.0 }}
            size={SushiSize { width: 120.0, height: 130.0 }}
        />
    }
}

#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <Sushi
            id="ex5"
            name="Salmon Nigiri"
            shape={SushiShape::Oval}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet { color: "transparent".into(), thickness: 0.0 }}
            size={SushiSize { width: 150.0, height: 100.0 }}
        />
    }
}

#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Sushi
            id="ex6"
            name="Crab Maki Front"
            shape={SushiShape::Circular}
            state={SushiState::Rolled}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Crab]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 140.0, height: 140.0 }}
        />
    }
}

#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <Sushi
            id="ex7"
            name="Shrimp Square Front"
            shape={SushiShape::Square}
            state={SushiState::Rolled}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Shrimp]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 8.0 }}
            size={SushiSize { width: 130.0, height: 130.0 }}
        />
    }
}

#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Sushi
            id="ex8"
            name="Avocado Triangle Front"
            shape={SushiShape::Triangular}
            state={SushiState::Rolled}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Avocado]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 6.0 }}
            size={SushiSize { width: 120.0, height: 130.0 }}
        />
    }
}

#[function_component(Example9)]
pub fn example9() -> Html {
    html! {
        <Sushi
            id="ex9"
            name="Tuna Nigiri Front"
            shape={SushiShape::Oval}
            state={SushiState::Rolled}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Tuna]}
            outer_sheet={SushiOuterSheet { color: "transparent".into(), thickness: 0.0 }}
            size={SushiSize { width: 150.0, height: 100.0 }}
        />
    }
}

#[function_component(Example10)]
pub fn example10() -> Html {
    html! {
        <Sushi
            id="ex10"
            name="Exploded Philadelphia Roll"
            shape={SushiShape::Circular}
            state={SushiState::Exploded}
            view={SushiView::Top}
            ingredients={vec![Ingredient::CreamCheese, Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 120.0, height: 120.0 }}
        />
    }
}

#[function_component(Example11)]
pub fn example11() -> Html {
    html! {
        <Sushi
            id="ex11"
            name="Red Caviar Roll"
            shape={SushiShape::Circular}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::RedCaviar]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 140.0, height: 140.0 }}
            top_edge={Some(TopEdgeDecoration { edge_type: TopEdgeType::RedCaviar, color: None })}
            rice_color={Some(String::from("#FFF9E8"))}
            scale={1.1}
        />
    }
}

#[function_component(Example12)]
pub fn example12() -> Html {
    html! {
        <SushiGallery items={default_sushi_gallery()} />
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Sushi RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <ExampleCard
                    title="Circular Top: Single Ingredient"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, SushiView, NORI_COLOR,
};

#[function_component(Example1)]
pub fn example1() -> Html {
    html! {
        <Sushi
            id="salmon-maki"
            name="Salmon Maki"
            shape={SushiShape::Circular}
            state={SushiState::Rolled}
            view={SushiView::Top}
            ingredients={vec![Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }}
            size={SushiSize { width: 140.0, height: 140.0 }}
        />
    }
}"##
                >
                    <Example1 />
                </ExampleCard>
                <ExampleCard
                    title="Circular Top: Multi-Ingredient + Sesame Edge"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
    TopEdgeDecoration, TopEdgeType, NORI_COLOR,
};

#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Sushi
            id="rainbow-roll"
            name="Rainbow Roll"
            shape={SushiShape::Circular}
            ingredients={vec![
                Ingredient::Salmon,
                Ingredient::Avocado,
                Ingredient::Tuna,
            ]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }}
            size={SushiSize { width: 140.0, height: 140.0 }}
            top_edge={Some(TopEdgeDecoration {
                edge_type: TopEdgeType::Sesame,
                color: None,
            })}
        />
    }
}"##
                >
                    <Example2 />
                </ExampleCard>
                <ExampleCard
                    title="Square Top: Tamago Egg"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, NORI_COLOR,
};

#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <Sushi
            id="tamago-square"
            name="Tamago Square"
            shape={SushiShape::Square}
            ingredients={vec![Ingredient::Egg]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 8.0,
            }}
            size={SushiSize { width: 130.0, height: 130.0 }}
        />
    }
}"##
                >
                    <Example3 />
                </ExampleCard>
                <ExampleCard
                    title="Triangular Top: Tuna Onigiri"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, NORI_COLOR,
};

#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <Sushi
            id="tuna-onigiri"
            name="Tuna Onigiri"
            shape={SushiShape::Triangular}
            ingredients={vec![Ingredient::Tuna]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 6.0,
            }}
            size={SushiSize { width: 120.0, height: 130.0 }}
        />
    }
}"##
                >
                    <Example4 />
                </ExampleCard>
                <ExampleCard
                    title="Oval Top: Salmon Nigiri (no nori)"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
};

#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <Sushi
            id="salmon-nigiri"
            name="Salmon Nigiri"
            shape={SushiShape::Oval}
            ingredients={vec![Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet {
                color: "transparent".into(),
                thickness: 0.0,
            }}
            size={SushiSize { width: 150.0, height: 100.0 }}
        />
    }
}"##
                >
                    <Example5 />
                </ExampleCard>
                <ExampleCard
                    title="Circular Front View: Crab"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
};

#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Sushi
            id="crab-front"
            name="Crab Maki Front"
            shape={SushiShape::Circular}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Crab]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }}
            size={SushiSize { width: 140.0, height: 140.0 }}
        />
    }
}"##
                >
                    <Example6 />
                </ExampleCard>
                <ExampleCard
                    title="Square Front View: Shrimp"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
};

#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <Sushi
            id="shrimp-sq-front"
            name="Shrimp Square Front"
            shape={SushiShape::Square}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Shrimp]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 8.0,
            }}
            size={SushiSize { width: 130.0, height: 130.0 }}
        />
    }
}"##
                >
                    <Example7 />
                </ExampleCard>
                <ExampleCard
                    title="Triangular Front View: Avocado"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView, NORI_COLOR,
};

#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Sushi
            id="avocado-tri-front"
            name="Avocado Triangle Front"
            shape={SushiShape::Triangular}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Avocado]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 6.0,
            }}
            size={SushiSize { width: 120.0, height: 130.0 }}
        />
    }
}"##
                >
                    <Example8 />
                </ExampleCard>
                <ExampleCard
                    title="Oval Front View: Tuna Nigiri"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiView,
};

#[function_component(Example9)]
pub fn example9() -> Html {
    html! {
        <Sushi
            id="tuna-nigiri-front"
            name="Tuna Nigiri Front"
            shape={SushiShape::Oval}
            view={SushiView::Front}
            ingredients={vec![Ingredient::Tuna]}
            outer_sheet={SushiOuterSheet {
                color: "transparent".into(),
                thickness: 0.0,
            }}
            size={SushiSize { width: 150.0, height: 100.0 }}
        />
    }
}"##
                >
                    <Example9 />
                </ExampleCard>
                <ExampleCard
                    title="Exploded State: Deconstructed Philadelphia Roll"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape,
    SushiSize, SushiState, NORI_COLOR,
};

#[function_component(Example10)]
pub fn example10() -> Html {
    html! {
        <Sushi
            id="philly-exploded"
            name="Exploded Philadelphia Roll"
            shape={SushiShape::Circular}
            state={SushiState::Exploded}
            ingredients={vec![
                Ingredient::CreamCheese,
                Ingredient::Salmon,
            ]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }}
            size={SushiSize { width: 120.0, height: 120.0 }}
        />
    }
}"##
                >
                    <Example10 />
                </ExampleCard>
                <ExampleCard
                    title="Red Caviar Edge + Custom Rice Color + Scale"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize,
    TopEdgeDecoration, TopEdgeType, NORI_COLOR,
};

#[function_component(Example11)]
pub fn example11() -> Html {
    html! {
        <Sushi
            id="caviar-roll"
            name="Red Caviar Roll"
            shape={SushiShape::Circular}
            ingredients={vec![Ingredient::RedCaviar]}
            outer_sheet={SushiOuterSheet {
                color: NORI_COLOR.into(),
                thickness: 10.0,
            }}
            size={SushiSize { width: 140.0, height: 140.0 }}
            top_edge={Some(TopEdgeDecoration {
                edge_type: TopEdgeType::RedCaviar,
                color: None,
            })}
            rice_color={Some("#FFF9E8".into())}
            scale={1.1}
        />
    }
}"##
                >
                    <Example11 />
                </ExampleCard>
                <ExampleCard
                    title="SushiGallery: Built-in Gallery Component"
                    code=r##"use yew::prelude::*;
use sushi_rs::yew::SushiGallery;
use sushi_rs::common::default_sushi_gallery;

#[function_component(Example12)]
pub fn example12() -> Html {
    html! {
        <SushiGallery items={default_sushi_gallery()} />
    }
}"##
                >
                    <Example12 />
                </ExampleCard>
            </div>
        </div>
    }
}
