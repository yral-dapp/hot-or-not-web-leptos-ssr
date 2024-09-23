use leptos::*;

const QUERY_LIST: [&str; 3] = ["Dog token", "Cat token", "Bird token"];

#[component]
pub fn ICPumpSearch() -> impl IntoView {
    let query = create_rw_signal("".to_string());

    let query_list = QUERY_LIST.to_vec();

    let search_action = move || {
        let q = query.get();
        leptos::logging::log!("Search query: {}", q);
        query.set("".to_string());
    };

    view! {
        <div class="min-h-screen bg-black text-white overflow-y-scroll pt-5 pb-12">
            <div class="flex flex-col justify-center mt-10 mb-10">
                <div class="grid grid-col-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <For
                        each=move || query_list.clone()
                        key=|t| t.to_string()
                        children=move |token_query: &str| {
                        view! {
                            <button class="text-white bg-gray-800 p-4 rounded-lg" on:click=move |_| {
                                query.set(token_query.to_string());
                                search_action();
                            }>{token_query}</button>
                        }
                        }
                    />
                </div>
            </div>
            <div class="flex ml-4 space-x-2">
                <input class="bg-gray-800 text-white p-2 rounded-lg" type="text" placeholder="Search for a token"
                on:input=move |ev| {
                    let q = event_target_value(&ev);
                    query.set(q);
                } />
                <button class="bg-gray-800 text-white p-2 rounded-lg"
                on:click=move |_| search_action()
                >Search</button>
            </div>
        </div>
    }
}
