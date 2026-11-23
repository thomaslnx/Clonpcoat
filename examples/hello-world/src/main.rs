fn main() {
    let dynamic = if 5 > 6 {
        clonpcoat::view! { "hi" }
    } else {
        clonpcoat::view! { "bye" }
    };

    let content = clonpcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                (dynamic)
                " "
                b class="cool" { "carl & friends"}
            }
        }
    };
    println!("{}", content);
}
