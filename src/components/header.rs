use yew::{function_component, Html, html};

#[function_component]
pub fn Header() -> Html {
    html! {
      <header><h1>{"This is my page"}</h1></header>
   }
}