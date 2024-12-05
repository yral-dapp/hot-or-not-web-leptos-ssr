
use leptos::*;

#[component]
pub fn ChevronLeftIcon(#[prop(optional, default = "w-full h-full".to_string())] classes: String) -> impl IntoView {
    view! {
        <svg
            class=format!("{}", classes)
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M15.2404 2.03981C15.7611 1.47858 16.6491 1.47858 17.1697 2.03981C17.638 2.54456 17.638 3.32496 17.1697 3.82972L9.58989 12L17.1697 20.1703C17.638 20.675 17.638 21.4554 17.1697 21.9602C16.6491 22.5214 15.7611 22.5214 15.2404 21.9602L7.26194 13.3602C6.55021 12.5931 6.55021 11.4069 7.26194 10.6398L15.2404 2.03981Z"
                fill="currentColor"
            />
        </svg>
    }
	}