use futures::prelude::*;
use futures::stream;
use gloo_timers::IntervalStream;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn run() {
    spawn_local(smart_counter());
}

fn smart_counter() -> impl Future<Item = (), Error = ()> {
    let input = gloo_dom::Input::get_by_id("range")
        .expect("expected input element with id #range to exist");
    let update_btn = gloo_dom::Button::get_by_id("update")
        .expect("expected button element with id #update to exist");

    update_btn
        .clicks()
        .map(move |_| input.value().parse::<i32>().unwrap())
        .for_each(move |end| {
            IntervalStream::new(20)
                .zip(stream::iter_ok(0..end))
                .for_each(|(_, num)| {
                    gloo_dom::Element::get_by_id("display")
                        .expect("expected element with id #display to exist")
                        .set_inner_text(&num.to_string());
                    Ok(())
                })
        })
}
