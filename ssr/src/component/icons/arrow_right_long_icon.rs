use leptos::*;

#[component]
pub fn ArrowRightLongIcon(#[prop(optional, default = "w-full h-full".to_string())] classes: String) -> impl IntoView {
    view! {
        <svg
            class=format!("{}", classes)
            viewBox="0 0 38 19"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M29.4648 17.0352L37 9.50002L29.4648 1.96484"
                stroke="currentColor"
                stroke-width="2"
                stroke-miterlimit="10"
                stroke-linecap="square"
                stroke-linejoin="round"
            />
            <path
                d="M1 9.5L36 9.5"
                stroke="currentColor"
                stroke-width="2"
                stroke-miterlimit="10"
                stroke-linecap="square"
                stroke-linejoin="round"
            />
        </svg>
    }
	}