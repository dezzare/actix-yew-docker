use gloo::console::log;
use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
struct MyStruct {
    first_name: String,
    last_name: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name: &str = "Pedro";
    let my_struct: MyStruct = MyStruct {
        first_name: name.to_owned(),
        last_name: "Bittencourt".to_string(),
    };

    log!(name);
    log!(serde_json::to_string_pretty(&my_struct).unwrap());

    let class_name: &str = "classe";
    let message: Option<&str> = Some("Mensagem");

    let to_do_list = vec![
        "ir no mercado",
        "comprar cerveja",
        "pedir um X",
        "assistir aula",
    ];

    html! {
        <>
            <h1 class={class_name}>{"Hello World!"}</h1>

            if class_name == "classe" {
                <p>{"Olá"}</p>
            } else {
                <p>{"Else"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"sem mensagem"}</p>
            }

            <ul>
                {to_do_list.iter().map(|item| html!{<li>{item}</li>}).collect::<Html>()}
            </ul>

        </>
    }
}
