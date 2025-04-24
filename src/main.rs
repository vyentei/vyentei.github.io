use async_main::{LocalSpawner, async_main};
use fetchy::Fetch;
use marksad::{Decoder, html::HtmlEncoder};
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;

#[async_main]
async fn main(_spawner: LocalSpawner) {
    std::panic::set_hook(Box::new(web_panic_hook::hook));

    // FIXME: Load in parts
    let text = Fetch::builder("/INDEX.md")
        .fetch()
        .all()
        .await
        .map_err(|_e| "Failed...")
        .unwrap();
    let text = String::from_utf8_lossy(&text);
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let main = document.create_element("main").unwrap();

    main.set_id("main");

    let decoder = Decoder::from_str(&text);
    let mut html = Vec::new();
    let mut encoder =
        HtmlEncoder::new(decoder.filter_map(|md| md.ok()), &mut html);

    encoder.encode_html().unwrap();

    let html = String::from_utf8(html).unwrap();

    main.set_inner_html(&html);
    body.append_child(&main).unwrap();

    let main = document.get_element_by_id("main").unwrap();
    let element_list = main.children();

    for i in 0..element_list.length() {
        let element = element_list.get_with_index(i).unwrap();

        if element.text_content().as_deref() == Some("REPLACE{TRY_IT_OUT}") {
            element.set_text_content(None);

            let text_area = document.create_element("textarea").unwrap();
            let test_text = document.create_element("h5").unwrap();

            text_area.set_attribute("rows", "3").unwrap();
            element.append_child(&text_area).unwrap();
            element.append_child(&test_text).unwrap();

            let text_area_clone =
                text_area.clone().dyn_into::<HtmlTextAreaElement>().unwrap();

            let cb: Closure<dyn Fn()> = Closure::new(move || {
                test_text.set_text_content(Some(&text_area_clone.value()))
            });

            text_area
                .add_event_listener_with_callback(
                    "input",
                    cb.as_ref().unchecked_ref(),
                )
                .unwrap();
            cb.forget();
        }
    }
}
