use leptos::*;

#[component]
pub fn ArrowDownIcon(#[prop(optional, default = "w-full h-full".to_string())] classes: String) -> impl IntoView {
    view! {
        <svg format!("{}" ,classes) viewBox="0 0 19 20" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M1.96481 11.4648L9.49998 19L17.0352 11.4648"
                stroke="currentColor"
                stroke-width="2"
                stroke-miterlimit="10"
                stroke-linecap="square"
                stroke-linejoin="round"
            />
            <path
                d="M9.5 0.999962L9.5 18.3793"
                stroke="currentColor"
                stroke-width="2"
                stroke-miterlimit="10"
                stroke-linecap="square"
                stroke-linejoin="round"
            />
        </svg>
    }
	}