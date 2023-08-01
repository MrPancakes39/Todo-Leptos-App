use leptos::*;

#[component]
pub fn Todo(cx: Scope, todo: String) -> impl IntoView {
    view! {cx,
        <div class="todo-item">
            <p class="text">{todo}</p>
            <button type="button">"x"</button>
        </div>
    }
}
