use crate::components::answers::*;
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
use rand::seq::SliceRandom;
use rand::thread_rng;
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
    let (answers, set_answers) = create_signal(cx, vec![vec![]]);
    let disabled = create_rw_signal(cx, false);
    let score = create_rw_signal(cx, 0);
    let correct = create_rw_signal(cx, "".to_string());

    let next_handler = move |e: MouseEvent| {
        e.prevent_default();
        set_current_question(current_question.get() + 1);
        disabled.set(false);
    };

    fn capitalize_first_letter(s: &str) -> String {
        s[0..1].to_uppercase() + &s[1..]
    }

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    let difficulty_value = difficulty.clone();
    {
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
                let fetched_questions = response_body
                    .data
                    .unwrap()
                    .questions_by_amount_and_category_id;
                set_questions(fetched_questions.clone());
                set_question_count(fetched_questions.len());
                let mut answerss = vec![];
                let mut answersss = vec![];
                for j in 0..fetched_questions.len() {
                    answerss.push(fetched_questions[j].correct_answer.clone());
                    for i in 0..fetched_questions[j].incorrect_answers.len() {
                        answerss.push(fetched_questions[j].incorrect_answers[i].clone());
                    }
                    answerss.shuffle(&mut thread_rng());
                    answersss.push(answerss.clone());
                    answerss.clear();
                }
                set_answers(answersss);
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
          {move || if correct.get() == "correct" {
            view!{cx,
                <>
                <div class="bg-green-100 border-t border-b border-green-500 text-green-700 px-4 py-3" role={"alert"}>
                  <p class="text-sm">"That answer is Correct!"</p>
                </div>
                </>
            }
          } else if correct.get() == "incorrect" {
            view!{cx,
                <>
                <div class="bg-red-100 border-t border-b border-red-500 text-red-700 px-4 py-3" role={"alert"}>
                    <p class="text-sm"> "That answer is Incorrect!" </p>
                </div>
                </>
            }
          } else {
              view!{cx, <></>}
          }}
          <div class="font-bold text-xl mb-2 text-center py-4">
            {move || html_escape::decode_html_entities(string_to_static_str(questions.get()[current_question.get()].question.clone()))}
          </div>
          <Answers
            answers=(move || answers.clone())()
            questions=(move || questions.clone())()
            current_question=(move || current_question.clone())()
            disabled=disabled.clone()
            score=score.clone()
            correct=correct.clone()
          />
        </div>

        {move || if disabled.get() {
            view!{cx,
                <>
                <div class="px-6 pt-4 pb-2 text-center">
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=next_handler>
                   "Next"
                    </button>
                </div>
                </>
            }
        } else {view!{cx, <></>}}}

      </div>
    }
}
