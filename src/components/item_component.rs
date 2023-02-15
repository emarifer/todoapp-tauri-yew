use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use crate::icon_paths::icon_paths::{DELETE_ICON, UPDATE_ICON};

#[derive(Properties, PartialEq)]
pub struct ItemComponentProps {
    pub item: (String, (bool, String, u64)),
    pub oncomplete: Callback<String>,
    pub ondelete: Callback<String>,
}

#[function_component(ItemComponent)]
pub fn item_component(props: &ItemComponentProps) -> Html {
    let item_style = "list-none flex justify-end items-center rounded-full m-[calc(1vw+1.2vmax)] p-[calc(0.5vw+0.75vmax)] \
                            hover:-translate-y-1.5 ease-in duration-300 shadow-md border-2 border-slate-400 lg:w-3/4 lg:mx-auto";

    let key_title = props.item.0.split("___").next().unwrap();

    let on_pressed_complete = {
        let oncomplete = props.oncomplete.clone();
        let item = props.item.clone();

        Callback::from(move |_e: MouseEvent| {
            let name = item.0.clone();
            oncomplete.emit(name);
        })
    };

    let on_pressed_delete = {
        let ondelete = props.ondelete.clone();
        let item = props.item.clone();

        Callback::from(move |_e: MouseEvent| {
            let name = item.0.clone();
            ondelete.emit(name);
        })
    };

    html! {
        <li class={classes!(if props.item.1.0 {"bg-lime-600 hover:shadow-green-700"} else {"bg-rose-500 hover:shadow-red-700"}, item_style)}>
            <b class="min-w-[25vw] max-w-[35vw] text-right whitespace-nowrap overflow-hidden text-ellipsis" title={key_title.to_owned()}>
                { key_title }
            </b>

            <pre>{": 🕒 "}</pre>

            { format!("{}", props.item.1.1) }

            <button onclick={on_pressed_complete} class="bg-yellow-400 hover:bg-slate-300 ml-2 p-2 rounded-full" title="Complete">
                <svg width="18" height="18" viewBox="0 0 18 18">
                    <g transform="translate(-1.23, -1.23) scale(0.4)" stroke="darkviolet">
                        <path stroke-width="3" d={UPDATE_ICON} />
                    </g>
                </svg>
            </button>

            <button onclick={on_pressed_delete} class="bg-blue-600 hover:bg-blue-500 ml-2 p-2 rounded-full" title="Delete">
                <svg width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                    <path d={DELETE_ICON} />
                </svg>
            </button>
        </li>
    }
}
