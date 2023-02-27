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
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn CardForm(_props: &CardFormProps) -> Html {
    let default_fields = _props.default_fields.clone().unwrap_or(CardFormEntity { title: String::from(""), content: String::from("") });
    let title_state = use_state(|| default_fields.title);
    let content_state = use_state(|| default_fields.content);

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

    let title_state_cloned = title_state.clone();
    let content_state_cloned = content_state.clone();
    let onchange_cloned = _props.onchange.clone();
    use_effect_with_deps(move |_| {
        let card_form_data = CardFormEntity { title: (*title_state_cloned).clone(), content: (*content_state_cloned).clone() };
        onchange_cloned.emit(card_form_data);
    }, [title_state.clone(), content_state.clone()]);

    html! {
        <form>
            <label>{"title: "}<input ref={input_node_ref} onchange={onchange_input} value={(*title_state).clone()} /></label>
            <label>{"content: "}<textarea ref={textarea_node_ref}  onchange={onchange_textarea} value={(*content_state).clone()} /></label>
            {_props.children.clone()}
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

    let card_form_data_cloned = card_form_data.clone();
    let card_form_data_update = card_form_data.clone();

    let onchange = use_callback(move |form_data, _| {
        card_form_data.set(form_data);
    }, ());

    let cards_state: UseStateHandle<Vec<CardEntity>> = use_state(|| vec![
        CardEntity { title: String::from("title1"), content: String::from("content1"), date: Utc::now() },
        CardEntity { title: String::from("title2"), content: String::from("content2"), date: Utc::now() },
    ]);

    let cards_state_cloned = cards_state.clone();
    let onclick_submit = Callback::from(move |_| {
        let _card_form_data_cloned = card_form_data_cloned.clone();
        let _cards_state_cloned = cards_state_cloned.clone();
        let mut cards_cloned: Vec<_> = _cards_state_cloned.to_vec();
        let _title = (*_card_form_data_cloned).title.clone();
        let _content = (*_card_form_data_cloned).content.clone();
        let new_card = CardEntity { title: _title, content: _content, date: Utc::now() };
        cards_cloned.push(new_card);
        _cards_state_cloned.set(cards_cloned);
    });

    let display_card_form = if *show_new_card == true {
        html! {
            <CardForm onchange={onchange.clone()}>
                <button onclick={onclick_submit} type="button">{"Add"}</button>
            </CardForm>
        }
    } else {
        html! {
            <></>
        }
    };

    let edit_mode_idx: UseStateHandle<Option<i32>> = use_state(|| None);
    let card_comps = (*cards_state).iter().enumerate().map(|(idx, props)| {
        let default_fields = CardFormEntity {
            title: props.title.clone(),
            content: props.content.clone(),
        };
        let edit_mode_idx_cloned = edit_mode_idx.clone();
        let show_new_card_cloned = show_new_card.clone();

        let _card_form_data_update = card_form_data_update.clone();
        let fields_update_form = default_fields.clone();
        let onclick = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            _card_form_data_update.set(fields_update_form.clone());
            edit_mode_idx_cloned.set(Some(idx.try_into().unwrap()));
            show_new_card_cloned.set(false);
        });

        let show_new_card_cloned2 = show_new_card.clone();
        let edit_mode_idx_for_update = edit_mode_idx.clone();
        let card_form_data_update_for_submit = card_form_data_update.clone();

        let cards_state_for_update = cards_state.clone();
        let _cards_for_update = cards_state.clone();
        let onclick_update = Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let _cards_state = cards_state_for_update.clone();
            let mut _cards: Vec<_> = cards_state_for_update.to_vec();

            let _title = (*card_form_data_update_for_submit).title.clone();
            let _content = (*card_form_data_update_for_submit).content.clone();
            let new_card = CardEntity { title: _title, content: _content, date: Utc::now() };

            _cards[edit_mode_idx_for_update.unwrap() as usize] = new_card;
            _cards_state.set(_cards);

            show_new_card_cloned2.set(true);
            edit_mode_idx_for_update.set(None);
        });
        let card_display = match *edit_mode_idx {
            Some(selected_id) => {
                if selected_id.eq(&<usize as TryInto<i32>>::try_into(idx).unwrap()) {
                    html! {
                        <CardForm onchange={onchange.clone()} {default_fields}>
                            <button onclick={onclick_update} type="button">{"Update"}</button>
                        </CardForm>
                    }
                } else {
                    html! {
                        <div onclick={onclick.clone()}>
                            <Card
                                key={props.date.to_string().clone()}
                                title={props.title.clone()}
                                content={props.content.clone()}
                                date={props.date.clone()}
                            />
                        </div>
                    }
                }
            }
            None => html! {
                <div onclick={onclick.clone()}>
                    <Card
                        key={props.date.to_string().clone()}
                        title={props.title.clone()}
                        content={props.content.clone()}
                        date={props.date.clone()}
                    />
                </div>
            },
        };
        html! {
            <li>
                {card_display}
            </li>
        }
    }).collect::<Html>();

    html! {
        <div>
            {display_card_form}
            <div>
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
