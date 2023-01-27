use crate::components::quizbox::question_w_amountand_cat::QuestionWAmountandCatQuestionsByAmountAndCategoryId;
use gloo;
use leptos::{
    web_sys::{EventTarget, HtmlInputElement, MouseEvent},
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
    current_question: RwSignal<usize>,
    answers: ReadSignal<Vec<Vec<String>>>,
    disabled: RwSignal<bool>,
    score: RwSignal<i32>,
    correct: RwSignal<String>,
) -> impl IntoView {
    let (current_answers, set_current_answers) = create_signal(cx, vec![]);
    let (_selected_answer, set_selected_answer) = create_signal(cx, "".to_string());

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
        } else {
            correct.set("incorrect".to_string());
        }

        set_selected_answer(answer.to_string());

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

                    /*{(move || {if correct.get() == "incorrect" && answer_string.answer.clone() == questions.get()[current_question.get()].correct_answer.clone() {
                        view!{cx, <button disabled={move || disabled.get()} class="bg-green-500 m-0.5 w-full text-white font-bold py-2 px-4 rounded" value={answer_string.answer.clone()} >
                            {move || html_escape::decode_html_entities(string_to_static_str(answer_string.answer.clone()))}
                        </button>}
                    }
                    else if correct.get() == "incorrect" && answer_string.answer.clone() == selected_answer.get() {
                        view!{cx, <button disabled={move || disabled.get()} class="bg-red-500 m-0.5 w-full text-white font-bold py-2 px-4 rounded" value={answer_string.answer.clone()} >
                            {move || html_escape::decode_html_entities(string_to_static_str(answer_string.answer.clone()))}
                        </button>}
                    }
                    else if correct.get() == "correct" && answer_string.answer.clone() == selected_answer.get() {
                        view!{cx, <button disabled={move || disabled.get()} class="bg-green-500 m-0.5 w-full text-white font-bold py-2 px-4 rounded" value={answer_string.answer.clone()} >
                            {move || html_escape::decode_html_entities(string_to_static_str(answer_string.answer.clone()))}
                        </button>}
                    }
                    else if disabled.get() {
                        view!{cx, <button disabled={move || disabled.get()} class="bg-blue-500 m-0.5 w-full text-white font-bold py-2 px-4 rounded" value={answer_string.answer.clone()} >
                            {move || html_escape::decode_html_entities(string_to_static_str(answer_string.answer.clone()))}
                        </button>}
                    } else {
                        view!{cx, <button disabled={move || disabled.get()} class="bg-blue-500 m-0.5 w-full hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" value={answer_string.answer.clone()} on:click=answer_handler >
                            {move || html_escape::decode_html_entities(string_to_static_str(answer_string.answer.clone()))}
                        </button>}
                    }})()}*/
                  }
                }
            />

    }
}
