use clonpcoat::view::View;

fn main() {
    let rendered = clonpcoat::dom::render(&clonpcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                "hi"
                b class="cool" { "carl" }
            }
        }
    });
    println!("{}", rendered);
}
