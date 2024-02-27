use leptos::{html::Input, *};

#[component]
fn ToggleInner(
    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    #[prop(optional)]
    node_ref: NodeRef<Input>,
    #[prop(optional)] checked: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <label class="relative inline-flex items-center cursor-pointer z-0">
            <input checked=checked node_ref=node_ref type="checkbox" value="" class="sr-only peer"/>
            <div class="w-11 h-6 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-800 rounded-full peer bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all border-gray-600 peer-checked:bg-orange-600"></div>
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn Toggle(
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] checked: Signal<bool>,
) -> impl IntoView {
    view! { <ToggleInner node_ref checked/> }
}

#[component]
pub fn ToggleWithLabel(
    #[prop(into)] lab: String,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] checked: Signal<bool>,
) -> impl IntoView {
    view! {
        <ToggleInner node_ref checked>
            <span class="ms-3 text-md font-medium text-gray-300">{lab}</span>
        </ToggleInner>
    }
}
