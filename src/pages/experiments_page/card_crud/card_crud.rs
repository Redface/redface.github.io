use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use log::info;
use chrono::{Utc, DateTime};

#[derive(Clone, Debug, PartialEq)]
pub struct CardEntity {
    title: String,
    content: String,
    date: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CardFormEntity {
    title: String,
    content: String,
}

#[derive(Properties, PartialEq)]
pub struct CardFormProps {
    pub onchange: Callback<CardFormEntity>,
    pub default_fields: Option<CardFormEntity>,
}

#[function_component]
pub fn CardForm(_props: &CardFormProps) -> Html {
    let title_state = use_state(|| String::from("redface"));
    let content_state = use_state(|| String::from("awesome"));

    let input_node_ref = use_node_ref();
    let onchange_input = {
        let _node_ref = input_node_ref.clone();
        let _title_state = title_state.clone();
        Callback::from(move |_| {
            let el = _node_ref.cast::<HtmlInputElement>();
            if let Some(el) = el {
                _title_state.set(el.value());
            }
        })
    };

    let textarea_node_ref = use_node_ref();
    let onchange_textarea = {
        let _node_ref = textarea_node_ref.clone();
        let _content_state = content_state.clone();
        Callback::from(move |_| {
            let el = _node_ref.cast::<HtmlTextAreaElement>();
            if let Some(el) = el {
                _content_state.set(el.value());
            }
        })
    };

    let card_form_data = CardFormEntity { title: (*title_state).clone(), content: (*content_state).clone() };
    _props.onchange.emit(card_form_data.clone());
    html! {
        <form>
            <label>{"title: "}<input ref={input_node_ref} onchange={onchange_input} value={card_form_data.title.clone()} /></label>
            <label>{"content: "}<textarea ref={textarea_node_ref}  onchange={onchange_textarea} value={card_form_data.content.clone()} /></label>
        </form>
    }
}

#[function_component]
pub fn CardCrud() -> Html {
    let show_new_card = use_state(|| true);
    let card_form_data = use_state(|| CardFormEntity {
        title: String::from(""),
        content: String::from(""),
    });
    let _card_form_data_cloned = card_form_data.clone();

    let onchange = use_callback(move |form_data, _| {
        card_form_data.set(form_data);
    }, _card_form_data_cloned.clone());
    let display_card_form = if *show_new_card == true {
        html! {
            <CardForm {onchange} />
        }
    } else {
        html! {
            <></>
        }
    };

    let cards_state: UseStateHandle<Vec<CardEntity>> = use_state(|| vec![
        CardEntity { title: String::from("title1"), content: String::from("content1"), date: Utc::now() },
        CardEntity { title: String::from("title2"), content: String::from("content2"), date: Utc::now() },
    ]);

    let onclick = {
        let cards_state_cloned = cards_state.clone();
        Callback::from(move |_| {
            let mut cards_cloned: Vec<_> = cards_state_cloned.to_vec();
            let _title = (*_card_form_data_cloned).title.clone();
            let _content = (*_card_form_data_cloned).content.clone();
            let new_card = CardEntity { title: _title, content: _content, date: Utc::now() };
            cards_cloned.push(new_card);
            cards_state_cloned.set(cards_cloned);
        })
    };

    let edit_mode_idx: UseStateHandle<Option<i32>> = use_state(|| None);
    let card_comps = (*cards_state).iter().enumerate().map(|(idx, props)| {
        let edit_mode_idx_cloned = edit_mode_idx.clone();
        let onclick = Callback::from(move |_: _| {
            info!("card clicked: {:?}", idx);
            edit_mode_idx_cloned.set(Some(idx.try_into().unwrap()));
        });
        let onchange = Callback::from(|_: _| {});
        let default_fields = CardFormEntity {
            title: props.title.clone(),
            content: props.content.clone(),
        };
        info!("selected idx: {:?}", *edit_mode_idx);
        let card_display = match *edit_mode_idx {
            Some(selected_id) => {
                if selected_id.eq(&<usize as TryInto<i32>>::try_into(idx).unwrap()) {
                    html! {
                        <CardForm {onchange} {default_fields} />
                    }
                } else {
                    html! {
                        <Card
                            key={props.date.to_string().clone()}
                            title={props.title.clone()}
                            content={props.content.clone()}
                            date={props.date.clone()}
                        />
                    }
                }
            }
            None => html! {
                <Card
                    key={props.date.to_string().clone()}
                    title={props.title.clone()}
                    content={props.content.clone()}
                    date={props.date.clone()}
                />
            },
        };
        html! {
            <li {onclick}>
                {card_display}
            </li>
        }
    }).collect::<Html>();
    info!("cards {:?}", card_comps);

    html! {
        <div>
            {display_card_form}
            <div>
                <button {onclick}>{"Add"}</button>
                <ol>
                    {card_comps}
                </ol>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub content: String,
    pub date: DateTime<Utc>,
}

#[function_component]
fn Card(_props: &CardProps) -> Html {
    html! {
            <div>
                <div>{_props.title.clone()}</div>
                <div>{_props.content.clone()}</div>
                <div>{_props.date.to_string().clone()}</div>
            </div>
        }
}
