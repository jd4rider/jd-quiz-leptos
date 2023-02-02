use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
       // <!-- Jumbotron -->
        <div
          class="p-12 text-center relative overflow-hidden bg-no-repeat bg-cover rounded-lg"
          style="
            background-image: url('https://mdbcdn.b-cdn.net/img/new/slides/041.webp');
            height: 400px;
          "
        >
          <div
            class="absolute top-0 right-0 bottom-0 left-0 w-full h-full overflow-hidden bg-fixed"
            style="background-color: rgba(0, 0, 0, 0.6)"
          >
            <div class="flex justify-center items-center h-full">
              <div class="text-white">
                <h2 class="font-semibold text-4xl mb-4">"Welcome to JD Quiz"</h2>
                <h4 class="font-semibold text-xl mb-6">"This app is written in Leptos Rust"</h4>
                <a
                  class="inline-block px-7 py-3 mb-1 border-2 border-gray-200 text-gray-200 font-medium text-sm leading-snug uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                  href="/quiz"
                  role="button"
                  data-mdb-ripple="true"
                  data-mdb-ripple-color="light">
                  "Try the Quiz"
                </a>
              </div>
            </div>
          </div>
        </div>
       // <!-- Jumbotron -->
    }
}
