use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::main_title::{Color, MainTitle};

const STYLESHEE_FILE: &str = include_str!("style.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet: Style = Style::new(STYLESHEE_FILE).unwrap();
    html! {
        <div class={stylesheet}>
            <MainTitle title="Olá  mundo!" color={Color::Ok} />
        </div>
    }
}
