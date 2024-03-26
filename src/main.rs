mod model;

use crate::model::HuntTemplate;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let hunt = HuntTemplate::default().generate();

    html! {
        <div>
            <h1>{"Scavenger Hunt!"}</h1>
            <h2>{hunt.name}</h2>
            <ul>
                {hunt.items.iter().map(|item| html!{
                    <li>{format!("{} {}", item.quantity, item.name)}</li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
