use leptos::*;

use crate::component::buttons::SecondaryButton;
use crate::component::icons::chevron_right_icon::ChevronRightIcon;
use crate::component::icons::chevron_left_icon::ChevronLeftIcon;

#[component]
fn NotificationPage() -> impl IntoView {

  let all_notifications = create_signal(vec![]);

  view! {
      {if (show_popup.get()) {
          view! { <RequestPopup /> }
      }}

      <div class="h-screen w-screen block text-white bg-black font-kumbh">

          <div class="h-full w-full py-20">
              <div class=format!(
                  "flex flex-col items-center justify-center gap-3 max-w-md mx-auto px-4 {}",
                  if all_notifications.get().len() > 0 { "pt-20" } else { "" },
              )>
                  {if (all_notifications.get().len() > 0) {
                      view! {
                          <For each=move || all_notifications.get()>
                              <NotificationItem
                                  status=notification.status
                                  notification_text=notification.notification_text
                                  status_expired_at=notification.status_expired_at
                              />
                          </For>
                          <NotificationItem
                              status="pending".to_string()
                              notification_text="32 BETTA has has been sent to yvlkr-usbct-hle46-xbfmr-w4r4h-uaahu-bsdco-3blze-imj7k-5oeb3-mae"
                                  .to_string()
                              status_expired_at="2 days ago".to_string()
                          />
                      }
                  } else {
                      view! {
                          <div class="text-zinc-500 font-medium pb-4">"No notifications"</div>
                          <SecondaryButton href="/wallet".to_string() alt_style=true>
                              "Go Back"
                          </SecondaryButton>
                      }
                  }}
              </div>
          </div>
      </div>
  }
}

#[component]
pub fn NotificationItem(status: String, status_expired_at: String, notification_text: String) -> impl IntoView {
  let request = create_signal(None);
  let show_popup = create_signal(false);
  let accept_request = create_action(move |_| {});
  let reject_request = create_action(move |_| {});
  let error = create_signal("".to_string());

  view! {
      <div class="flex flex-col gap-4 rounded-sm w-full p-3">
          <div class="flex items-center justify-between">
              <div class="line-clamp-2">
                  {if (request.get().is_some()) {
                      view! {
                          "Swap request for"
                          <span class="font-bold">
                              {request.details.amount} {request.toToken.symbol}
                          </span>
                          to
                          <span class="font-bold">
                              {request.details.amount * 0.854} {request.fromToken.symbol}
                          </span>
                          by
                          <span class="font-bold">{request.details.fromAddress}</span>
                      }
                  } else {
                      view! { {notificationText.get()} }
                  }}
              </div>
              <button on:click=move |_| show_popup.set(true)>
                  <ChevronRightIcon classes="w-6 h-6 shrink-0".to_string() />
              </button>
          </div>
          <div class="flex items-center justify-between gap-4 text-sm">
              {if (request.get().is_some()) {
                  view! {
                      <div class="flex items-center gap-4">
                          <SecondaryButton on_click=accept_request>"Accept"</SecondaryButton>
                          <SecondaryButton alt_style=true on_click=reject_request>
                              "Reject"
                          </SecondaryButton>
                      </div>
                  }
              } else if status {
                  view! {
                      <div>
                          <NotificationStatus status=status expiredAt=status_expired_at />
                      </div>
                  }
              }}
          </div>
          {if (error.get().is_some()) {
              view! { <div class="text-xs font-medium text-red-500">* {error.get()}</div> }
          }}
      </div>
  }
}

#[component]
pub fn NotificationStatus() -> impl IntoView {
  let status = create_signal(None);
  let expired_at = create_signal(None);

    view! {
        <div class=format!(
            "px-2 py-1 text-xs rounded-full {status.get() === 'expired' ? 'bg-[#505156]/20 text-[#505156]' : ''} {status.get() === 'pending' ? 'bg-[#B38929]/20 text-[#B38929]' : ''} {status.get() === 'accepted' ? 'bg-[#158F5C]/20 text-[#158F5C]' : ''} {status.get() === 'rejected' ? 'bg-[#AB3023]/20 text-[#AB3023]' : ''}",
        )>
            {if (expired_at.get().is_some()) {
                view! {
                    <p>Request expired at {new Date(expired_at.get().unwrap()).toDateString()}</p>
                }
            } else {
                view! { <p>Status: {status.get()}</p> }
            }}

        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
  view! {
      <div class="bg-black z-[4] fixed top-0 w-full select-none inset-x-0">
          <div class="flex items-center justify-center relative gap-3 px-4 py-3 mx-auto max-w-md">
              <div class="text-xl font-semibold">"Notifications"</div>
              <a href="/wallet" class="absolute z-[5] left-0 px-4">
                  <ChevronLeftIcon classes="w-4 h-4".to_string() />
              </a>
          </div>
      </div>
  }
}
