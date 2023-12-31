use leptos::*;

#[component]
pub fn Styles(cx: Scope) -> impl IntoView {
    view! {cx,
    <style>r#"
        @import url("https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap");

        :root {
            --torquise: #64dac4;
            --light-blue: #4678d8;
            --purple: #8548da;
        }

        html,
        body {
            margin: 0;
            width: 100%;
            height: 100%;
        }

        body {
            display: grid;
            place-items: center;
            font-family: "Roboto", sans-serif;
        }

        .background {
            z-index: -1;
            position: fixed;
            inset: 0;
            background: linear-gradient(180deg, var(--torquise) 0%, var(--light-blue) 100%);
        }

        .todo-container {
            --btn-size: 4rem;
            width: 600px;
            height: 400px;
            padding: 2rem;
            background: #fff;
            border-radius: 20px;
            display: flex;
            flex-direction: column;
            align-items: center;
        }

        .todo-container .title {
            font-weight: bold;
        }

        .todo-container :is(.form-group, .todo-item) {
            width: 100%;
            display: flex;
            gap: 1rem;
            margin-bottom: 1rem;
        }

        .form-group input {
            margin: 0;
            flex-grow: 1;
            border: 2px solid #e4e4e4;
            border-radius: 5px;
            padding-inline: 1rem;
            outline: transparent;
        }

        .form-group input:focus {
            border-color: var(--light-blue);
        }

        .form-group button {
            margin: 0;
            padding: 0;
            width: var(--btn-size);
            height: var(--btn-size);
            color: #fff;
            font-size: 3rem;
            font-weight: bold;
            text-align: center;
            border: 0;
            background-color: var(--purple);
        }

        .form-group button:is(:hover, :focus) {
            background-color: hsl(265, 66%, 52%);
        }

        .todo-container .todos {
            flex-grow: 1;
            margin-bottom: 1rem;
            overflow: auto;
            width: 100%;
        }

        .form-group p {
            margin: 0;
            flex-grow: 1;
        }

        .form-group #clear {
            font-size: 1rem;
            width: fit-content;
            height: auto;
            padding-inline: 1rem;
        }

        .todo-item {
            background-color: #e4e4e4;
            border-radius: 5px;
            margin-block-end: 1rem;
            align-items: center;
        }

        .todo-item .text {
            margin: 0;
            text-align: start;
            padding-inline: 1rem;
            flex-grow: 1;
        }

        .todo-item button {
            margin: 0;
            padding: 0;
            border: 0;
            background-color: #d94a3a;
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
            color: #fff;
            font-weight: bold;
            font-size: 1.75rem;
            width: var(--btn-size);
        }

        .todo-item button:is(:hover, :focus) {
            background-color: hsl(6, 68%, 50%);
        }"#
    </style>
        }
}
