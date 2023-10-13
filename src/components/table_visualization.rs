use crate::state::StyewardState;
use crate::StyewardConfig;

use std::collections::HashMap;

use gloo_net;
use gloo_net::http::{Request, Response};
use gloo_utils::document;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console::error_1;
use yew::functional::use_context;
use yew::prelude::*;
use yew::prelude::{function_component, html, Html, UseStateHandle};
use yew::suspense::use_future_with_deps;

const AG_GRID_DIV_ID: &str = "grid-div";

pub enum TableVisualizationError {
    EndpointError(String),
    GlooError(String),
    HttpStatusError(String),
    DeserializeError(String),
    TablePropsError(String),
    TableDataFormatError(String),
    TableVisualizationError(String),
}

fn get_error_div(label: String) -> Html {
    html! {
        <div>
            {label.clone()}
        </div>
    }
}

#[function_component]
pub fn TableVisualization() -> Html {
    let state_handle = use_context::<UseStateHandle<StyewardState>>();
    let current_schema = match state_handle {
        Some(ref handle) => handle.current_schema.clone(),
        None => None,
    };
    let current_table = match state_handle {
        Some(ref handle) => handle.current_table.clone(),
        None => None,
    };
    html! {<ValidateLocalDeps schema={current_schema} table={current_table}/>}
}

#[derive(Properties, PartialEq)]
struct ValidateLocalDepsProp {
    schema: Option<String>,
    table: Option<String>,
}

