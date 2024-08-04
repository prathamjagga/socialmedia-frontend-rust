use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(LoginForm)]
fn login_form() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let message = message.clone();
        Callback::from(move |_| {
            if *username == "dev.to" && *password == "dev.to" {
                message.set("Login successful!".to_string());
            } else {
                message.set("Invalid username or password.".to_string());
            }
        })
    };

    html! {
        <div>
            <h1>{ "Login" }</h1>
            <input
                type="text"
                placeholder="Username"
                value={(*username).clone()}
                oninput={on_username_input}
            />
            <input
                type="password"
                placeholder="Password"
                value={(*password).clone()}
                oninput={on_password_input}
            />
            <button onclick={on_submit}>{ "Login" }</button>
            <p>{ (*message).clone() }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <LoginForm />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
