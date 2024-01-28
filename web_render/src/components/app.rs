use crate::components::falling_sand::FallingSand;
use leptos::*;
use std::time::Duration;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <FallingSand width=512 height=512 tick_delay=Duration::new(0, 500) />
    }
}
