use leptos::*;

#[component]
pub fn ICPumpLanding() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-black text-white overflow-y-scroll pt-10 pb-12">
                <div class="flex ml-4">
                    <BackButton fallback="/".to_string()/>
                </div>
                <div class="grid grid-cols-1 gap-5 justify-normal justify-items-center w-full">
                    <div class="flex flex-row w-11/12 sm:w-7/12 justify-center">
                        <div class="flex flex-col justify-center items-center">
                            <img
                                class="h-24 w-24 rounded-full"
                                alt=username_or_principal.clone()
                                src=profile_pic
                            />
                            <div class="flex flex-col text-center items-center">
                                <span
                                    class="text-md text-white font-bold"
                                    class=("w-full", is_connected)
                                    class=("w-5/12", move || !is_connected())
                                    class=("truncate", move || !is_connected())
                                >
                                    {display_name}
                                </span>
                                <div class="text-sm flex flex-row">
                                    // TODO: Add username when it's available
                                    // <p class="text-white">@ {username_or_principal}</p>
                                    <p class="text-primary-500">{earnings} Earnings</p>
                                </div>
                                <Show when=move || !is_connected()>
                                    <div class="md:w-4/12 w-6/12 pt-5">
                                        <ConnectLogin cta_location="profile"/>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    </div>
                    <div class="flex justify-around text-center rounded-full divide-x-2 divide-white/20 bg-white/10 p-4 my-4 w-11/12 sm:w-7/12">
                        // <Stat stat=user.followers_cnt info="Lovers"/>
                        // <Stat stat=user.following_cnt info="Loving"/>
                        <Stat stat=user.hots info="Hots"/>
                        <Stat stat=user.nots info="Nots"/>
                    </div>
                </div>
            </div>
    }
}
