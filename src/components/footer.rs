use yew::{function_component, html};

#[function_component(Footer)]
pub fn header_component() -> Html {
    html! {
        <footer>
            {"Contact: "}<a href="mailto:redface0512@gmail.com">{"Here"}</a>
        </footer>
   }
}
