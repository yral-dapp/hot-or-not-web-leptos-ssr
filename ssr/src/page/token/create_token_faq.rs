use crate::component::{back_btn::BackButton, title::TitleText};
use leptos::*;
use leptos_meta::*;

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
                "The name of the token issued by the SNS ledger. Must be a string of 4 to 255 bytes without leading or trailing spaces.
E.g.: travel token".to_string(),
            ),
        },
        Section {
            question: "Description".to_string(),
            answer: Some("Give some details around what the token represents. Must be a string of at most 2,000 bytes".into()),
        },
        Section {
            question: "Token Logo".to_string(),
            answer: Some("PNG image which will act as logo of your created token.Must have less than 341,334 bytes. The only supported format is PNG.".into()),
        },
        Section {
            question: "Token Symbol".to_string(),
            answer: Some("The symbol of the token you would want. Must be a string of 3 to 5 without leading or trailing spaces.
E.g: TRAV".into()),
        },
        Section {
            question: "Token supply".to_string(),
            answer: Some("Total supply of the token. Please note the total supply of the token is halved and 50% of the supply is allocated to swap participants. E.g. If you give value of 10,000 the allocated developer’s token give to you is 5,000.".into()),
        },
    ];

    // Example 2: Advanced settings
    let sections2 = vec![
        Section {
            question: "Dapp cannister ID".to_string(),
            answer: Some("The dapp canister(ID) that will be decentralized if the decentralization swap succeeds. This is system defined and cannot be changed".into()),
        },
        Section {
            question: "Transaction fee".to_string(),
            answer: Some("Fee for sending, receiving token post creation (canister to canister sending)".into()),
        },
        Section {
            question: "Rejection fee".to_string(),
            answer: Some("Fee for proposal rejection once we raised the SNS proposal)".into()),
        },
        Section {
            question: "Initial voting period".to_string(),
            answer: Some("Duration for which the proposal remains live once raised.".into()),
        },
        Section {
            question: "Maximum wait for quite deadline extension".to_string(),
            answer: Some("Till how far into the sns swap process you can increase the duration for the swap".into()),
        },
        Section {
            question: "Minimum creation stake".to_string(),
            answer: Some("Minimum amount of tokens (e8s) to stake in each neuron".into()),
        },

        Section {
            question: "Minimum dissolve delay".to_string(),
            answer: Some("Time taken to disburse the liquid token from the neuron".into()),
        },
        Section {
            question: "Max age bonus duration".to_string(),
            answer: Some("Age at which participants will earn full bonus".into()),
        },
        Section {
            question: "Max age bonus %".to_string(),
            answer: Some("% reward post maximum age is hit".into()),
        },
        Section {
            question: "Minimum participant".to_string(),
            answer: Some("Min number of participant required for execution of SNS proposal".into()),
        },
        Section {
            question: "Minimum direct participation icp".to_string(),
            answer: Some("Minimum token required when direct participant is involved".into()),
        },
        Section {
            question: "Maximum direct participation icp".to_string(),
            answer: Some("Maximum token required when direct participant is involved".into()),
        },
    ];

    view! {
        <Title text="ICPump - Token FAQ" />
        <div class="w-dvw min-h-dvh bg-black" style="padding-bottom:5rem;">
            <TitleText justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full px-4" style="background: black">
                    <BackButton fallback="/menu" />
                    <span class="font-bold justify-self-center">Help</span>
                </div>
            </TitleText>

            // Render two different use cases of the component
            <CreateTokenFaqView title="Create a token".to_string() sections=sections1 />
            <CreateTokenFaqView title="Advanced settings".to_string() sections=sections2 />
        </div>
    }
}

#[component]
fn CreateTokenFaqView(title: String, sections: Vec<Section>) -> impl IntoView {
    let (open_section, set_open_section) = create_signal(None::<usize>);

    view! {
        <div class="bg-black text-white p-4">
            <header class="flex justify-between items-centers">
                <h1 class="text-lg font-bold">{title.clone()}</h1>
            </header>

            <div>
                {sections
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(index, section)| {
                        let is_open = move || open_section() == Some(index);
                        view! {
                            <div class="border-b border-gray-200 dark:border-gray-700 py-2 ">
                                <h2 id=format!("accordion-heading-{}", index)>
                                    <button
                                        type="button"
                                        class="flex items-center justify-between w-full py-2 text-md font-medium text-white gap-3"
                                        on:click=move |_| {
                                            set_open_section
                                                .update(|current| {
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
                                            class=move || {
                                                if is_open() { "w-3 h-3" } else { "w-3 h-3 rotate-180" }
                                            }

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
                                            ></path>
                                        </svg>
                                    </button>
                                </h2>
                                <div
                                    id=format!("accordion-body-{}", index)
                                    class=move || if is_open() { "" } else { "hidden" }
                                >
                                    {if let Some(answer) = section.answer {
                                        view! {
                                            <div class="py-2 mb-2">
                                                <p class="text-xs text-gray-400">{answer}</p>
                                            </div>
                                        }
                                    } else {
                                        view! { <div></div> }
                                    }}

                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
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
