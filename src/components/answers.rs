use crate::components::quizbox::question_w_amountand_cat::QuestionWAmountandCatQuestionsByAmountAndCategoryId;
use gloo;
use gloo_console::log;
use html_escape::*;
use html_escape::*;
use leptos::{
    web_sys::{Event, EventTarget, HtmlInputElement, HtmlMapElement, MouseEvent, SubmitEvent},
    *,
};
use uuid::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    id: Uuid,
    answer: String,
}

#[component]
pub fn Answers(
    cx: Scope,
    questions: ReadSignal<Vec<QuestionWAmountandCatQuestionsByAmountAndCategoryId>>,
    current_question: ReadSignal<usize>,
    answers: ReadSignal<Vec<Vec<String>>>,
    disabled: RwSignal<bool>,
    score: RwSignal<i32>,
    correct: RwSignal<String>,
) -> impl IntoView {
    let (current_answers, set_current_answers) = create_signal(cx, vec![]);

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    let answer_handler = move |event: MouseEvent| {
        gloo::console::log!("Hello answers");

        let target: EventTarget = event
            .target()
            .expect("Event should have a target when dispatched");
        event.prevent_default();

        let answer = target.unchecked_into::<HtmlInputElement>().value();

        if answer
            == questions.get()[current_question.get()]
                .correct_answer
                .clone()
        {
            score.update(|score: &mut i32| *score += 1);
            correct.set("correct".to_string());

            gloo::console::log!("correct");
        } else {
            correct.set("incorrect".to_string());

            gloo::console::log!("incorrect");
        }

        gloo::console::log!(format!("answer: {}", answer));
        gloo::console::log!(correct.get());
        gloo::console::log!(format!("score: {}", score.get()));
        disabled.set(true);
    };

    create_effect(cx, move |_| {
        let mut answerss = vec![];

        for i in 0..answers.get()[current_question.get()].len() {
            answerss.push(Answer {
                id: Uuid::new_v4(),
                answer: answers.get()[current_question.get()][i].clone(),
            });
        }
        set_current_answers(answerss);
    });

    view! {cx,


            <For
                // a function that returns the items we're iterating over; a signal is fine
                each=(move || current_answers)()
                // a unique key for each item
                key=|i| i.id
                // renders each item to a view
                view=move |answer_string: Answer| {
                  view! {
                    cx,
                    <button value={answer_string.answer.clone()} disabled={move || disabled.get()} class="bg-blue-500 m-0.5 w-full hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=answer_handler>{move || html_escape::decode_html_entities(string_to_static_str(answer_string.answer.clone()))}</button>
                  }
                }
            />

    }
}
