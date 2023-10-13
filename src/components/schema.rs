#[cfg(test)]
mod test_schema;

use crate::state::StyewardState;
use crate::StyewardConfig;

use gloo_net;
use gloo_net::http::{Request, Response};
use serde_json;
use url::Url;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console::error_1, Event, HtmlSelectElement};
use yew::functional::use_context;
use yew::prelude::{
    function_component, html, Callback, Children, Html, Properties, UseStateHandle,
};
use yew::suspense::use_future;
use yew::virtual_dom::{VList, VNode, VTag, VText};

pub enum SchemeSelectionError {
    EndpointError(String),
    GlooError(String),
    HttpStatusError(String),
    DeserializeError(String),
}

#[derive(Properties, PartialEq)]
struct ConfigProp {
    config: StyewardConfig,
}

fn style_the_select(node: VNode) -> Html {
    match node {
        VNode::VTag(v_tag) => {
            let mut tag = v_tag.clone();
            tag.add_attribute("class", "form-select form-select-md sm");
            VNode::VTag(tag)
        }
        input_node => input_node,
    }
}

fn get_error_select(label: String) -> Html {
    let output_node = html! {
        <select disabled={true}>
            <option selected={true} value={label.clone()} disabled={true}>
                {label.clone()}
            </option>
        </select>
    };
    style_the_select(output_node)
}

#[function_component]
pub fn SchemaSelection() -> Html {
    match use_context::<StyewardConfig>() {
        Some(config) => {
            html! {<RemoteSelection config={config}/>}
        }
        None => get_error_select(format!("error!")),
    }
}

#[function_component]
fn RemoteSelection(prop: &ConfigProp) -> Html {
    let config = prop.config.clone();
    let remote_schemas = use_future(|| async move {
        let schema_path = format!("api/v1/schemas");
        get_schemas(&config.scheme, &config.host, config.port, &schema_path)
            .await
            .map(get_schema_options)
    });
    match remote_schemas {
        Ok(result_ref) => match &*result_ref {
            Ok(v_list) => {
                let vnode = VNode::VList(v_list.clone());
                html! {
                    <FilledSelection>
                        {vnode}
                    </FilledSelection>
                }
            }
            Err(error) => {
                match error {
                    SchemeSelectionError::EndpointError(m)
                    | SchemeSelectionError::GlooError(m)
                    | SchemeSelectionError::HttpStatusError(m)
                    | SchemeSelectionError::DeserializeError(m) => error_1(&m.into()),
                };
                get_error_select(format!("error!"))
            }
        },
        Err(_) => get_error_select(format!("loading...")),
    }
}

#[derive(Properties, PartialEq)]
struct PropOptionsList {
    children: Children,
}

#[derive(Properties, PartialEq)]
struct PropSelectCallback {
    cb: Callback<Event>,
    children: Children,
}

#[function_component]
fn FilledSelection(prop: &PropOptionsList) -> Html {
    match use_context::<UseStateHandle<StyewardState>>() {
        Some(handle) => {
            let cb = Callback::from(move |event: Event| {
                let handle = handle.clone();
                update_current_schema(handle, event);
            });
            html! {
                <HandledSelection cb={cb}>
                    <option selected={true} value={"NA"} disabled={true}>
                        {"select schema"}
                    </option>
                    {for prop.children.iter()}
                </HandledSelection>
            }
        }
        None => {
            error_1(&JsValue::from_str(
                "schema select error: state context not provided",
            ));
            get_error_select(format!("ERROR: Contact Support."))
        }
    }
}

#[function_component]
fn HandledSelection(prop: &PropSelectCallback) -> Html {
    let onchange = prop.cb.clone();
    let output_vnode = html! {
        <select {onchange} id={"schema-select"}>
            {for prop.children.iter()}
        </select>
    };
    style_the_select(output_vnode)
}

fn update_current_schema(handle: UseStateHandle<StyewardState>, event: Event) {
    match event.target() {
        Some(target) => match target.dyn_into::<HtmlSelectElement>() {
            Ok(select_elem) => {
                let schema = select_elem.value();
                handle.set(StyewardState {
                    current_schema: Some(schema),
                    current_table: None,
                });
            }
            Err(_) => {
                error_1(&JsValue::from(
                    "schema select error: event target is not the schema selection element.",
                ));
            }
        },
        None => {
            error_1(&JsValue::from("schema select error: target not found."));
        }
    };
}

fn get_schema_options(option_strings: Vec<String>) -> VList {
    let option_nodes: Vec<VNode> = option_strings.iter().map(_get_schema_option).collect();
    let mut v_list = VList::new();
    v_list.add_children(option_nodes);
    v_list
}

fn _get_schema_option(option_text: &String) -> VNode {
    let text_node = VNode::VText(VText::new(String::from(option_text)));
    let mut option_node = VTag::new("option");
    option_node.add_attribute("value", String::from(option_text));
    option_node.add_child(text_node);
    VNode::VTag(Box::new(option_node))
}

async fn get_schemas(
    scheme: &String,
    host: &String,
    port: u16,
    schema_path: &String,
) -> Result<Vec<String>, SchemeSelectionError> {
    let endpoint = format!("{scheme}://{host}:{port}/{schema_path}");
    match Url::parse(&endpoint) {
        Ok(url) => get_schemas_with_gloo(url).await,
        Err(error) => Err(SchemeSelectionError::EndpointError(format!("{error}"))),
    }
}

async fn get_schemas_with_gloo(url: Url) -> Result<Vec<String>, SchemeSelectionError> {
    match Request::get(url.as_str()).send().await {
        Ok(response) => {
            if response.ok() {
                get_schemas_from_response(response).await
            } else {
                match response.text().await {
                    Ok(body_string) => Err(SchemeSelectionError::HttpStatusError(body_string)),
                    Err(_) => Err(SchemeSelectionError::HttpStatusError(format!(
                        "Status: {}",
                        response.status()
                    ))),
                }
            }
        }
        Err(error) => Err(handle_gloo_error(error)),
    }
}

fn handle_gloo_error(gloo_error: gloo_net::Error) -> SchemeSelectionError {
    match gloo_error {
        gloo_net::Error::SerdeError(error) => SchemeSelectionError::GlooError(format!("{error}")),
        gloo_net::Error::JsError(error) => SchemeSelectionError::GlooError(error.message),
        gloo_net::Error::GlooError(error) => SchemeSelectionError::GlooError(error),
    }
}

async fn get_schemas_from_response(
    response: Response,
) -> Result<Vec<String>, SchemeSelectionError> {
    match response.text().await {
        Ok(text) => match serde_json::from_str::<Vec<String>>(&text) {
            Ok(schemas) => Ok(schemas),
            Err(_) => Err(SchemeSelectionError::DeserializeError(format!(
                "Unable to deserialize server response to list of schemas"
            ))),
        },
        Err(error) => Err(handle_gloo_error(error)),
    }
}
