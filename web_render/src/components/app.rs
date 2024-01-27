use crate::components::falling_sand::FallingSand;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <FallingSand width=512 height=512 />
    }
}
