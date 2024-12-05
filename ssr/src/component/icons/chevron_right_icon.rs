use leptos::*;

#[component]
pub fn ChevronRightIcon(#[prop(optional, default = "w-full h-full".to_string())] classes: String) -> impl IntoView {
    view! {
        <svg
            class=format!("{}", classes)
            viewBox="0 0 30 30"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M12.1224 24.0156C11.7522 24.3858 11.1518 24.3858 10.7815 24.0156V24.0156C10.4113 23.6453 10.4113 23.0449 10.7815 22.6746L17.891 15.5652L10.7815 8.45569C10.4113 8.08541 10.4113 7.48507 10.7815 7.11479V7.11479C11.1518 6.74451 11.7522 6.74451 12.1224 7.11479L19.8657 14.8581C20.2562 15.2486 20.2562 15.8818 19.8657 16.2723L12.1224 24.0156Z"
                fill="currentColor"
            />
        </svg>
    }
	}