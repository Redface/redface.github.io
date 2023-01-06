use yew::{function_component, Html, html};

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer>
            {"Contact: "}<a href="mailto:redface0512@gmail.com">{"Here"}</a>
        </footer>
   }
}
