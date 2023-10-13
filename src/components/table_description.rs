use crate::state::StyewardState;
use yew::functional::use_context;
use yew::prelude::{
    function_component, html, use_effect_with_deps, use_state, Html, UseStateHandle,
};
use yew::virtual_dom::VNode;
#[derive(PartialEq, Clone)]
pub struct PreviousSelection {
    pub previous_schema: Option<String>,
    pub previous_table: Option<String>,
}

#[function_component()]
pub fn TableDescription() -> Html {
    let current_state = use_context::<UseStateHandle<StyewardState>>();
    let previous_state = use_state(|| PreviousSelection {
        previous_schema: None,
        previous_table: None,
    });
    let _previous_state = previous_state.clone();
    let current_schema = match current_state.clone() {
        Some(handle) => handle.current_schema.clone(),
        None => None,
    };
    let current_table = match current_state.clone() {
        Some(handle) => handle.current_table.clone(),
        None => None,
    };
    let _current_schema = current_schema.clone();
    let _current_table = current_table.clone();
    log::info!("1{:?}{:?}", _current_schema, _current_table);
    use_effect_with_deps(
        move |_| {
            let previous_state = previous_state.clone();
            if current_schema.clone().is_some() && current_table.clone().is_some() {
                previous_state.set(PreviousSelection {
                    previous_schema: current_schema.clone(),
                    previous_table: current_table.clone(),
                });
            };
            || ()
        },
        current_state.clone(),
    );
    if _current_schema.is_some() && _current_table.is_some() {
        let text = format!(
            "Description for table {}.{} is currently not available",
            _current_schema.unwrap(),
            _current_table.unwrap()
        );
        let v_node = style_the_text(text);
        style_current_selection_card(v_node)
    } else if _current_schema.is_some() && _current_table.is_none() {
        if _previous_state.previous_schema.clone().is_some()
            && _previous_state.previous_table.clone().is_some()
        {
            let text = format!(
                "Description for table {}.{} is currently not available",
                _previous_state.previous_schema.clone().unwrap(),
                _previous_state.previous_table.clone().unwrap()
            );
            let v_node = style_the_text(text);
            style_current_selection_card(v_node)
        } else {
            let text = "No table is selected".to_string();
            let v_node = style_the_text(text);
            style_current_selection_card(v_node)
        }
    } else if _current_schema.is_none() && _current_table.is_some() {
        html! {
            <div>{"ERROR: You cant select a table without selecting a schema"}</div>
        }
    } else {
        let text = "No table is selected".to_string();
        let v_node = style_the_text(text);
        style_current_selection_card(v_node)
    }
}

fn style_current_selection_card(node: VNode) -> Html {
    html! {
        <div class="h-auto d-inline-block">
            <div class="card" style="max_width; margin-top:10px; font-size:12px">
                <div class="card-header" style="text-align: center;">
                        {"Table Description"}
                </div>
                <div class="card-body">
                    {node}
                </div>
            </div>
        </div>
    }
}

fn style_the_text(text: String) -> Html {
    html! {
        <div class="card-text" style="font-family: courier, monospace;  white-space: nowrap;">{text}</div>
    }
}
