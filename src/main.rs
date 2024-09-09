mod components;
mod pages;

use yew_router::prelude::*;
use yew::prelude::*;


use pages::{
    experiments_page::experiments::Experiments,
    home_page::home::Home,
    not_found::NotFound,
};
use components::{
    header::Header,
    footer::Footer,
    nav::Nav,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/experiments")]
    ExperimentsRoot,
    #[at("/experiments/*")]
    Experiments,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::ExperimentsRoot | Route::Experiments => html! { <Experiments /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
fn App() -> Html {
    // logger init
    wasm_logger::init(wasm_logger::Config::default());
    html! {
        <>
            <Header />
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch} />
            </BrowserRouter>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
