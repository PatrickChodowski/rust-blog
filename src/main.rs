use yew::prelude::*;

mod about;
mod counter;

#[function_component]
fn App() -> Html {

    html! {
        <>
            <about::AboutPage />
            <counter::Counter />
            
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}