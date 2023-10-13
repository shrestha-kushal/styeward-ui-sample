use crate::MainRoute;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav class="navbar navbar-dark navbar-expand-lg bg-dark px-3 py-2">
            <div class="container-fluid px-1">
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarText" aria-controls="navbarText" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarText">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <a class="nav-link"><Link<MainRoute> to={MainRoute::Home}>{ "Home" }</Link<MainRoute>></a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link"><Link<MainRoute> to={MainRoute::DataTable}>{ "Data Tables" }</Link<MainRoute>></a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
