use leptos::*;
mod components;

use crate::components::front::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <>
                <main class="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-green-400 to-blue-400">
                    <Front />
                </main>
            </>
        }
    })
}