#[function_component]
fn ValidateLocalDeps(prop: &ValidateLocalDepsProp) -> Html {
    let schema = prop.schema.clone();
    let table = prop.table.clone();
    log::info!("schema/table: {:?}/{:?}", schema, table);
    if schema.is_some() && table.is_some() {
        log::info!("Trying to visualize the table");
        html! {<ValidateConfig schema={schema.unwrap()} table={table.unwrap()}/>}
    } else if schema.is_some() && table.is_none() {
        log::info!("Trying to re-visualize the table");
        match document().get_element_by_id(AG_GRID_DIV_ID) {
            Some(element) => html! { <div class="p-0 m-0">{Html::VRef(element.into())}</div>},
            None => html! {},
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct ValidateConfigProp {
    schema: String,
    table: String,
}

#[function_component]
fn ValidateConfig(prop: &ValidateConfigProp) -> Html {
    match use_context::<StyewardConfig>() {
        Some(config) => {
            html! {
                <FetchRemoteDeps
                 scheme={config.scheme.clone()}
                 host={config.host.clone()}
                 port={config.port.clone()}
                 schema={prop.schema.clone()}
                 table={prop.table.clone()} />
            }
        }
        None => {
            let msg = String::from("The config object is None.");
            error_1(&msg.into());
            get_error_div(format!("error!"))
        }
    }
}

#[derive(Properties, PartialEq)]
struct FetchRemoteDepsProp {
    scheme: String,
    host: String,
    port: u16,
    schema: String,
    table: String,
}

#[function_component]
fn FetchRemoteDeps(prop: &FetchRemoteDepsProp) -> Html {
    let scheme = prop.scheme.clone();
    let host = prop.host.clone();
    let port = prop.port.clone();
    let schema = prop.schema.clone();
    let table = prop.table.clone();

    let remote_table_data = use_future_with_deps(
        |table| async move {
            match get_table_url(
                &scheme.clone(),
                &host.clone(),
                port.clone(),
                &schema,
                &table,
            )
            .await
            {
                Ok(data_url) => match get_table_data(&scheme, &host, port, &data_url).await {
                    Ok(table_data) => Ok(table_data),
                    Err(error) => Err(error),
                },
                Err(error) => Err(error),
            }
        },
        table.clone(),
    );
    match remote_table_data {
        Ok(result_ref) => match &*result_ref {
            Ok(row_data) => html! {<VisualizeTable row_data={row_data.clone()} />},
            Err(error) => match error {
                TableVisualizationError::EndpointError(msg)
                | TableVisualizationError::GlooError(msg)
                | TableVisualizationError::HttpStatusError(msg)
                | TableVisualizationError::DeserializeError(msg)
                | TableVisualizationError::TableDataFormatError(msg)
                | TableVisualizationError::TableVisualizationError(msg) => {
                    error_1(&msg.into());
                    get_error_div(format!("error!"))
                }
                TableVisualizationError::TablePropsError(msg) => {
                    error_1(&msg.into());
                    html! {}
                }
            },
        },
        Err(_) => {
            html! {
                <div id="spinner" class="visible text-center position-absolute top-50 start-0" hidden=false>
                    <div class="spinner-grow text-secondary" style="width: 10em; height: 10em;" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            }
        }
    }
}

#[derive(Properties, PartialEq)]
struct VisualizeTableProp {
    row_data: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AGGridData {
    pub row_data: Vec<HashMap<String, String>>,
    pub col_defs: Vec<HashMap<String, String>>,
}

#[derive(Properties, PartialEq)]
struct CallJavaScriptCodeProp {
    ag_grid_data: AGGridData,
}

#[function_component]
fn VisualizeTable(prop: &VisualizeTableProp) -> Html {
    // Get the column names
    let mut col_defs = Vec::<HashMap<String, String>>::new();
    match prop.row_data.first() {
        Some(first_row) => {
            for (k, _v) in first_row.clone() {
                let mut field = HashMap::<String, String>::new();
                field.insert(String::from("field"), k.clone());
                if k.eq("subject") {
                    field.insert(String::from("pinned"), String::from("left"));
                }
                col_defs.push(field.clone());
            }
        }
        None => (),
    }

    let ag_grid_data = AGGridData {
        row_data: prop.row_data.clone(),
        col_defs: col_defs.clone(),
    };

    html! { <CallJavaScriptCode ag_grid_data={ag_grid_data}/>}
}

#[wasm_bindgen(module = "/js/ag_grid.js")]
extern "C" {
    fn create_grid(ag_grid_data: JsValue, grid_div_id: String) -> bool;
}

#[function_component]
fn CallJavaScriptCode(prop: &CallJavaScriptCodeProp) -> Html {
    use_effect_with_deps(
        |ag_grid_data| {
            let ag_grid_data = ag_grid_data.clone();
            let grid_div_id = String::from(AG_GRID_DIV_ID);
            match document().get_element_by_id(grid_div_id.as_str()) {
                Some(_div) => {
                    let ag_grid_data_js_value = JsValue::from_serde(&ag_grid_data).unwrap();
                    create_grid(ag_grid_data_js_value, grid_div_id);
                }
                None => (),
            };
            || {}
        },
        prop.ag_grid_data.clone(),
    );

    html! { <div id={AG_GRID_DIV_ID} class="ag-theme-alpine px-0" style="width: 100%; height: 100%; margin-top:10px;"></div>}
}

async fn get_table_data(
    scheme: &String,
    host: &String,
    port: u16,
    data_url: &String,
) -> Result<Vec<HashMap<String, String>>, TableVisualizationError> {
    let endpoint = format!("{scheme}://{host}:{port}/{data_url}");
    match Url::parse(&endpoint) {
        Ok(url) => get_table_data_with_gloo(url).await,
        Err(error) => Err(TableVisualizationError::EndpointError(format!("{error}"))),
    }
}

async fn get_table_data_with_gloo(
    url: Url,
) -> Result<Vec<HashMap<String, String>>, TableVisualizationError> {
    match Request::get(url.as_str()).send().await {
        Ok(response) => {
            if response.ok() {
                get_table_data_from_response(response).await
            } else {
                match response.text().await {
                    Ok(body_string) => Err(TableVisualizationError::HttpStatusError(body_string)),
                    Err(_) => Err(TableVisualizationError::HttpStatusError(format!(
                        "Status: {}",
                        response.status()
                    ))),
                }
            }
        }
        Err(error) => Err(handle_gloo_error(error)),
    }
}

async fn get_table_data_from_response(
    response: Response,
) -> Result<Vec<HashMap<String, String>>, TableVisualizationError> {
    match response.text().await {
        Ok(text) => {
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(data) => {
                    // Deconvolute the table data.
                    match build_table_data_struct(&data) {
                        Ok(table_data) => Ok(table_data),
                        Err(error) => Err(error),
                    }
                }
                Err(_) => Err(TableVisualizationError::DeserializeError(format!(
                    "Unable to deserialize server response to list of schemas"
                ))),
            }
        }
        Err(error) => Err(handle_gloo_error(error)),
    }
}

fn build_table_data_struct(
    data: &Value,
) -> Result<Vec<HashMap<String, String>>, TableVisualizationError> {
    if !data.is_array() {
        return Err(TableVisualizationError::TableDataFormatError(
            "Data requested is not an array.".to_owned(),
        ));
    }

    // Map the data to a datasource to load in AG-grid
    let mut datasource = Vec::<HashMap<String, String>>::new();
    match data.as_array() {
        Some(table_array) => {
            for item in table_array {
                let mut row = HashMap::<String, String>::new();
                match item.as_object() {
                    Some(row_data_map) => {
                        for (k, v) in row_data_map {
                            if v.is_string() {
                                let string_value = v.as_str().unwrap().to_owned();
                                row.insert(k.to_string(), string_value);
                            } else if v.is_null() {
                                let string_value = "N/A".to_owned();
                                row.insert(k.to_string(), string_value);
                            }
                            // look into converting f64 ints to have only
                            else if v.is_f64() {
                                let string_value = format!("{:.5}", v.as_f64().unwrap());
                                row.insert(k.to_string(), string_value);
                            } else if v.is_u64() {
                                let string_value = v.as_u64().unwrap().to_string();
                                row.insert(k.to_string(), string_value);
                            } else if v.is_boolean() {
                                let string_value = v.as_bool().unwrap().to_string();
                                row.insert(k.to_string(), string_value);
                            } else if v.is_i64() {
                                let string_value = v.as_i64().unwrap().to_string();
                                row.insert(k.to_string(), string_value);
                            } else if v.is_number() {
                                let string_value = v.as_f64().unwrap().to_string();
                                row.insert(k.to_string(), string_value);
                            } else {
                                // Don't support anything but null and string values, ideally no null values should exist after 'cleansing'
                                return Err(TableVisualizationError::TableDataFormatError("A non-null or non-string value encountered in the returned data.".to_owned()));
                            }
                        }
                    }
                    None => {
                        return Err(TableVisualizationError::TableDataFormatError(
                            "non-row type data encountered in the data received.".to_owned(),
                        ))
                    }
                }
                datasource.push(row.clone());
            }
        }
        None => {
            return Err(TableVisualizationError::TableDataFormatError(
                "No series of rows found in data received.".to_owned(),
            ))
        }
    }
    Ok(datasource)
}

async fn get_table_url(
    scheme: &String,
    host: &String,
    port: u16,
    schema: &String,
    table: &String,
) -> Result<String, TableVisualizationError> {
    let endpoint = format!("{scheme}://{host}:{port}/api/v1/{schema}/{table}/uri");
    match Url::parse(&endpoint) {
        Ok(url) => get_table_url_with_gloo(url).await,
        Err(error) => Err(TableVisualizationError::EndpointError(format!("{error}"))),
    }
}

async fn get_table_url_with_gloo(url: Url) -> Result<String, TableVisualizationError> {
    match Request::get(url.as_str()).send().await {
        Ok(response) => {
            if response.ok() {
                get_table_url_from_response(response).await
            } else {
                match response.text().await {
                    Ok(body_string) => Err(TableVisualizationError::HttpStatusError(body_string)),
                    Err(_) => Err(TableVisualizationError::HttpStatusError(format!(
                        "Status: {}",
                        response.status()
                    ))),
                }
            }
        }
        Err(error) => Err(handle_gloo_error(error)),
    }
}

fn handle_gloo_error(gloo_error: gloo_net::Error) -> TableVisualizationError {
    match gloo_error {
        gloo_net::Error::SerdeError(error) => {
            TableVisualizationError::GlooError(format!("{error}"))
        }
        gloo_net::Error::JsError(error) => TableVisualizationError::GlooError(error.message),
        gloo_net::Error::GlooError(error) => TableVisualizationError::GlooError(error),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableLocation {
    pub url: String,
}

async fn get_table_url_from_response(
    response: Response,
) -> Result<String, TableVisualizationError> {
    match response.text().await {
        Ok(text) => match serde_json::from_str::<TableLocation>(&text) {
            Ok(table_location) => Ok(table_location.url),
            Err(_) => Err(TableVisualizationError::DeserializeError(format!(
                "Unable to deserialize server response to list of schemas"
            ))),
        },
        Err(error) => Err(handle_gloo_error(error)),
    }
}
