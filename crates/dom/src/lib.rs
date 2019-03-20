use futures::prelude::*;
use futures::sync::mpsc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Element {
    raw: web_sys::HtmlElement,
}

impl Element {
    pub fn get_by_id(id: &str) -> Option<Element> {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)?
            .dyn_into()
            .ok()?;

        Some(Element { raw: element })
    }

    pub fn set_inner_text(&self, s: &str) {
        self.raw.set_inner_text(s);
    }
}

pub struct Input {
    raw: web_sys::HtmlInputElement,
}

impl Input {
    pub fn get_by_id(id: &str) -> Option<Input> {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)?
            .dyn_into()
            .ok()?;

        Some(Input { raw: element })
    }

    pub fn value(&self) -> String {
        self.raw.value()
    }
}

// TODO: docs
pub struct Button {
    raw: web_sys::HtmlButtonElement,
}

impl Button {
    pub fn get_by_id(id: &str) -> Option<Button> {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)?
            .dyn_into()
            .ok()?;

        Some(Button { raw: element })
    }

    pub fn clicks(&self) -> Events<MouseEvent> {
        let (sender, receiver) = mpsc::unbounded::<MouseEvent>();

        let handler = Closure::wrap(Box::new(move |event| {
            sender.unbounded_send(MouseEvent { raw: event }).unwrap();
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.raw
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .set_onclick(Some(handler.as_ref().unchecked_ref()));

        Events {
            closure: handler,
            inner: receiver,
        }
    }
}

pub struct MouseEvent {
    raw: web_sys::MouseEvent,
}

pub struct Events<T> {
    closure: Closure<dyn FnMut(web_sys::MouseEvent)>,
    inner: mpsc::UnboundedReceiver<T>,
}

impl<T> Stream for Events<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<T>, ()> {
        self.inner.poll()
    }
}
