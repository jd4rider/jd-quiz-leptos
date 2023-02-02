use leptos::*;
use leptos_router::*;

use crate::partials::navbar::*;

use crate::components::front::*;

use crate::pages::home::*;

#[component]
pub fn Routing(cx: Scope) -> impl IntoView {
    view! {cx,
    //        <div id="root">
              // we wrap the whole app in a <Router/> to allow client-side navigation
              // from our nav links below
              <Router>
                // <nav> and <main> will show on every route
                /*<nav>
                  // LR will enhance the active <a> link with the [aria-current] attribute
                  // we can use this for styling them with CSS like `[aria-current] { font-weight: bold; }`
                  <A href="contacts">"Contacts"</A>
                  <A href="about">"About"</A>
                  <A href="settings">"Settings"</A>
                </nav>*/
                <Navbar />
                  // <Routes/> both defines our routes and shows them on the page
                  <Routes>
                    // our root route: the contact list is always shown
                    <Route
                      path=""
                      view=move |cx| view! {cx, <Home/> }
                    />
                    <Route
                      path="quiz"
                      view=move |cx| view! { cx,  <Front/> }
                    />
                      // users like /gbj or /bob
                      /*<Route
                        path=":id"
                        view=move |cx| view! { cx,  <Contact/> }
                      />
                      // a fallback if the /:id segment is missing from the URL
                      <Route
                        path=""
                        view=move |_| view! { cx,  <p class="contact">"Select a contact."</p> }
                      />
                    </Route>
                    // LR will automatically use this for /about, not the /:id match above
                    <Route
                      path="about"
                      view=move |cx| view! { cx,  <About/> }
                    />*/
                  </Routes>
              </Router>
    //        </div>
        }
}
