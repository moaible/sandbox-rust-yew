#![recursion_limit="512"]
#[macro_use]
extern crate stdweb;
extern crate yew;

use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::html;

pub struct Model {
    pub value: i64,
}

pub enum Msg {
}

impl html::Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: html::ComponentLink<Self>) -> Self {
        Model {
            value: 0,
        }
    }

    fn update(&mut self, _: Self::Message) -> html::ShouldRender {
        true
    }
}

impl html::Renderable<Model> for Model {
    fn view(&self) -> html::Html<Self> {
        html! {
            <>
                <h1>{ "Sample Title" }</h1>
            </>
        }
    }
}
