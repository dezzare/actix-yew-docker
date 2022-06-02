use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    html! {
        <h1>{"Hello World!!!!!!"}</h1>
    }
}
