use crate::components::front::Cat;
use crate::components::quizbox::question_w_amountand_cat::QuestionWAmountandCatQuestionsByAmountAndCategoryId;
use gloo;
use gloo_console::log;
use gloo_net::http::Request;
use graphql_client::{GraphQLQuery, Response};
use html_escape::*;
use leptos::{
    web_sys::{Event, EventTarget, HtmlInputElement, HtmlMapElement, MouseEvent, SubmitEvent},
    *,
};
use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/opentdb_schema.json",
    query_path = "graphql/questionswamountandcat.graphql",
    response_derives = "Debug, Clone"
)]
pub struct QuestionWAmountandCat;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Question {
    category: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[component]
pub fn Quizbox(
    cx: Scope,
    category: Cat,
    number: String,
    difficulty: String,
    quiz_type: String,
) -> impl IntoView {
    let (questions, set_questions) = create_signal(
        cx,
        vec![QuestionWAmountandCatQuestionsByAmountAndCategoryId {
            category: "".to_string(),
            difficulty: "".to_string(),
            question: "".to_string(),
            correct_answer: "".to_string(),
            incorrect_answers: vec!["".to_string()],
        }],
    );
    let (question_count, set_question_count) = create_signal(cx, 0);
    let (current_question, set_current_question) = create_signal(cx, 0);

    let next_handler = move |e: MouseEvent| {
        e.prevent_default();
        set_current_question(current_question.get() + 1);
    };

    fn capitalize_first_letter(s: &str) -> String {
        s[0..1].to_uppercase() + &s[1..]
    }

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    //let binding = (questions.get()[current_question.get()].question.clone());
    //let decoded_question = html_escape::decode_html_entities(&binding);
    let difficulty_value = difficulty.clone();
    {
        //let question_number = number.clone();
        create_effect(cx, move |_| {
            let number = number.clone();
            let difficulty = difficulty.clone();
            let quiz_type = quiz_type.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let cat_id_int: i64 = category.id.try_into().unwrap();
                let question_number_int: i64 = number.to_owned().parse().unwrap();
                let request_body =
                    QuestionWAmountandCat::build_query(question_w_amountand_cat::Variables {
                        amount: question_number_int,
                        category_id: cat_id_int,
                        difficulty: difficulty,
                        quiz_type: quiz_type,
                    });
                let response_body: Response<question_w_amountand_cat::ResponseData> =
                    Request::post("/graphql")
                        .json(&request_body)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                        .json::<Response<question_w_amountand_cat::ResponseData>>()
                        .await
                        .unwrap();
                gloo::console::log!(format!("{:?}", response_body));
                let fetched_questions = response_body
                    .data
                    .unwrap()
                    .questions_by_amount_and_category_id;
                set_questions(fetched_questions.clone());
                set_question_count(fetched_questions.len());
            })
        })
    }
    view! {cx,
    <div class="bg-white max-w-lg rounded overflow-hidden shadow-lg">
        <div class="px-6 py-4">
        <h3 class="bg-gray-100 text-center py-3">
            {move || format!("Question #{} of {}", current_question.get() + 1, question_count.get())}
            <br />
            {move || format!("Category: {}", category.name.clone())}
            <br />
            {format!("Difficulty: {}", capitalize_first_letter(&difficulty_value))}
          </h3>
          /*if correct_value.clone() == "correct" {
            <div class={classes!("bg-green-100", "border-t", "border-b", "border-green-500", "text-green-700", "px-4", "py-3")} role={"alert"}>
              <p class={classes!("text-sm")}>{ "That answer is Correct!" }</p>
            </div>
          } else if correct_value.clone() == "incorrect" {
            <div class={classes!("bg-red-100", "border-t", "border-b", "border-red-500", "text-red-700", "px-4", "py-3")} role={"alert"}>
              <p class={classes!("text-sm")}>{ "That answer is Incorrect!" }</p>
            </div>
          }*/
          <div class="font-bold text-xl mb-2 text-center py-4">
            {move || html_escape::decode_html_entities(string_to_static_str(questions.get()[current_question.get()].question.clone()))}
          </div>
          /*<Answers
            incorrect_answers={questions[current_question_value].incorrect_answers.clone()}
            correct_answer={questions[current_question_value].correct_answer.clone()}
            disabled={disabled_value}
            set_disabled={disabled_callback_comp.clone()}
            set_score={score_callback.clone()}
            score={score_value}
            set_correct={correct_callback_comp.clone()}
            correct={correct_value}
          />*/
        </div>

          //if disabled_value {
        <div class="px-6 pt-4 pb-2 text-center">
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=next_handler>
               "Next"
              </button>
            </div>
          //}

      </div>
    }
}
