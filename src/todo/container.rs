use leptos::*;

#[derive(Debug, Clone)]
struct TodoItem {
    text: String,
    key: u32,
}

pub fn TodoContainer(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Vec::<TodoItem>::new());

    view! {cx,
        <form class="todo-container">
            <h4 class="title">"Cool Todo App"</h4>
            <div class="form-group">
                <input placeholder="Add a todo item" id="todo_input" />
                <button type="button">"+"</button>
            </div>
            <div class="todos">
                <For each=move||todos.get() key=|todo|todo.key view=move|cx, todo|{} />
            </div>
            <div class="form-group">
                <p>"You have "{move||todos.get().len()}" pending tasks"</p>
                <button type="button" id="clear" on:click=move|_|set_todos.update(|v| v.clear())>
                    "Clear All"
                </button>
            </div>
        </form>
    }
}
