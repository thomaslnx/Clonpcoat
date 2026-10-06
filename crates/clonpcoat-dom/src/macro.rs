#[macro_export]
macro_rules! html {
    ($($tt:tt)*) => {{
        ::clonpcoat::view! { $($tt)* }
    }}
}
