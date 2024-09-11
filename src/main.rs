use serde::{Deserialize, Serialize};
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

// Define the routes
#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/posts")]
    Hello,
    #[at("/world")]
    World,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Define the Post structure
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Post {
    username: String,
    content: String,
}

#[function_component(Hello)]
fn hello() -> Html {
    // Use state to manage the content of the post
    let content = use_state(|| String::new());

    // Handler for content input
    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    // Handler for the submit button
    let on_submit = {
        let content = content.clone();
        Callback::from(move |_| {
            // Get the username from localStorage
            if let Some(storage) = window().unwrap().local_storage().unwrap() {
                if let Ok(Some(username)) = storage.get_item("username") {
                    // Get the current posts from localStorage
                    let mut posts: Vec<Post> =
                        if let Ok(Some(posts_str)) = storage.get_item("posts") {
                            serde_json::from_str(&posts_str).unwrap_or_else(|_| vec![])
                        } else {
                            vec![]
                        };

                    // Add the new post
                    let new_post = Post {
                        username: username.clone(),
                        content: (*content).clone(),
                    };
                    posts.push(new_post);

                    // Save the updated posts array back to localStorage
                    let posts_str = serde_json::to_string(&posts).unwrap();
                    storage.set_item("posts", &posts_str).unwrap();

                    web_sys::console::log_1(&"Post added!".into());
                }
            }
        })
    };

    html! {
        <div>
            <h1>{ "Hello" }</h1>
            <div>
                <label for="content">{ "Content: " }</label>
                <input id="content" type="text" value={(*content).clone()} oninput={on_content_input} />
            </div>
            <button onclick={on_submit}>{ "Submit" }</button>
        </div>
    }
}

#[function_component(World)]
fn world() -> Html {
    // Fetch posts from localStorage
    let posts: Vec<Post> = {
        if let Some(storage) = window().unwrap().local_storage().unwrap() {
            if let Ok(Some(posts_str)) = storage.get_item("posts") {
                serde_json::from_str(&posts_str).unwrap_or_else(|_| vec![]) // Deserialize posts from JSON
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    };

    html! {
        <div>
            <h1>{ "World" }</h1>
            <h2>{ "User Posts" }</h2>
            <ul>
                { for posts.iter().map(|post| html! {
                    <li>
                        <strong>{ format!("{}: ", post.username) }</strong>
                        <span>{ &post.content }</span>
                    </li>
                })}
            </ul>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());

    // Handler for username input
    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    // Handler for password input
    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    // Submit handler that saves the username to local storage
    let on_submit = {
        let username = username.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); // Prevent form submission
            if let Some(storage) = window().unwrap().local_storage().unwrap() {
                storage.set_item("username", &username).unwrap();
                web_sys::console::log_1(&"Username saved to local storage".into());
            }
        })
    };

    html! {
        <div>
            <h1>{ "Home" }</h1>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Hello}>{ "Add a new post" }</Link<Route>></li>
                    <li><Link<Route> to={Route::World}>{ "Posts" }</Link<Route>></li>
                </ul>
            </nav>

            <form>
                <div>
                    <label for="username">{ "Username: " }</label>
                    <input id="username" type="text" value={(*username).clone()} oninput={on_username_input} />
                </div>
                <div>
                    <label for="password">{ "Password: " }</label>
                    <input id="password" type="password" value={(*password).clone()} oninput={on_password_input} />
                </div>
                <button type="submit" onclick={on_submit}>{ "Submit" }</button>
            </form>
        </div>
    }
}
// Root component with router
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={|routes: Route| switch(&routes)} />
        </BrowserRouter>
    }
}

// Route switching logic
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Hello => html! { <Hello /> },
        Route::World => html! { <World /> },
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <h1>{ "404 - Page Not Found" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
