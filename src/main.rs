use leptos::*;
mod components;
mod pages;
mod partials;

use crate::components::routing::*;

use crate::components::front::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <>
                <Routing />
            </>
        }
    })
}
