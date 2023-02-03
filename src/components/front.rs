use crate::categories::CategoriesAllCategories;
use crate::components::quizbox::*;
use gloo_net::http::Request;
use graphql_client::{GraphQLQuery, Response};
use leptos::{
    web_sys::{Event, EventTarget, HtmlInputElement, SubmitEvent},
    *,
};
use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/opentdb_schema.json",
    query_path = "graphql/categories.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Categories;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Cat {
    pub id: usize,
    pub name: String,
}

#[component]
pub fn Front(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    //let (count, set_count) = create_signal(cx, 0);
    //let on_click = move |_| set_count.update(|count| *count += 1);
    let (category, set_category) = create_signal(cx, vec![]);
    let start_quiz = create_rw_signal(cx, false);
    let (category_picked, set_category_picked) = create_signal(
        cx,
        Cat {
            id: 0,
            name: "Any Category".to_string(),
        },
    );
    let (difficulty_picked, set_difficulty_picked) = create_signal(cx, "any".to_string());
    let (quiz_type_picked, set_quiz_type_picked) = create_signal(cx, "any".to_string());
    let (number_questions, set_number_questions) = create_signal(cx, "10".to_string());
    let (loading, set_loading) = create_signal(cx, true);

    let start_handler = move |e: SubmitEvent| {
        e.prevent_default();
        start_quiz.set(true);
    };

    let category_handler = move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        let cat_info = target.unchecked_into::<HtmlInputElement>().value();

        let cat_info = cat_info.split("_");

        let cat_info_vec = cat_info.collect::<Vec<&str>>();

        let cat_id = cat_info_vec[0];
        let cat_name = cat_info_vec[1];

        let value = Cat {
            id: cat_id.parse::<usize>().unwrap(),
            name: cat_name.to_string(),
        };

        set_category_picked(value);
    };

    let difficulty_handler = move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        let difficulty_info = target.unchecked_into::<HtmlInputElement>().value();
        let mut difficulty_info = difficulty_info.to_lowercase();
        if difficulty_info == "any difficulty" {
            difficulty_info = "any".to_string();
        }
        set_difficulty_picked(difficulty_info);
    };

    let number_handler = move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        e.prevent_default();

        set_number_questions(target.unchecked_into::<HtmlInputElement>().value());
    };

    let type_handler = move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        let type_info = target.unchecked_into::<HtmlInputElement>().value();

        if type_info == "Multiple Choice" {
            set_quiz_type_picked("multiple".to_string());
        } else if type_info == "True / False" {
            set_quiz_type_picked("boolean".to_string());
        } else {
            set_quiz_type_picked("any".to_string());
        }
    };

    {
        create_effect(cx, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let request_body = Categories::build_query(categories::Variables {});
                let response_body: Response<categories::ResponseData> = Request::post("/graphql")
                    .json(&request_body)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json::<Response<categories::ResponseData>>()
                    .await
                    .unwrap();
                //gloo::console::log!(format!("{:?}", response_body));
                set_category(response_body.data.unwrap().all_categories);
                set_loading(false);
            })
        })
    }

    view! {cx,
        <main class="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-green-400 to-blue-400">
        {move || if !start_quiz.get(){view!{cx,
        <div>
        {move || if loading.get() {view!{cx, <><div class="lds-ring"><div></div><div></div><div></div><div></div></div></>}}
        else {view!{cx,
        <>
        <form class="w-full max-w-sm" on:submit=start_handler>
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="num-of-questions">
            "No. of Questions"
            </label>
          </div>
          <div class="md:w-2/3">
          <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="num-of-questions" type="number" value=number_questions.get() min={5} max={30} on:change=number_handler />
          </div>
        </div>
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="categories">
            "Quiz Category"
            </label>
          </div>
          <div class="md:w-2/3">
            <select id="categories" class="block w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline" on:change=category_handler >
              <option value="0_Any Category" selected={true}>"Any Category"</option>
              <For
                // a function that returns the items we're iterating over; a signal is fine
                each=category
                // a unique key for each item
                key=|cat| cat.id
                // renders each item to a view
                view=move |cat: CategoriesAllCategories| {
                  view! {
                    cx,
                    <option key={cat.id.to_string()} value={cat.id.to_string() + "_" + &cat.name.clone()}>{ format!("{}", cat.name) }</option>
                  }
                }
              />
            </select>
          </div>
        </div>
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="difficulty">
            "Quiz Difficulty"
            </label>
          </div>
          <div class="md:w-2/3">
            <select id="difficulty" class="block w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline" on:change=difficulty_handler >
              <option selected={true}>"Any Difficulty"</option>
              <option>"Easy"</option>
              <option>"Medium"</option>
              <option>"Hard"</option>
            </select>
          </div>
        </div>
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="quiztype">
            "Quiz Question Types"
            </label>
          </div>
          <div class="md:w-2/3">
          <select id="quiztype" class="block w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline" on:change=type_handler >
              <option selected={true}>{"Any Type"}</option>
              <option>"Multiple Choice"</option>
              <option>"True / False"</option>
            </select>
          </div>
        </div>
        <div class="md:flex md:items-center">
          <div class="md:w-1/3"></div>
          <div class="md:w-2/3">
            <button class="shadow bg-purple-500 hover:bg-purple-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="submit" >
            "Start Quiz"
            </button>
          </div>
        </div>
      </form></>}}}
      </div>
     }}else{view!{cx,
         <div class="text-center">
            <Quizbox category=category_picked.get() number=number_questions.get() difficulty=difficulty_picked.get() quiz_type=quiz_type_picked.get() start_quiz=start_quiz.clone() />
        </div>}}}
        </main>
    }
}
