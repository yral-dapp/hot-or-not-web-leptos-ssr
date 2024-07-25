use leptos::*;
use leptos_use::use_event_listener;

use crate::utils::web::FileWithUrl;

#[component]
pub fn ImgToPng(
    #[prop(into)] img_file: Signal<Option<FileWithUrl>>,
    #[prop(into)] output_b64: SignalSetter<Option<String>>,
) -> impl IntoView {
    let img_ref: NodeRef<html::Img> = create_node_ref();
    let canvas_ref: NodeRef<html::Canvas> = create_node_ref();

    _ = use_event_listener(img_ref, ev::load, move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

            let Some(canvas_elem) = canvas_ref.get_untracked() else {
                return;
            };
            let canvas: &HtmlCanvasElement = &canvas_elem;
            let img_elem = img_ref.get_untracked().unwrap();
            let img: &HtmlImageElement = &img_elem;

            canvas.set_width(img.width());
            canvas.set_height(img.height());

            let ctx_raw = canvas.get_context("2d").unwrap().unwrap();
            let ctx: &CanvasRenderingContext2d = ctx_raw.dyn_ref().unwrap();
            ctx.draw_image_with_html_image_element(img, 0.0, 0.0)
                .unwrap();

            let png_data = canvas.to_data_url_with_type("image/png").unwrap();

            output_b64.set(Some(png_data));
        }
    });

    view! {
        <canvas _ref=canvas_ref class="hidden"/>
        <Show when=move || img_file.with(|img| img.is_some())>
            <img _ref=img_ref class="hidden" src=move || img_file.with(|f| f.as_ref().unwrap().url.to_string())/>
        </Show>
    }
}
