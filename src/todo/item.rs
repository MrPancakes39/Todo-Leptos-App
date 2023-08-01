use leptos::{ev::MouseEvent, *};

#[component]
pub fn Todo<F>(cx: Scope, todo: String, remove: F) -> impl IntoView
where
    F: FnMut(MouseEvent) -> () + 'static,
{
    view! {cx,
        <div class="todo-item">
            <p class="text">{todo}</p>
            <button type="button" on:click=remove>"x"</button>
        </div>
    }
}
