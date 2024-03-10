use crate::element_api::ElementApi;
use crate::DOWN;

pub fn solid_behaviour(mut api: ElementApi) {
    api.flip_visited();
    let _ = api.swap(DOWN);
}
