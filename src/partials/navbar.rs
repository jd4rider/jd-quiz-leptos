use leptos::{
    web_sys::{window, MouseEvent},
    *,
};
use leptos_router::*;

#[component]
pub fn Navbar(
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
    let (profile_class, set_profile_class) = create_signal(cx, "hidden absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none".to_string());
    let (mobile_menu_class, set_mobile_menu_class) =
        create_signal(cx, "hidden space-y-1 px-2 pt-2 pb-3".to_string());
    let (hamburger_class, set_hamburger_class) = create_signal(cx, "block h-6 w-6".to_string());
    let (close_class, set_close_class) = create_signal(cx, "hidden h-6 w-6".to_string());

    let _open_profile = move |_: MouseEvent| {
        if profile_class.get() == "hidden absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" {
            set_profile_class("absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none".to_string());
        } else {
            set_profile_class("hidden absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none".to_string());
        }
    };

    let open_mobile_menu = move |_| {
        if mobile_menu_class.get() == "hidden space-y-1 px-2 pt-2 pb-3" {
            set_mobile_menu_class("space-y-1 px-2 pt-2 pb-3".to_string());
            set_close_class("block h-6 w-6".to_string());
            set_hamburger_class("hidden h-6 w-6".to_string());
        } else {
            set_mobile_menu_class("hidden space-y-1 px-2 pt-2 pb-3".to_string());
            set_hamburger_class("block h-6 w-6".to_string());
            set_close_class("hidden h-6 w-6".to_string());
        }
    };

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

    let navbar_change_class_effect = navbar_change_class.clone();

    let handle_active = move |e: MouseEvent| {
        e.prevent_default();
        navbar_change_class();
    };

    create_effect(cx, move |_| {
        let current_url = window().unwrap().location().pathname().unwrap();

        navbar_change_class_effect();
        let location = use_location(cx);
    });

    view! {cx,
            <nav class="bg-gray-800">
              <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="relative flex h-16 items-center justify-between">
                  <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
                    //<!-- Mobile menu button-->
                    <button type="button"
                            class="inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                            aria-controls="mobile-menu"
                            aria-expanded="false"
                            on:click=open_mobile_menu>
                      <span class="sr-only">"Open main menu"</span>
                      /*<!--
                        Icon when menu is closed.

                        Heroicon name: outline/bars-3

                        Menu open: "hidden", Menu closed: "block"
                      -->*/
                      <svg class={move || hamburger_class.get()} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                      </svg>
                      /*<!--
                        Icon when menu is open.

                        Heroicon name: outline/x-mark

                        Menu open: "block", Menu closed: "hidden"
                      -->*/
                      <svg class={move || close_class.get()} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                    <div class="flex flex-shrink-0 items-center">
                      <img class="block h-8 w-auto lg:hidden" src="/public/images/quizlogotransparent.png" alt="JD Quiz" />
                      <img class="hidden h-8 w-auto lg:block" src="/public/images/quizlogotransparent.png" alt="JD Quiz" />
                    </div>
                    <div class="hidden sm:ml-6 sm:block">
                      <div class="flex space-x-4">
                        //<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
                        //<span class="bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium" aria-current="page">
                        //<A href="">
                        //<a href="/" class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                        <a href="/" class={move || home_class.get()} on:click=handle_active.clone()>
                            "Home"
                        </a>
                        //</A>

                        <a href="/pages/quiz" class={move || quiz_class.get()} on:click=handle_active.clone()>
                            "Quiz"
                        </a>

    //                    <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">"Projects"</a>

      //                  <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">"Calendar"</a>
                      </div>
                    </div>
                  </div>
                  <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
                    <button type="button" class="rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                      <span class="sr-only">"View notifications"</span>
                      //<!-- Heroicon name: outline/bell -->
                      <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                      </svg>
                    </button>

                    //<!-- Profile dropdown -->
                    /*<div class="relative ml-3">
                      <div>
                        <button type="button"
                                class="flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                                id="user-menu-button"
                                aria-expanded="false"
                                aria-haspopup="true"
                                on:click=open_profile>
                          <span class="sr-only">"Open user menu"</span>
                          <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
                        </button>
                      </div>

                      /*<!--
                        Dropdown menu, show/hide based on menu state.

                        Entering: "transition ease-out duration-100"
                          From: "transform opacity-0 scale-95"
                          To: "transform opacity-100 scale-100"
                        Leaving: "transition ease-in duration-75"
                          From: "transform opacity-100 scale-100"
                          To: "transform opacity-0 scale-95"
                      -->*/
                      <div class={move || profile_class.get()} role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button" tabindex="-1">
                        //<!-- Active: "bg-gray-100", Not Active: "" -->
                        <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-0">"Your Profile"</a>
                        <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-1">"Settings"</a>
                        <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-2">"Sign out"</a>
                      </div>
                    </div>*/
                  </div>
                </div>
              </div>

              //<!-- Mobile menu, show/hide based on menu state. -->
              <div class="sm:hidden" id="mobile-menu">
                <div class={move || mobile_menu_class.get()}>
                  //<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
                    //<span class="bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium" aria-current="page">
                    <a href="/" class={move || home_mobile_class.get()} on:click=handle_active.clone()>
                        "Home"
                    </a>

                    <a href="/pages/quiz" class={move || quiz_mobile_class.get()} on:click=handle_active.clone()>
                        "Quiz"
                    </a>

      //            <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium">"Projects"</a>

     //             <a href="#" class="text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium">"Calendar"</a>
                </div>
              </div>
            </nav>

        }
}
