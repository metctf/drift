use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html!{
        <div class={classes!("not-found")}>
            <h1>{"O you don't have the right, o you don't have the right, o you don't have the right."}</h1>
        </div>
        }
}
