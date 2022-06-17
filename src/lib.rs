use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

mod components;

use components::main_title::{Color, MainTitle};

#[styled_component(App)]
pub fn app() -> Html {
    let main_title_load: Callback<String> = Callback::from(|message: String| log!(message));

    html! {
        <div>
            <MainTitle title="Olá  mundo!" color={Color::Ok} on_load={main_title_load}/>
        </div>
    }
}
