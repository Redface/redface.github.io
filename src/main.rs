use yew::{function_component, html};

mod components;

use components::{
    header::Header,
    footer::Footer,
    body::Body,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <Body />
            <Footer />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
