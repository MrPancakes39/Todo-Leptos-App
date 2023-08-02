use super::item::Todo;
use leptos::{ev::SubmitEvent, html::Input, *};
// use log::info;

#[derive(Debug, Clone)]
struct TodoItem {
    text: String,
    key: u32,
}

#[component]
pub fn TodoContainer(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Vec::<TodoItem>::new());
    let (warning, set_warning) = create_signal(cx, Some(""));
    let input_ref: NodeRef<Input> = create_node_ref(cx);

    let add_to_list = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = input_ref.get().expect("<input> to exist");
        let value = input.value();
        if value.is_empty() {
            set_warning.set(Some("Can't add empty item."));
        } else {
            set_warning.set(None);
            let new_key = todos
                .with(|v| v.last().map(|todo| todo.key + 1))
                .unwrap_or(0);
            set_todos.update(|v| {
                v.push(TodoItem {
                    text: value,
                    key: new_key,
                })
            });
            input.set_value("");
            // info!("{:#?}", todos.get());
        }
    };

    view! {cx,
        <form class="todo-container" on:submit=add_to_list>
            <h4 class="title">"Cool Todo App"</h4>
            <div class="form-group">
                <input placeholder="Add a todo item" id="todo_input" node_ref=input_ref />
                <button type="submit">"+"</button>
            </div>
            {move|| warning.get().map(|msg| view! {cx, <div>{msg}</div>})}
            <div class="todos">
                <For each=move||todos.get() key=|todo|todo.key view=move|cx, todo|{
                    view! {cx, <Todo todo={todo.text}
                                     remove=move|_|set_todos.update(|v| v.retain(|td| td.key != todo.key))
                                />
                    }
                } />
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
