fn main() {
    let rendered = clonpcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                "hi"
                b class="cool" { "carl & friends" }
            }
        }
    };
    println!("{}", rendered);
}
