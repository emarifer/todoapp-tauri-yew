use chrono::{Datelike, Utc};
use yew::{
    function_component, html, use_effect_with_deps, use_state, Callback, Html, KeyboardEvent,
    UseStateHandle,
};

use crate::components::{header_section::HeaderSection, item_component::ItemComponent};
use crate::helpers::helpers::{convert_data, display_data, onclose, remove_todo, update_todo};
use crate::types::types::Todo;

#[function_component(App)]
pub fn app() -> Html {
    let data: UseStateHandle<Option<Todo>> = use_state(|| None);

    {
        let data = data.clone();

        let data2 = data.clone();

        use_effect_with_deps(
            move |_| {
                display_data(data);

                || {}
            },
            data2,
        );
    }

    let on_close = Callback::from(|e: KeyboardEvent| onclose(e));

    let complete_todo = {
        let data = data.clone();
        Callback::from(move |name: String| {
            let data = data.clone();
            update_todo(data, name);
        })
    };

    let delete_todo = {
        let data = data.clone();
        Callback::from(move |name: String| {
            let data = data.clone();
            remove_todo(data, name);
        })
    };

    let data_show_section = match (*data).clone() {
        Some(result) => html! {
            <ul>
                {
                    convert_data(result).iter().map(|item| {
                        let item = item.clone();
                        let complete_todo = complete_todo.clone();
                        let delete_todo = delete_todo.clone();
                        let id = item.1.2;
                        html!(<ItemComponent item={item} oncomplete={complete_todo} ondelete={delete_todo} key={id} />)
                    }).collect::<Html>()
                }
            </ul>
        },
        None => html! {},
    };

    html! {
        <div onkeyup={on_close} tabindex={0} class="flex flex-col justify-center text-center">
            <header>
                <HeaderSection data={data} />
            </header>

            <main class="mt-56">
                {data_show_section}
            </main>

            <footer class="mt-3 mb-6">
                <a
                  class="italic tracking-wider hover:text-sky-500 ease-in duration-300"
                  href="https://github.com/emarifer/todoapp-tauri-yew"
                  target="_blank"
                >
                  {format!("MIT Licensed | Copyright © {} Enrique Marín", Utc::now().year())}
                </a>
            </footer>
        </div>
    }
}

/*
 * Vim regex backreference. VER:
 * https://stackoverflow.com/questions/3339080/vim-regex-backreference
 * Esta es la regex usada:
 * %s/\v(ICON)_([^:};,]*)/\2_\1/g
 *
 * Regular expression to select all characters except letters or digits. VER:
 * https://stackoverflow.com/questions/44771364/regular-expression-to-select-all-characters-except-letters-or-digits
 *
 * Keyed lists. VER:
 * https://yew.rs/docs/concepts/html/lists#keyed-lists
 */
