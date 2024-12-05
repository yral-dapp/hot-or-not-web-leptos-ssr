use leptos::*;

#[component]
pub fn PageSelector(current_page: u8, total_pages: u8, previous_href: String, next_href: String, href: impl Fn(u8) -> String) -> impl IntoView {
    let pages = (0..total_pages);
    view! {
        <div class="flex gap-1 items-start text-sm font-medium text-[#A0A1A6]">
            <a
                href=previous_href
                class="flex justify-center items-center w-8 h-8 rounded-lg bg-[#3A3A3A]"
            >
                <ChevronRightIcon classes="w-4 h-4 rotate-180" />
            </a>
            {pages
                .into_iter()
                .map(|i| {
                    let page_number = i + 1;
                    view! {
                        <a
                            href=href(page_number)
                            class=format!(
                                "w-8 h-8 rounded-lg flex items-center justify-center {}",
                                if page_number == current_page {
                                    "text-white bg-[#3D8EFF]"
                                } else {
                                    "bg-[#3A3A3A]"
                                },
                            )
                        >
                            {page_number}
                        </a>
                    }
                })
                .collect_view()}
            <a
                href=next_href
                class="flex justify-center items-center w-8 h-8 rounded-lg bg-[#3A3A3A]"
            >
                <ChevronRightIcon classes="w-4 h-4" />
            </a>
        </div>
    }
	}