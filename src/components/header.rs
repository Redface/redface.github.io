use yew::{function_component, html};

#[function_component(Header)]
pub fn header_component() -> Html {
    html! {
      <header><h1>{"This is my page"}</h1></header>
   }
}