use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    let todo_state = use_state(|| ("".to_string(), "".to_string(), false, None as Option<i32>));
    let message = use_state(|| "".to_string());
    let todos = use_state(Vec::new);

    let get_todos = {
        let todos = todos.clone();
        let message = message.clone();
        Callback::from(move |_| {
            let todos = todos.clone();
            let message = message.clone();
            spawn_local(async move {
                match Request::get("http://127.0.0.1:8000/api/todos").send().await {
                    Ok(resp) if resp.ok() => {
                        let fetched_todos: Vec<Todo> = resp.json().await.unwrap_or_default();
                        todos.set(fetched_todos);
                    }

                    _ => message.set("Failed to fetch todos".into()),
                }
            });
        })
    };

    let create_todo = {
        let todo_state = todo_state.clone();
        let message = message.clone();
        let get_todos = get_todos.clone();
        Callback::from(move |_| {
            let (title, description, completed, _) = (*todo_state).clone();
            let todo_state = todo_state.clone();
            let message = message.clone();
            let get_todos = get_todos.clone();

            spawn_local(async move {
                let todo_data = serde_json::json!({ "title": title, "description": description, "completed": completed });

                let response = Request::post("http://127.0.0.1:8000/api/todos")
                    .header("Content-Type", "application/json")
                    .body(todo_data.to_string())
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        message.set("Todo created successfully".into());
                        get_todos.emit(());
                    }

                    _ => message.set("Failed to create todo".into()),
                }

                todo_state.set(("".to_string(), "".to_string(), false, None));
            });
        })
    };

    let update_todo = {
        let todo_state = todo_state.clone();
        let message = message.clone();
        let get_todos = get_todos.clone();

        Callback::from(move |_| {
            let (title, description, completed, editing_todo_id) = (*todo_state).clone();
            let todo_state = todo_state.clone();
            let message = message.clone();
            let get_todos = get_todos.clone();

            if let Some(id) = editing_todo_id {
                spawn_local(async move {
                    let todo_payload = serde_json::json!({ "title": title, "description": description, "completed": completed });

                    let response = Request::put(&format!("http://127.0.0.1:8000/api/todos/{}", id))
                        .header("Content-Type", "application/json")
                        .body(todo_payload.to_string())
                        .send()
                        .await;

                    match response {
                        Ok(resp) if resp.ok() => {
                            message.set("Todo updated successfully".into());
                            get_todos.emit(());
                        }

                        _ => message.set("Failed to update todo".into()),
                    }

                    todo_state.set(("".to_string(), "".to_string(), false, None));
                });
            }
        })
    };

    let delete_todo = {
        let message = message.clone();
        let get_todos = get_todos.clone();

        Callback::from(move |id: i32| {
            let message = message.clone();
            let get_todos = get_todos.clone();

            spawn_local(async move {
                let response = Request::delete(&format!("http://127.0.0.1:8000/api/todos/{}", id)).send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                        message.set("Todo deleted successfully".into());
                        get_todos.emit(());
                    }

                    _ => message.set("Failed to delete todo".into()),
                }
            });
        })
    };

    let edit_todo = {
        let todo_state = todo_state.clone();
        let todos = todos.clone();

        Callback::from(move |id: i32| {
            if let Some(todo) = (*todos).iter().find(|t| t.id == id) {
                todo_state.set((todo.title.clone(), todo.description.clone(), todo.completed, Some(id)));
            }
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-4xl font-bold text-green-500 mb-4">{ "Todo List Application" }</h1>
                <div class="mb-4">
                    <input
                        placeholder="Title"
                        value={todo_state.0.clone()}
                        oninput={Callback::from({
                            let todo_state = todo_state.clone();
                            move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                todo_state.set((input.value(), todo_state.1.clone(), todo_state.2, todo_state.3));
                            }
                        })}
                        class="border rounded px-4 py-2 mr-2"
                    />
                    <input
                        placeholder="Description"
                        value={todo_state.1.clone()}
                        oninput={Callback::from({
                            let todo_state = todo_state.clone();
                            move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                todo_state.set((todo_state.0.clone(), input.value(), todo_state.2, todo_state.3));
                            }
                        })}
                        class="border rounded px-4 py-2 mr-2"
                    />
                    <label class="inline-flex items-center mr-2">
                        <input
                            type="checkbox"
                            checked={todo_state.2}
                            onclick={Callback::from({
                                let todo_state = todo_state.clone();
                                move |_| {
                                    todo_state.set((
                                        todo_state.0.clone(),
                                        todo_state.1.clone(),
                                        !todo_state.2,
                                        todo_state.3,
                                    ));
                                }
                            })}
                            class="form-checkbox"
                        />
                        <span class="ml-2">{"Completed"}</span>
                    </label>

                    <button
                        onclick={if todo_state.3.is_some() { update_todo.clone() } else { create_todo.clone() }}
                        class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                    >
                        { if todo_state.3.is_some() { "Update Todo" } else { "Create Todo" } }
                    </button>
                        if !message.is_empty() {
                        <p class="text-green-500 mt-2">{ &*message }</p>
                    }
                </div>

                <button
                    onclick={get_todos.reform(|_| ())}  
                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4"
                >
                    { "Fetch Todo List" }
                </button>

                <h2 class="text-2xl font-bold text-gray-700 mb-2">{ "Todo List" }</h2>

                <ul class="list-disc pl-5">
                    { for (*todos).iter().map(|todo| {
                        let todo_id = todo.id;
                        html! {
                            <li class="mb-2">
                                <span class="font-semibold">
                                    { format!("ID: {}, Title: {}, Description: {}, Completed: {}", todo.id, todo.title, todo.description, todo.completed) }
                                </span>
                                <button
                                    onclick={delete_todo.clone().reform(move |_| todo_id)}
                                    class="ml-4 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded"
                                >
                                    { "Delete" }
                                </button>
                                <button
                                    onclick={edit_todo.clone().reform(move |_| todo_id)}
                                    class="ml-4 bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-1 px-2 rounded"
                                >
                                    { "Edit" }
                                </button>
                            </li>
                        }
                    })}
                </ul>
                    
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
