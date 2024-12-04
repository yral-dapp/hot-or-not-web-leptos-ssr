use leptos::*;

#[component]
pub fn PageSelector(current_page: number, total_pages: number, previous_href: String, next_href: String, href: impl Fn(page: number) -> String) -> impl IntoView {
    view! {
			<div class="flex gap-1 text-sm text-[#A0A1A6] items-start font-medium">
				<a href=previous_href class="bg-[#3A3A3A] w-8 h-8 rounded-lg flex items-center justify-center">
					<ChevronRightIcon classes="w-4 h-4 rotate-180" />
				</a>
				total_pages.into_iter().map(|i| 
					let page_number = i + 1;
					view! {
						<a
							href=href(page_number)
							class="w-8 h-8 rounded-lg flex items-center justify-center"
							class=(
								["text-white bg-[#3D8EFF]"],
								move || page_number === current_page,
							)
							class=(["bg-[#3A3A3A]"], move || page_number !== current_page)
							
						
						>
							{page_number}
						</a>
					}).collect_view()
				<a href=next_href class="bg-[#3A3A3A] w-8 h-8 rounded-lg flex items-center justify-center">
					<ChevronRightIcon classes="w-4 h-4" />
				</a>
			</div>
		}
	}