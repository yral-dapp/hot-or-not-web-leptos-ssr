use leptos::{ev, prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_use::use_event_listener;

use crate::utils::web::FileWithUrl;
use leptos::html;
#[component]
pub fn TokenLogoSanitize(
    #[prop(into)] img_file: Signal<Option<FileWithUrl>, LocalStorage>,
    #[prop(into)] output_b64: SignalSetter<Option<String>>,
) -> impl IntoView {
    let img_ref: NodeRef<html::Img> = NodeRef::new();
    let canvas_ref: NodeRef<html::Canvas> = NodeRef::new();

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
            let im_w = img.width();
            let im_h = img.height();

            let min_dim = im_w.min(im_h);
            let canvas_dim = min_dim.min(200);
            canvas.set_width(canvas_dim);
            canvas.set_height(canvas_dim);

            let mut crop_x = 0.;
            let mut crop_y = 0.;
            let mut scaled_width = im_w as f64;
            let mut scaled_height = im_h as f64;

            if im_w > im_h {
                crop_x = (scaled_width - scaled_height) / 2.;
                scaled_width = scaled_height;
            } else {
                crop_y = (scaled_height - scaled_width) / 2.;
                scaled_height = scaled_width;
            }

            let canvas_dim_f64 = canvas_dim as f64;
            let ctx_raw = canvas.get_context("2d").unwrap().unwrap();
            let ctx: &CanvasRenderingContext2d = ctx_raw.dyn_ref().unwrap();
            ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                img,
                crop_x,
                crop_y,
                scaled_width,
                scaled_height,
                0.0,
                0.0,
                canvas_dim_f64,
                canvas_dim_f64,
            )
            .unwrap();

            let png_data = canvas.to_data_url_with_type("image/png").unwrap();

            output_b64.set(Some(png_data));
        }
    });

    view! {
        <canvas node_ref=canvas_ref class="hidden"></canvas>
        <Show when=move || img_file.with(|img| img.is_some())>
            <img
                node_ref=img_ref
                class="hidden"
                src=move || img_file.with(|f| f.as_ref().unwrap().url.to_string())
            />
        </Show>
    }
}
