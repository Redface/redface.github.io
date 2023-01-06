use yew::{function_component, Html, html};

mod components;

use components::{
    header::Header,
    footer::Footer,
    body::Body,
};

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <Body />
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
