use ray_tracer_loader::parse_config;
use serde_yaml::Value;
use web_sys::{console, HtmlTextAreaElement};
use yew::prelude::*;
use yew::{function_component, html, use_state, Callback};

#[function_component(App)]
pub fn header_input() -> Html {
    let base64 = use_state(|| "".to_owned());

    let onkeypress = {
        let base64 = base64.clone();
        Callback::from(move |event: KeyboardEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            let yaml = input.value();

            let config: Result<Value, serde_yaml::Error> = serde_yaml::from_str(&*yaml);

            match config {
                Ok(config) => match parse_config(config) {
                    Ok((camera, world)) => {
                        let canvas = camera.render(world);

                        let img = image::load_from_memory(&canvas.to_ppm().as_bytes()).unwrap();

                        console::log_1(&format!("img: {:#?}", img).into());

                        let mut buf = vec![];
                        img.write_to(&mut buf, image::ImageOutputFormat::Png)
                            .unwrap();
                        let res_base64 = base64::encode(&buf);

                        base64.set(format!("data:image/png;base64,{}", res_base64));
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            };
        })
    };

    html! {
        <div>
            <textarea
                class="yaml-config"
                placeholder="Yaml scene config"
                {onkeypress}
            />
            <img src={ (*base64).clone() } alt="Scene" />
        </div>

    }
}

fn main() {
    yew::start_app::<App>();
}
