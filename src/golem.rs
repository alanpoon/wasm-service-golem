pub mod todos {
    use crate::{site::*, Handler, Request};
    use html_to_string_macro::html;
    use serde::Deserialize;
    use std::sync::{Mutex, RwLock};

    pub(crate) fn register(
        router: &mut matchit::Router<Handler>,
    ) -> Result<(), matchit::InsertError> {
        const TODOS: &str = "Todos";
        router.insert("/todos", |_, _| page(TODOS, component()))?;
        router.insert("/golem/todos;nav", |_, r| r.body.clone())?;
        router.insert("/golem/todos;add", |_, r| r.body.clone())?;
        router.insert("/golem/todos/:id", |_, r| r.body.clone())?;
        router.insert("/golem/todos/:id/toggle", |_, r| r.body.clone())?;
        router.insert("/golem/todos;toggleall", |_, r| r.body.clone())?;
        router.insert("/golem/todos;filter=:filter", |_, r| r.body.clone())?;
        
        Ok(())
    }
    fn component() -> String {
        html! {
            { tabs(Tabs::Todos) }
            <section class="todos">
                <header>
                    <h1>"todos"</h1>
                    <div id="controls">
                    { input_frag(false) }
                    <div hx-post="./golem/todosAdd" hx-ext="json-enc" hx-vars="title:document.getElementById('todo-new').value,priority:['medium']" hx-target=".todos ul">"Add"</div>
                    </div>
                </header>
                { items_frag(false) }
                <footer>
                </footer>
                <style>r#"
            .todos {
                font-size: 1.5em;
            }
            .todos input {
                display: inline-block;
                vertical-align: middle;
            }
            header input[type=checkbox] {
                font-size: 1.5em;
            }
            .todos ul {
                list-style-type: none;
                padding-left: 0;
            }
            .todos li {
                position: relative;
                height: 2em;
                width: 20em;
                overflow: hidden;
                padding-top: .2em;
            }
            .todos li>* {
                vertical-align: middle;
            }
            .todos li>input[type=checkbox]:checked + label {
                text-decoration: line-through;
                color: var(--text-muted);
            }
            .todos button.delete:after {
                content: '×';
                font-size: 2em;
                position: relative;
                top: -.2em;
            }
            .todos button.delete {
                position: absolute;
                margin-top: -.2em;
                right: 0;
                padding: 0;
                color: #af5b5e;
                background-color: var(--background-body);
            }
            .todos button.delete:hover {
                color: #ac262b;
            }

            "#
                </style>
            </section>
        }
    }

    fn items_frag(oob: bool) -> String {
      
        html! {
            <ul id="items-list" { if oob { r#"hx-swap-oob="true""# } else { "" } }>
            </ul>
        }
    }

    fn input_frag(oob: bool) -> String {
        // html! {
        //     <input id="todo-new" name="todo-new" placeholder="What needs to be done?" autofocus
        //         hx-post="./golem/todos;add" hx-target=".todos ul" hx-swap="afterbegin" { if oob { r#"hx-swap-oob="true""# } else { "" } } />
        // }
        html! {
            <input id="todo-new" name="todo-new" placeholder="What needs to be done?" autofocus
             hx-target=".todos ul" hx-swap="afterbegin" { if oob { r#"hx-swap-oob="true""# } else { "" } } />
        }
    }
}
