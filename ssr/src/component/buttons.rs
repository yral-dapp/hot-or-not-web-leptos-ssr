use gloo::history::{BrowserHistory, History};
use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_navigate;
use reqwest::Url;

#[component]
pub fn Button(
  child: Children,
  #[prop(into)] on_press: Callback<PressEvent>,
  #[prop(optional)] classes: Option<String>,
  #[prop(optional)] alt_style: Option<bool>,
  #[prop(optional)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
 view! {
     <button
         {disabled}
         on:click
         class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
         class=(["text-white/50"], move || disabled.get())
         class=(["text-[#E2017B]"], move || alt_style.get())
         class=(["text-white"], move || (!disabled.get() && !alt_style.get()))
         class=(classes, move || classes.get())
         style=(
             "background: linear-gradient(73deg, #DE98BE 0%, #E761A9 33%, #7B5369 100%",
             move || disabled.get(),
         )
         style=("background: linear-gradient(73deg, #FFF 0%, #FFF 1000%", move || alt_style.get())
         style=(
             "background: linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%",
             move || (!disabled.get() && !alt_style.get()),
         )
     >
         {children()}
     </button>
 }
    }
  

#[component]
pub fn LinkButton(
  child: Children,
  href: String,
  #[prop(optional)] classes: Option<String>,
  #[prop(optional)] alt_style: Option<bool>,
  #[prop(optional)] disabled: MaybeSignal<bool>,
) -> impl IntoView { 
   view! {
       <a
           {href}
           {disabled}
           class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
           class=(["text-white/50"], move || disabled.get())
           class=(["text-[#E2017B]"], move || alt_style.get())
           class=(["text-white"], move || (!disabled.get() && !alt_style.get()))
           class=(classes, move || classes.get())
           style=(
               "background: linear-gradient(73deg, #DE98BE 0%, #E761A9 33%, #7B5369 100%",
               move || disabled.get(),
           )
           style=("background: linear-gradient(73deg, #FFF 0%, #FFF 1000%", move || alt_style.get())
           style=(
               "background: linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%",
               move || (!disabled.get() && !alt_style.get()),
           )
       >
           {children()}
       </a>
   }
      
}



#[component]
pub fn SecondaryLinkButton(href: String, child: Children, alt_style: MaybeSignal<bool>) -> impl IntoView {
    view! {
        <a
            {href}
            class="rounded-full border border-white text-sm font-bold font-kumbh px-5 py-2"
            class=(
                ["bg-transparent text-white hover:bg-white/10 active:bg-white/5"],
                move || alt_style.get(),
            )
            class=(["bg-white text-black"], move || !alt_style.get())
        >
            {children()}
        </a>
    }
}



#[component]
pub fn SecondaryButton(child: Children, disabled: MaybeSignal<bool>, alt_style: MaybeSignal<bool>, on_press: Callback<PressEvent>) -> impl IntoView {
    view! {
        <button
            {disabled}
            on:click=on_press
            class="rounded-full border border-white text-sm font-bold font-kumbh px-5 py-2"
            class=(
                ["bg-transparent text-white hover:bg-white/10 active:bg-white/5"],
                move || alt_style.get(),
            )
            class=(["bg-white text-black"], move || !alt_style.get())
        >

            {children()}
        </button>
    }
}
