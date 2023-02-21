use yew::{function_component, Html, html};

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div>
            {"Page not found"}
        </div>
   }
}
