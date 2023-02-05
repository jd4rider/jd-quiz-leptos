use leptos::*;
use leptos_router::*;

use crate::partials::navbar::*;

use crate::components::front::*;

use crate::pages::home::*;

#[component]
pub fn Routing(cx: Scope) -> impl IntoView {
    let active_class =
        "bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium".to_string();
    let inactive_class =
        "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
            .to_string();
    let active_mobile_class =
        "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium".to_string();
    let inactive_mobile_class = "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium".to_string();
    let home_class = create_rw_signal(cx, inactive_class.clone());
    let home_mobile_class = create_rw_signal(cx, inactive_mobile_class.clone());
    let quiz_class = create_rw_signal(cx, inactive_class.clone());
    let quiz_mobile_class = create_rw_signal(cx, inactive_mobile_class.clone());
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
                <Navbar home_class=home_class.clone() home_mobile_class=home_mobile_class.clone() quiz_class=quiz_class.clone() quiz_mobile_class=quiz_mobile_class.clone()/>
                  // <Routes/> both defines our routes and shows them on the page
                  <Routes>
                    // our root route: the contact list is always shown
                    <Route
                      path=""
                      view=move |cx| view! {cx, <Home home_class=home_class.clone() home_mobile_class=home_mobile_class.clone() quiz_class=quiz_class.clone() quiz_mobile_class=quiz_mobile_class.clone()/> }
                    />
                    <Route
                      path="pages/quiz"
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
