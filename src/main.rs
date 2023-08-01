mod style;
mod todo;

use leptos::*;
use style::Styles;
use todo::TodoContainer;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <Styles/>
        <div class="app">
            <div class="background"></div>
            <TodoContainer />
        </div>
    }
}

fn main() {
    // console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    mount_to_body(|cx| view! { cx,  <App/> })
}
