use yew::{function_component, Html, html, Properties};

#[function_component]
pub fn Nav() -> Html {
    let nav_list = vec![
        LinkMenuProps { link: String::from("/"), text: String::from("Home") },
        LinkMenuProps { link: String::from("/experiments"), text: String::from("Experiments") },
    ];
    let link_menus = nav_list
        .iter()
        .map(|link_menu| html! {
            <LinkMenu key={link_menu.link.clone()} link={link_menu.link.clone()} text={link_menu.text.clone()}/>
        })
        .collect::<Html>();
    html! {
        <nav>
            {link_menus}
        </nav>
   }
}

#[derive(Properties, PartialEq)]
pub struct LinkMenuProps {
    pub link: String,
    pub text: String,
}

#[function_component]
pub fn LinkMenu(props: &LinkMenuProps) -> Html {
    html! {
        <a href={props.link.clone()}>{props.text.clone()}</a>
    }
}
