use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
fn NavIcon(
    idx: usize,
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] icon: icondata_core::Icon,
    cur_selected: Memo<usize>,
) -> impl IntoView {
    view! {
        <a href=href class="flex items-center justify-center">
            <Show
                when=move || cur_selected() == idx
                fallback=move || view! { <Icon icon=icon class="text-white h-6 w-6"/> }
            >
                <Icon icon=icon class="text-orange-600 h-6 w-6"/>
                <div class="absolute bottom-0 bg-orange-600 py-1 w-6 blur-md"></div>
            </Show>
        </a>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let cur_location = use_location();
    let home_path = create_rw_signal("/".to_string());
    let cur_selected = create_memo(move |_| {
        let path = cur_location.pathname.get();
        match path.as_str() {
            "/" => 0,
            "/upload" => 1,
            "/menu" => 2,
            s if s.starts_with("/hot-or-not") => {
                home_path.set(path);
                0
            }
            _ => 2,
        }
    });

    view! {
        <div class="flex flex-row justify-between px-4 py-5 w-full bg-transparent fixed left-0 bottom-0 z-50">
            <NavIcon idx=0 href=home_path icon=icondata::TbHome cur_selected=cur_selected/>
            <NavIcon
                idx=1
                href="/upload"
                icon=icondata::AiPlusCircleFilled
                cur_selected=cur_selected
            />
            <NavIcon idx=2 href="/menu" icon=icondata::AiMenuOutlined cur_selected=cur_selected/>
        </div>
    }
}
