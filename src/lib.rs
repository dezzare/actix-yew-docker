use gloo::console::log;
use serde::Serialize;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Serialize)]
struct MyStruct {
    first_name: String,
    last_name: String,
}
#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style!(
        r#"
            h1 {
                color: orange;    
            }

            p {
                color: red;
            }
        "#
    )
    .unwrap();
    let name: &str = "Pedro";
    let my_struct: MyStruct = MyStruct {
        first_name: name.to_owned(),
        last_name: "Bittencourt".to_string(),
    };

    log!(name);
    log!(serde_json::to_string_pretty(&my_struct).unwrap());

    let message: Option<&str> = Some("Mensagem");

    let to_do_list = vec![
        "ir no mercado",
        "comprar cerveja",
        "pedir um X",
        "assistir aula",
    ];

    html! {
        <>
            <h1 class={stylesheet}>{"Hello World!"}</h1>

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"sem mensagem"}</p>
            }

            <ul class={css!("color: yellow;")}>
            // {to_do_list.iter().map(|item| html!{<li>{item}</li>}).collect::<Html>()}
                {to_li_html(to_do_list)}
            </ul>

        </>
    }
}

fn to_li_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
