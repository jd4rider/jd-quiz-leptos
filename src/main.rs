use leptos::*;
mod components;
mod partials;

use crate::partials::navbar::*;

use crate::components::front::*;

use crate::components::routing::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <>
                //<Navbar />
                //<main class="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-green-400 to-blue-400">
                    <Routing />
                    //<Front />
                //</main>
            </>
        }
    })
}
