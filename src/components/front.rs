use crate::categories::CategoriesAllCategories;
use gloo;
use gloo_console::log;
use gloo_net::http::Request;
use graphql_client::{GraphQLQuery, Response};
use leptos::{web_sys::HtmlMapElement, *};
use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/opentdb_schema.json",
    query_path = "graphql/categories.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Categories;

#[component]
pub fn Front(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    //let (count, set_count) = create_signal(cx, 0);
    //let on_click = move |_| set_count.update(|count| *count += 1);
    let (category, set_category) = create_signal(cx, vec![]);

    //let category_value = category.into_iter();

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
            })
        })
    }
    view! { cx,
        <form class="w-full max-w-sm" /*onsubmit={start_handler}*/>
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="num-of-questions">
            "No. of Questions"
            </label>
          </div>
          <div class="md:w-2/3">
            <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="num-of-questions" type="number" /*value={numberQuestionsValue}*/ min={5} max={30} /*onchange={number_handler}*/ />
          </div>
        </div>
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="categories">
            "Quiz Category"
            </label>
          </div>
          <div class="md:w-2/3">
            <select id="categories" class="block w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline" /*onchange={category_handler}*/>
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
            <select id="difficulty" class="block w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline" /*onchange={difficulty_handler}*/ >
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
            <select id="quiztype" class="block w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline" /*onchange={type_handler}*/ >
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
      </form>
    }
}
