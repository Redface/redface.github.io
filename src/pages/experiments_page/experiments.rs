use yew::{function_component, Html, html};
use yew_router::prelude::*;
use crate::pages::experiments_page::card_crud::card_crud::CardCrud;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/experiments")]
    Index,
    #[at("/experiments/card_crud")]
    CardCRUD,
}

#[function_component]
fn Index() -> Html {
    html! {
        <div>
        {"Experiments Index"}
            <div>
                <Link<Route> to={Route::CardCRUD}>{ "Card CRUD" }</Link<Route>>
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::CardCRUD => html! { <CardCrud /> },
    }
}

#[function_component]
pub fn Experiments() -> Html {
    html! {
        <div>
            {" Experiments Root "}
            <Switch<Route> render={switch} />
        </div>
    }
}
