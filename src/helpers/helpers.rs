use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::{KeyboardEvent, UseStateHandle};

use crate::types::types::Todo;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn message(content: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window.appWindow"])]
    async fn close() -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[derive(Serialize, Deserialize)]
struct MessageArgs<'a> {
    title: &'a str,
    #[serde(rename = "type")]
    kind: &'a str,
}

#[derive(Serialize, Deserialize)]
struct TodoArgs<'a> {
    key: &'a str,
}

pub fn onclose(e: KeyboardEvent) {
    if e.ctrl_key() && e.key() == "q".to_string() {
        spawn_local(async {
            close().await;
        })
    }
}

pub fn show_message(content: &str, kind: &str) {
    let content = content.to_string();
    let kind = kind.to_string();
    spawn_local(async move {
        message(
            &content,
            to_value(&MessageArgs {
                title: "TodoApp Tauri + Yew",
                kind: &kind,
            })
            .unwrap(),
        )
        .await;
    });
}

pub fn display_data(data: UseStateHandle<Option<Todo>>) {
    // Si "invoke" se define con 2 parámetros pero nuestra función asíncrona solo
    // toma 1, será necesario poner como segundo parámetro «JsValue::undefined()»
    spawn_local(async move {
        let new_data = invoke("show_data", JsValue::undefined()).await;
        match new_data {
            Ok(result_ok) => {
                let res = result_ok.clone();
                let res2 = result_ok.clone();
                log(format!("{:?}", from_value::<Todo>(res).unwrap()).as_str());
                data.set(Some(from_value::<Todo>(res2).unwrap()));
            }
            Err(result_err) => {
                log(&result_err.as_string().unwrap());
                show_message(&result_err.as_string().unwrap(), "error");
            }
        }
    });
}

pub fn create_todo(data: UseStateHandle<Option<Todo>>, name: String) {
    spawn_local(async move {
        let resp = invoke("add_todo", to_value(&TodoArgs { key: &*name }).unwrap()).await;
        match resp {
            Ok(_) => data.set(None),
            Err(result_err) => show_message(&result_err.as_string().unwrap(), "error"),
        }
    });
}

pub fn update_todo(data: UseStateHandle<Option<Todo>>, name: String) {
    spawn_local(async move {
        let resp = invoke("update_todo", to_value(&TodoArgs { key: &*name }).unwrap()).await;
        match resp {
            Ok(_) => data.set(None),
            Err(result_err) => show_message(&result_err.as_string().unwrap(), "error"),
        }
    });
}

pub fn remove_todo(data: UseStateHandle<Option<Todo>>, name: String) {
    spawn_local(async move {
        let resp = invoke("remove_todo", to_value(&TodoArgs { key: &*name }).unwrap()).await;
        match resp {
            Ok(_) => data.set(None),
            Err(result_err) => show_message(&result_err.as_string().unwrap(), "error"),
        }
    });
}

pub fn convert_data(result: Todo) -> Vec<(String, (bool, String, u64))> {
    let mut mut_vec = result
        .map
        .into_iter()
        .map(|item| item)
        .collect::<Vec<(String, (bool, String, u64))>>();

    mut_vec.sort_by(|a, b| (b.1).2.cmp(&(a.1).2));

    mut_vec
}
