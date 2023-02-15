use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect, use_node_ref, Callback, Html, MouseEvent, Properties,
    UseStateHandle,
};

use crate::helpers::helpers::{create_todo, show_message};
use crate::icon_paths::icon_paths::ADD_ICON;
use crate::types::types::Todo;

#[derive(Properties, PartialEq)]
pub struct HeaderSectionProps {
    pub data: UseStateHandle<Option<Todo>>,
}

#[function_component(HeaderSection)]
pub fn header_section(props: &HeaderSectionProps) -> Html {
    let input_ref = use_node_ref();

    let on_pressed_add = {
        let data = props.data.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |_e: MouseEvent| {
            let data = data.clone();
            let name = input_ref.cast::<HtmlInputElement>().unwrap().value();

            if name.is_empty() {
                show_message("This field can not be blank", "warning");
                return;
            }

            create_todo(data, name);

            // Limpamos el input
            input_ref.cast::<HtmlInputElement>().unwrap().set_value("");
        })
    };

    {
        let input_ref = input_ref.clone();
        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
        })
    }

    html! {
              <nav
                class="fixed top-0 left-0 right-0 z-10 flex flex-col justify-center bg-primary pb-2"
              >
                <a
                  href="https://github.com/emarifer?tab=repositories"
                  class="m-auto"
                  target="_blank"
                >
                  <img
                    src="img/app-logo.svg"
                    class="h-40 p-6 will-change-[filter] logo"
                    alt="App Logo"
                  />
                </a>

                <div class="flex justify-center gap-3">
                  <input
                    ref={input_ref}
                    type="text"
                    class="p-2 rounded-lg text-slate-600 focus:outline-none focus:ring focus:ring-blue-400"
                    placeholder="Enter a Todo…"
                  />
                  <button
                    type="button"
                    onclick={on_pressed_add}
                    class="rounded-lg border border-transparent px-3 py-1 bg-sky-600 hover:bg-sky-500"
                    title="Add"
                  >
                    <svg width="18" height="18" viewBox="0 0 18 18">
                      <g transform="translate(-1.25, -1.25) scale(0.4)" stroke="white">
                        <path stroke-width="3" d={ADD_ICON} />
                      </g>
                    </svg>
                  </button>
                </div>

                <h1 class="mt-4 text-lg font-bold">{"My To Do List:"}</h1>
              </nav>

    }
}
