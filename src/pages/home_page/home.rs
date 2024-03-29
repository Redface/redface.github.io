use yew::prelude::*;

#[function_component]
fn Test() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {
        <div>
            <button {onclick}>{ "Hit as much as you like it (Just practicing Rust Lang & Yew)" }</button>
            <p>
                <b>{ "Like: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <main>
            <section>
                <header>
                    <h2>{"Daehwan 'Redface' Lee"}</h2>
                </header>
                <div>
                    <ul>
                        <li>{"What I do for living: Front-end developer"}</li>
                        <li>{"What I study for fun: Rust lang"}</li>
                        <li>{"What I do for life/fun: Dance, Art form"}</li>
                    </ul>
                </div>
            </section>
            <Test />
        </main>
   }
}
