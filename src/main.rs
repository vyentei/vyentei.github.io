use async_main::{async_main, LocalSpawner};
use fetchy::Fetch;
use marksad::{Decoder, html::HtmlEncoder};

#[async_main]
async fn main(_spawner: LocalSpawner) {
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
    let decoder = Decoder::from_str(&text);
    let mut html = Vec::new();
    let mut encoder = HtmlEncoder::new(decoder.filter_map(|md| md.ok()), &mut html);

    encoder.encode_html().unwrap();
    
    let html = String::from_utf8(html).unwrap();

    main.set_inner_html(&html);

    /*for md in decoder {
        let p = document.create_element("p").unwrap();

        p.set_text_content(Some(&format!("{md:?}")));
        main.append_child(&p).unwrap();
    }*/

    body.append_child(&main).unwrap();
}
