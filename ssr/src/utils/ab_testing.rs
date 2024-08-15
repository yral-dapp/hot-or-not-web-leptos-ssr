use leptos::*;
use std::collections::HashSet;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ABFlags {
    FlagA,
    FlagB,
    FlagC,
    // Add more flags as needed
}

// pub fn ab_chooser<V: IntoView>(variations: Vec<(ABFlags, V)>, identifier: Option<String>) -> impl IntoView {
//     // let user_flags = create_resource(|| (), |_| {
//     //     // Dummy implementation: Assume the user has FlagA and FlagC enabled
//     //     let enabled_flags = HashSet::from([ABFlag::FlagA, ABFlag::FlagC]);
//     //     futures::future::ready(enabled_flags)
//     // });

//     let flag_to_view = match identifier {
//         Some(id) => {
//             match id.as_str() {
//                 "A" => ABFlags::FlagA,
//                 "B" => ABFlags::FlagB,
//                 "C" => ABFlags::FlagC,
//                 _ => ABFlags::FlagA,
//             }
//         },
//         None => ABFlags::FlagA,
//     };

//     // // Ensure that `selected_view` is a closure that implements `Fn`
//     // let selected_view = {
//     //     let variations = variations.clone();  // Clone the variations to avoid moving them
//     //     move || {
//     //         user_flags.get().map(|flags| {
//     //             variations.iter().find_map(|(flag, view)| {
//     //                 if flags.contains(flag) {
//     //                     Some(view.clone()) // Return the corresponding view if the flag matches
//     //                 } else {
//     //                     None
//     //                 }
//     //             })
//     //             .unwrap_or_else(|| default_view.clone()) // Fallback to default_view if no flags match
//     //         }).unwrap_or_else(|| default_view.clone()) // Fallback in case the resource is not ready
//     //     }
//     // };

//     // let selected_view = variations.iter().find_map(|(flag, view)| {
//     //     if flag == &flag_to_view {
//     //         Some(view.clone())
//     //     } else {
//     //         None
//     //     }
//     // }).unwrap_or_else(|| default_view);

//     // view! {
//     //     <Suspense fallback={move || { default_view.clone() }}>
//     //         {selected_view()}
//     //     </Suspense>
//     // }

//     // return default
//     // let mut selected_view = default_view;
//     // for (flag, view) in variations {
//     //     if flag == flag_to_view {
//     //         selected_view = view;
//     //         break;
//     //     }
//     // }

//     view! {
//         {
//             if !variations.is_empty() {
//                 // Return the first element as a View
//                 view! { <div> {variations[0].1} </div> }.into_view()
//             } else {
//                 // Return a fallback View
//                 view! { <div> No variations found </div> }.into_view()
//             }
//         }
//     }
// }

// pub fn ab_chooser<V: IntoView>(variations: Vec<(ABFlags, V)>, identifier: Option<String>) -> impl IntoView {
//     return variations[0].1;
// }

#[component]
pub fn abselector_2comp<CompA: Fn() -> AIV, CompB: Fn() -> AIV, AIV: IntoView>(identifier: Option<String>, component_a: CompA, component_b: CompB) -> impl IntoView {
    let mut flags = vec![false; 2];

    // function to input the identifier and enable a flag
    flags[match identifier {
        Some(id) => {
            match id.as_str() {
                "A" => 0,
                "B" => 1,
                _ => 0,
            }
        },
        None => 0,
    }] = true;

    let b_enabled = flags[1];

    view! {
        <Suspense fallback = component_a>
            <Show when=move || {b_enabled} fallback=component_a>
                component_b
            </Show>
        </Suspense>
    }

    // return component_a or component_b based on the flag
    // if b_enabled {
    //     component_b
    // } else {
    //     component_a
    // }
}

#[macro_export]
macro_rules! abselectorold {
    (
        default: $default_view:expr,
        identifier: $identifier:expr,
        $($flag:expr => $view:expr),* $(,)?
    ) => {
        pub fn ab_chooser() -> impl leptos::IntoView {
            let identifier = $identifier;

            let selected_view = move || {
                $(
                    if identifier == $flag {
                        return $view;
                    }
                )*
                $default_view
            };

            view! {
                <Suspense fallback=$default_view>
                    { selected_view() }
                </Suspense>
            }
        }
    };
}

// #[macro_export]
// macro_rules! abselector {
//     (
//         default: $default_view:expr,
//         identifier: $identifier:expr,
//         $($flag:expr => $view:expr),* $(,)?
//     ) => {
//         pub fn ab_chooser() -> impl leptos::IntoView {
//             let identifier = $identifier;

//             let selected_view = move || {
//                 $(
//                     if identifier == $flag {
//                         return $view.into_view(); // Ensure that we call `into_view` to convert it to the common type
//                     }
//                 )*
//                 $default_view.into_view() // Ensure that the default view is also converted to the common type
//             };

//             view! {
//                 <Suspense fallback=$default_view.into_view()>
//                     { selected_view() }
//                 </Suspense>
//             }
//         }
//     };
// }

// // Example usage:
// ABSelection! {
//     default: default_view(),
//     identifier: "some_identifier".to_string(),
//     "flag_a" => view_a(),
//     "flag_b" => view_b(),
//     "flag_c" => view_c(),
// }
