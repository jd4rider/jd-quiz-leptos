use leptos::{
    web_sys::{window, MouseEvent},
    *,
};
#[component]
pub fn Home(
    cx: Scope,
    home_class: RwSignal<String>,
    home_mobile_class: RwSignal<String>,
    quiz_class: RwSignal<String>,
    quiz_mobile_class: RwSignal<String>,
) -> impl IntoView {
    let active_class =
        "bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium".to_string();
    let inactive_class =
        "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
            .to_string();
    let active_mobile_class =
        "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium".to_string();
    let inactive_mobile_class = "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium".to_string();

    let navbar_change_class = move || {
        let current_url = window().unwrap().location().pathname().unwrap();

        if current_url == "/" {
            home_class.set(active_class.clone());
            home_mobile_class.set(active_mobile_class.clone());
            quiz_class.set(inactive_class.clone());
            quiz_mobile_class.set(inactive_mobile_class.clone());
        } else {
            quiz_class.set(active_class.clone());
            quiz_mobile_class.set(active_mobile_class.clone());
            home_class.set(inactive_class.clone());
            home_mobile_class.set(inactive_mobile_class.clone());
        }
    };

    let handle_active = move |e: MouseEvent| {
        e.prevent_default();
        navbar_change_class();
    };

    view! {cx,
        <div class="relative overflow-hidden bg-no-repeat bg-cover" style="background-position: 50%; background-image: url('/public/images/041.webp'); height: 500px;"></div>

        <div class="container mx-auto px-6 md:px-12 xl:px-32">
          <div class="text-center text-gray-800">
            <div class="block rounded-lg shadow-lg px-6 py-12 md:py-16 md:px-12" style="margin-top: -170px; background: hsla(0, 0%, 100%, 0.7); backdrop-filter: blur(30px);">
              <h1 class="text-5xl md:text-6xl xl:text-7xl font-bold tracking-tight mb-12">"Welcome to JD Quiz" <br /><span class="text-blue-600">"This app is written in Leptos Rust"</span></h1>
              <a href="/quiz" class="inline-block px-7 py-3 mb-2 md:mb-0 mr-0 md:mr-2 bg-blue-600 text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
                 data-mdb-ripple="true"
                 data-mdb-ripple-color="light"
                 role="button"
                 on:click=handle_active>
                 "Try the Quiz"
              </a>
              <a class="inline-block px-7 py-3 text-white font-medium text-sm leading-snug bg-transparent text-blue-600 font-medium text-xs leading-tight uppercase rounded hover:text-blue-700 hover:bg-gray-100 focus:bg-gray-100 focus:outline-none focus:ring-0 active:bg-gray-200 transition duration-150 ease-in-out"
                 data-mdb-ripple="true"
                 data-mdb-ripple-color="light"
                 target="_blank"
                 href="https://github.com/leptos-rs/leptos"
                 role="button">"Learn more about Leptos"</a>
            </div>
          </div>
        </div>
    }
}
