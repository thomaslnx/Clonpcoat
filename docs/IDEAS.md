```rust
use clonpcoat::{component, html, Router};

async fn use_auth() -> Result<Option<User>, clonpcoat::Error> {
    match use_session() {
        Some(session) => User::first_by_session(session.id).exec(&mut use_db()).await?,
        None => Ok(None),
    }
}

async fn require_auth() -> Result<User, clonpcoat::Error> {
    let user = use_auth().await?,

    match user {
        Some(user) => user,
        None => clonpcoat::redirect(sign_in),
    }
}

#[component]
fn my_button(button_attrs: clonpcoat::dom::button::Attrs) {
    html! {
        button {..button_attrs} class={button_props.class + " button"}
    }
}

#[component]
async fn events() -> Result<Html, anyhow::Error> {
    let user = require_auth().await?;

    let db = use_db();
    let events = Event::get_by_user_id().exec(&mut db).await?;

    html! {
        div class="flex flex-col" {
            for event in events {
                div class="card" {
                    h3 { (event.title) }

                    if event.admin_user_id == user.id {
                        (my_button) { "Edit" }
                    }
                }
            }
        }
    }
}

#[layout]
async fn nav(children: Html) -> Html {
    html! {
        html {
            head {
                title { "Clonpcoat demo app" }

                (clonpcoat::script::htmx)
                (clonpcoat::script::alpinejs)
            }
            body {
                nav {
                    // Type-safe href somehow based on the components?

                    a href={events} { "Events" }
                    a href="/submissions" { "Submissions" }

                    if use_signed_in().await {
                        // Inline API handlers?
                        (my_button) onclick={async || {
                            delete_session().await;
                            return clonpcoat::redirect(sign_in);
                        }} {
                            "Sign out"
                        }
                    }
                }
                main {
                    (children)
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .layout(nav)
        .get("/events", events);

    // or
    let router = Router::file();

    clonpcoat::serve(router).await;
}
```
