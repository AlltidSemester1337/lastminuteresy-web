mod utils;

use wasm_bindgen::prelude::*;
use web_sys::Document;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, lastminuteresy-web!");
}


todo!(Maybe move this to separate file);
//#background-color:  #green <- free, clickable
#[wasm_bindgen]
pub fn mark_tables_as_bookable(num_free_tables:i32) {
    for i in 0..num_free_tables {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        //let body = document.body().expect("document expect to have have a body");
        let val = document.get_element_by_id("paragraphId")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        web_sys::console::log_2(&"URL: %s".into(),&JsValue::from_str(&val.inner_text()));
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let body = document.body().expect("document expect to have have a body");
        let val = document.create_element("p").unwrap();
        val.set_inner_html("Hello World from WebAssemblyMan!");
        body.append_child(&val).unwrap();
    }
}