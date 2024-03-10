use crate::element_api::ElementApi;
use crate::{DOWN, LEFT};

#[rustfmt::skip]
#[allow(clippy::short_circuit_statement)]
pub fn powder_behaviour(mut api: ElementApi) {
    api.flip_visited();

    let dx = api.rand_dir() as isize;
    let _ = api.swap(DOWN)
         || api.swap(DOWN + LEFT * dx)
         || api.swap(DOWN + LEFT * -dx);
}
