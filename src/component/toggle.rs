use leptos::{html::Input, *};

#[component]
pub fn Toggle(
    #[prop(into)] lab: String,
    #[prop(optional)] node_ref: Option<NodeRef<Input>>,
) -> impl IntoView {
    view! {
        <label class="relative inline-flex items-center cursor-pointer z-0">
            {move || {
                if let Some(_ref) = node_ref {
                    view! { <input _ref=_ref type="checkbox" value="" class="sr-only peer"/> }
                } else {
                    view! { <input type="checkbox" value="" class="sr-only peer"/> }
                }
            }}
            <div class="w-11 h-6 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-800 rounded-full peer bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all border-gray-600 peer-checked:bg-orange-600"></div>
            <span class="ms-3 text-md font-medium text-gray-300">{lab}</span>
        </label>
    }
}
