use leptos::*;
use crate::components::falling_sand::FallingSand;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <FallingSand width=512 height=512 />
    }
}
