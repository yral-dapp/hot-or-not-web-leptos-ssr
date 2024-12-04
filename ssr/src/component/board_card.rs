use leptos::*;

pub struct BoardCardProps {
	pub image_url: String,
	pub title: String,
	pub symbol: String,
	pub description: String,
	pub created_by: String,
	pub created_at: String,
	pub nsfw: Option<bool>,
}


#[component]
pub fn BoardCard(href: String, card: BoardCardProps) -> impl IntoView {
    let (show_nsfw, set_show_nsfw) = signal(false);

    view! {




<div
class="text-xs w-full py-3 px-3 md:px-4 flex rounded-lg flex-col gap-2 group transition-colors bg-[#131313] hover:from-[#626262] hover:to-[#3A3A3A] hover:bg-gradient-to-b font-kumbh"
>
<div class="flex gap-3">
	<div
		style="box-shadow: 0px 0px 4px rgba(255, 255, 255, 0.16);"
		class="relative w-[7rem] h-[7rem] rounded-[4px] overflow-hidden shrink-0"
	>
		if card.nsfw && !show_nsfw.get() {
			<button
				on:click={() => (showNsfw = true)}
				class="absolute inset-0 z-[2] w-full h-full backdrop-blur-[4px] bg-black/50 flex items-center justify-center rounded-[4px]"
			>
				<div class="text-xs flex flex-col items-center gap-1">
					<EyeHiddenIcon class="w-6 h-6" />
					<span class="uppercase">"Nsfw"</span>
				</div>
			</button>
		}
		<img alt={card.title} src={card.imageUrl} class="w-full h-full" />
	</div>
	<div class="flex flex-col gap-3 text-left">
		<div class="flex w-full items-center justify-between gap-4 text-lg">
			<span class="shrink line-clamp-1 font-medium">{card.title}</span>
			<span class="shrink-0 font-bold">{card.symbol}</span>
		</div>
		<span class="line-clamp-2 text-sm transition-colors text-[#A0A1A6] group-hover:text-white">
			{card.description}
		</span>
		<div
			class="flex items-center justify-between gap-2 text-[#505156] group-hover:text-white text-sm font-medium"
		>
			<span class="line-clamp-1">Created by {card.createdBy}</span>
			<span class="shrink-0">{card.createdAt}</span>
		</div>
	</div>
</div>
<div class="flex items-center justify-between gap-4 p-2">
	<BoardCardButton label="Send" href="#" >
		<SendIcon classes="w-full h-full" />
	</BoardCardButton>
	<BoardCardButton label="Buy/Sell" href="#" >
		<ArrowLeftRightIcon classes="w-full h-full" />
	</BoardCardButton>
	<BoardCardButton label="Airdrop" href="#" >
		<AirdropIcon classes="w-full h-full" />
	</BoardCardButton>
	<BoardCardButton label="Share" href="#" >
		<ShareIcon classes="w-full h-full" />
	</BoardCardButton>
	<BoardCardButton label="Details" href="#" >
		<ChevronRightIcon classes="w-full h-full" />
	</BoardCardButton>
</div>
</div>

		}
	}

#[component]
pub fn BoardCardButton(href: String, label: String, child: Children) -> impl IntoView {
    view! {
		<a
			{href}
			class="text-xs flex flex-col gap-1 items-center transition-colors group-hover:text-white justify-center text-[#A0A1A6]"
		>
			<div class="w-[1.875rem] h-[1.875rem]">{child()}</div>

			<div>{label}</div>
		</a>
	}
	}
	

