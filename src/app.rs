use fake::{Fake, faker::{internet::en::SafeEmail, phone_number::fr_fr::PhoneNumber}};
use leptos::{leptos_dom::ev::SubmitEvent, *};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

struct UserInfo {
    name: String,
    email: String,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let (user_name, set_user_name) = create_signal(cx, String::new());
    let (user_email, set_user_email) = create_signal(cx, String::new());
    let (user_phone, set_user_phone) = create_signal(cx, String::new());

    let generate_random = move |_| {
        let generated_name = Name(EN).fake::<String>();
        let generated_email = SafeEmail().fake::<String>();
        let generated_phone = PhoneNumber().fake::<String>();
        set_user_name.set(generated_name);
        set_user_email.set(generated_email);
        set_user_phone.set(generated_phone);
    };

    view! { cx,
        <main class="container">
            <div>
                <p>{ move || user_name.get() }</p>
                <p>{ move || user_email.get() }</p>
                <p>{ move || user_phone.get() }</p>
            </div>
            <button on:click=generate_random>"Generate"</button>
        </main>
    }
}
