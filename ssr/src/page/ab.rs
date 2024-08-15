use leptos::*;
// use crate::abselector;
use crate::utils::ab_testing::{abselector_2comp, abselector_2compProps};

enum ViewEnum {
    Default,
    ViewB,
    ViewC,
}

impl IntoView for ViewEnum {
    fn into_view(self) -> leptos::View {
        match self {
            ViewEnum::Default => view! { <ComponentA /> },
            ViewEnum::ViewB => view! { <ComponentB /> },
            ViewEnum::ViewC => view! { <ComponentC /> },
        }
    }
}

#[component]
fn ComponentA() -> impl IntoView {
    view! {
        <div>
            <h1>Component A</h1>
            <p>This is Component A</p>
        </div>
    }
}

#[component]
fn ComponentB() -> impl IntoView {
    view! {
        <div>
            <h1>Component B</h1>
            <p>This is Component B</p>
        </div>
    }
}

#[component]
fn ComponentC() -> impl IntoView {
    view! {
        <div>
            <h1>Component C</h1>
            <p>This is Component C</p>
        </div>
    }
}

#[component]
pub fn ABTestTesting() -> impl IntoView {
    // let variations: Vec<(ABFlags, Box<dyn IntoView>)> = vec![
    //     (ABFlags::FlagA, Box::new(ComponentA)),
    //     (ABFlags::FlagB, Box::new(ComponentB)),
    //     (ABFlags::FlagC, Box::new(ComponentC)),
    // ];

    // // let comp = ab_chooser(Box::new(ComponentB), variations, Some("B".to_string()));

    // view! {
    //     {ABSelection!}
    // }

    // Example usage
    // abselector! {
    //     default: ComponentA,
    //     identifier: "flag_b",
    //     "flag_b" => ComponentB,
    //     "flag_c" => ComponentC,
    // }

    view! {
        {
            // abselector_2comp(Some("B".to_string()), ComponentA, ComponentB)
            abselector_2compProps { 
                identifier: Some("B".to_string()),
                component_a: ComponentA,
                component_b: ComponentB,
            }
        }
    }
}