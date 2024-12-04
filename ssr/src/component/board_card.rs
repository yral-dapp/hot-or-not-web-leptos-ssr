use leptos::*;

pub struct PropsBoardCard {
	pub image_url: String,
	pub title: String,
	pub symbol: String,
	pub description: String,
	pub created_by: String,
	pub created_at: String,
	pub nsfw: Option<bool>,
}


#[component]
pub fn BoardCard(href: String, card: PropsBoardCard) -> impl IntoView {

    let show_nsfw = create_rw_signal(false);

    view! {
			<div class="flex flex-col gap-2 py-3 px-3 w-full text-xs rounded-lg transition-colors md:px-4 hover:bg-gradient-to-b group bg-[#131313] font-kumbh hover:from-[#626262] hover:to-[#3A3A3A]">
				<div class="flex gap-3">
					<div
						style="box-shadow: 0px 0px 4px rgba(255, 255, 255, 0.16);"
						class="overflow-hidden relative w-[7rem] h-[7rem] rounded-[4px] shrink-0"
					>
						<Show when=move || card.nsfw.unwrap_or(false) && !show_nsfw.get()>
							<button
								on:click=move |_| show_nsfw.set(!show_nsfw.get())
								class="flex absolute inset-0 justify-center items-center w-full h-full z-[2] backdrop-blur-[4px] bg-black/50 rounded-[4px]"
							>
								<div class="flex flex-col gap-1 items-center text-xs">
									<EyeHiddenIcon classes="w-6 h-6".to_string() />
									<span class="uppercase">nsfw</span>
								</div>
							</button>
						</Show>
						<img alt=card.title.clone() src=card.image_url class="w-full h-full" />
					</div>
					<div class="flex flex-col gap-3 text-left">
						<div class="flex gap-4 justify-between items-center w-full text-lg">
							<span class="font-medium shrink line-clamp-1">{card.title}</span>
							<span class="font-bold shrink-0">{card.symbol}</span>
						</div>
						<span class="text-sm transition-colors group-hover:text-white line-clamp-2 text-[#A0A1A6]">
							{card.description}
						</span>
						<div class="flex gap-2 justify-between items-center text-sm font-medium group-hover:text-white text-[#505156]">
							<span class="line-clamp-1">"Created by" {card.created_by}</span>
							<span class="shrink-0">{card.created_at}</span>
						</div>
					</div>
				</div>
				<div class="flex gap-4 justify-between items-center p-2">
					<BoardCardButton label="Send".to_string() href="#".to_string()>
						<SendIcon classes="w-full h-full".to_string() />
					</BoardCardButton>
					<BoardCardButton label="Buy/Sell".to_string() href="#".to_string()>
						<ArrowLeftRightIcon classes="w-full h-full".to_string() />
					</BoardCardButton>
					<BoardCardButton label="Airdrop".to_string() href="#".to_string()>
						<AirdropIcon classes="w-full h-full".to_string() />
					</BoardCardButton>
					<BoardCardButton label="Share".to_string() href="#".to_string()>
						<ShareIcon classes="w-full h-full".to_string() />
					</BoardCardButton>
					<BoardCardButton label="Details".to_string() href=href>
						<ChevronRightIcon classes="w-full h-full".to_string() />
					</BoardCardButton>
				</div>
			</div>
		} 
}

#[component]
pub fn BoardCardButton(href: String, label: String, children: Children) -> impl IntoView {
	view! {
		<a
			href=href
			class="flex flex-col gap-1 justify-center items-center text-xs transition-colors group-hover:text-white text-[#A0A1A6]"
		>
			<div class="w-[1.875rem] h-[1.875rem]">
				{children()}
			</div>

			<div>{label}</div>
		</a>
	}
}	

