use leptos::*;

#[component]
pub fn NotificationIcon(#[prop(optional, default = "w-full h-full".to_string())] classes: String, #[prop(optional)] show_dot: bool) -> impl IntoView {
			if show_dot {
				view! {
                    <svg
                        format!("{}" ,classes)
                        viewBox="0 0 32 32"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M18.3067 28C18.0723 28.4041 17.7358 28.7395 17.331 28.9727C16.9261 29.2058 16.4672 29.3286 16 29.3286C15.5328 29.3286 15.0739 29.2058 14.669 28.9727C14.2642 28.7395 13.9277 28.4041 13.6933 28M24 10.6666C24 8.54489 23.1571 6.51006 21.6569 5.00977C20.1566 3.50948 18.1217 2.66663 16 2.66663C13.8783 2.66663 11.8434 3.50948 10.3431 5.00977C8.84286 6.51006 8 8.54489 8 10.6666C8 20 4 22.6666 4 22.6666H28C28 22.6666 24 20 24 10.6666Z"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <circle
                            cx="26"
                            cy="6"
                            r="5"
                            fill="#E2017B"
                            stroke="black"
                            stroke-width="2"
                        />
                    </svg>
                }
			} else {
				view! {
                    <svg
                        format!("{}" ,classes)
                        viewBox="0 0 32 32"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M18.3067 28C18.0723 28.4041 17.7358 28.7395 17.331 28.9727C16.9261 29.2058 16.4672 29.3286 16 29.3286C15.5328 29.3286 15.0739 29.2058 14.669 28.9727C14.2642 28.7395 13.9277 28.4041 13.6933 28M24 10.6666C24 8.54489 23.1571 6.51006 21.6569 5.00977C20.1566 3.50948 18.1217 2.66663 16 2.66663C13.8783 2.66663 11.8434 3.50948 10.3431 5.00977C8.84286 6.51006 8 8.54489 8 10.6666C8 20 4 22.6666 4 22.6666H28C28 22.6666 24 20 24 10.6666Z"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                    </svg>
                }
			}
    }
	