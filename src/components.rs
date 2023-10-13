pub mod current_selection;
pub mod nav;
pub mod schema;
pub mod table_description;
pub mod table_selection;
pub mod table_visualization;

use crate::state::StyewardState;

use current_selection::CurrentSelection;
use nav::NavBar;
use schema::SchemaSelection;
use table_selection::TableSelection;
use table_visualization::TableVisualization;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="container-fluid bg-light m-0 p-0" style="height: 100vh">
            <div class="row">
                <NavBar/>
            </div>
            <div class="row bg-light text-dark px-3 py-4">
                <div class="col">
                    <span class="align-middle">
                        {"Navigate to the DataTable page to visualize data tables in real time."}
                    </span>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn StyewardDataTables() -> Html {
    let initial_state = use_state(|| StyewardState {
        current_schema: None,
        current_table: None,
    });

    html! {
        <div class="container-fluid bg-light m-0 p-0" style="height: 100vh">
            <div class="row">
                <NavBar/>
            </div>
            <div class="row bg-light text-dark px-3 py-4">
                <div class="col">
                    <span class="align-middle">{"Explore Agenus clinical trial data by schema and table."}</span>
                </div>
            </div>
            <div class="row px-3">
                <ContextProvider<UseStateHandle<StyewardState>> context={initial_state}>
                    <div class="container px-2">
                        <div class="row">
                            <div class="col shadow-sm px-0 ms-3 flex" style="max-width: 300px;"><SchemaSelection/></div>
                            <div class="col shadow-sm px-0 ms-3 me-3 flex" style="max-width: 300px;"><TableSelection/></div>
                        </div>
                         <div class="row">
                            <div class="col-auto px-0 ms-3 flex" style="max-width: 300px;"> <CurrentSelection/></div>
                        </div>

                        <div class="row p-0 m-0 justify-content-md-center" style="min-height: 500px; height: calc(100vh - 290px);">
                            <TableVisualization/>
                        </div>
                    </div>
                </ContextProvider<UseStateHandle<StyewardState>>>
            </div>
        </div>
    }
}
