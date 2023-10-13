pub mod components;
pub mod state;
use crate::components::{Home, StyewardDataTables};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen(module = "/js/config.js")]
extern "C" {
    fn get_host() -> String;
}

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/site")]
    Home,
    #[at("/site/datatable")]
    DataTable,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<Home/>},
        MainRoute::DataTable => html! {<StyewardDataTables/>},
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

#[derive(PartialEq, Clone)]
struct StyewardConfig {
    scheme: String,
    host: String,
    port: u16,
}

#[function_component]
pub fn App() -> Html {
    let config = StyewardConfig {
        scheme: "https".to_string(),
        host: get_host(),
        port: 443,
    };
    html! {
        <main>
            <ContextProvider<StyewardConfig> context={config}>
                <BrowserRouter>
                    <Switch<MainRoute> render={switch_main} />
                </BrowserRouter>
            </ContextProvider<StyewardConfig>>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
