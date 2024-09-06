use crate::component::{back_btn::BackButton, title::Title};
use leptos::*;

#[derive(Clone)]
struct Section {
    question: String,
    answer: Option<String>, // Optional answer to the question.
}

#[component]
pub fn CreateTokenFAQ() -> impl IntoView {
    let sections1 = vec![
        Section {
            question: "Token name".to_string(),
            answer: Some(
                "Add a name to your cryptocurrency as per your choice & taste.".to_string(),
            ),
        },
        Section {
            question: "Description".to_string(),
            answer: None,
        },
        Section {
            question: "Token Symbol".to_string(),
            answer: None,
        },
        Section {
            question: "Supply".to_string(),
            answer: None,
        },
        Section {
            question: "What is ICP & do you want to raise ICP?".to_string(),
            answer: None,
        },
    ];

    // Example 2: Advanced settings
    let sections2 = vec![
        Section {
            question: "Token name".to_string(),
            answer: None,
        },
        Section {
            question: "Token name".to_string(),
            answer: None,
        },
    ];

    view! {
        <div class="w-dvw min-h-dvh bg-black" >
                 <Title justify_center=false >
                    <div class="grid grid-cols-3 justify-start w-full px-4" style="background: black" >
                        <BackButton fallback="/menu"/>
                        <span class="font-bold justify-self-center">Help</span>
                    </div>
                    </Title>

            // Render two different use cases of the component
            <CreateTokenFaqView title="Create a token".to_string() sections=sections1/>
            <CreateTokenFaqView title="For advanced settings".to_string() sections=sections2 />
        </div>
    }
}

#[component]
fn CreateTokenFaqView(title: String, sections: Vec<Section>) -> impl IntoView {
    let (open_section, set_open_section) = create_signal(None::<usize>);

    view! {
        <div class="bg-black text-white p-6">
            <header class="flex justify-between items-center mb-6">
                <h1 class="text-lg font-bold">{title.clone()}</h1>
            </header>

            <div class="space-y-4">
                {sections.clone().into_iter().enumerate().map(|(index, section)| {
                    let is_open = move || open_section() == Some(index);

                    view! {
                        <div class="border-b border-gray-200 dark:border-gray-700" >
                            <h2 id={format!("accordion-heading-{}", index)}>
                                <button
                                    type="button"
                                    class="flex items-center justify-between w-full py-5 font-medium text-white dark:text-gray-400 gap-3"
                                    on:click=move |_| {
                                        // Toggle section visibility
                                        set_open_section.update(|current| {
                                            if *current == Some(index) {
                                                *current = None;
                                            } else {
                                                *current = Some(index);
                                            }
                                        });
                                    }
                                >
                                    <span>{section.question.clone()}</span>
                                    <svg
                                        data-accordion-icon
                                        class={move || if is_open() { "w-3 h-3" } else { "w-3 h-3 rotate-180" }}
                                        aria-hidden="true"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 10 6"
                                    >
                                        <path
                                            stroke="currentColor"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M9 5 5 1 1 5"
                                        />
                                    </svg>
                                </button>
                            </h2>
                            <div id={format!("accordion-body-{}", index)} class={move || if is_open() { "" } else { "hidden" }}>
                                <div class="py-5">
                                    {if let Some(answer) = section.answer {
                                        view! {
                                            <p class="text-xs text-gray-400">{answer}</p>
                                        }
                                    } else {
                                        view! { <p></p> }
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }

    /*
       view! {

           <div class="min-h-screen bg-black text-white p-6">
               <header class="flex justify-between items-center mb-6">
                   <h1 class="text-lg font-bold">{title.clone()}</h1>
               </header>

               <div class="space-y-4">
                   {sections.clone().into_iter().map(|section| view! {

         <h2 id="accordion-flush-heading-3">
               <button type="button" class="flex items-center justify-between w-full py-5 font-medium rtl:text-right text-gray-500 border-b border-gray-200 dark:border-gray-700 dark:text-gray-400 gap-3" data-accordion-target="#accordion-flush-body-3" aria-expanded="false" aria-controls="accordion-flush-body-3">
                 <span>{section.question.clone()}</span>
                 <svg data-accordion-icon class="w-3 h-3 rotate-180 shrink-0" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                   <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5 5 1 1 5"/>
                 </svg>
               </button>
         </h2>
         <div id="accordion-flush-body-3" class="hidden" aria-labelledby="accordion-flush-heading-3">
               <div class="py-5 border-b border-gray-200 dark:border-gray-700">
                {if let Some(answer) = section.answer {
                               view! {
                                   <p class="text-xs text-gray-400">{answer}</p>
                               }
                           } else {
                               view! { <p></p> }
                           }}

                         </div>
             </div>
                  }).collect_view()}
               </div>
           </div>
       }
    */
}
