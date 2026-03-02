use crate::api::quiz::quiz_get;
use crate::Route;
use dioxus::prelude::*;
use dioxus_html::FormEvent;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct QuestionForm {
    answer: String,
}
const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.

#[component]
pub fn Quiz(id: i32, quest: i32) -> Element {
    let quiz = match use_resource(move || async move { quiz_get(id as u32).await })
        .read_unchecked()
        .clone()
    {
        Some(Ok(quiz)) => quiz,
        Some(Err(error)) => return rsx! { "{error}" },
        None => return rsx! { "loading..." },
    };

    let mut done = use_signal(|| false);
    let mut score = use_signal(|| 0);

    let Some(question) = quiz.questions.get(quest as usize) else {
        return rsx! {
            p { "question not found :/" }
        };
    };

    let q2 = question.clone();

    let mut response = use_signal(|| None);
    let answers = quiz.questions[quest as usize].answers.clone().into_iter().enumerate().map(|(index, answer)| {
       rsx!{
        div { class:"",
            label {
                class: "has-checked:bg-indigo-50  flex items-center justify-between gap-4 rounded border border-gray-300 bg-white p-3 text-sm font-medium shadow-sm transition-colors hover:bg-gray-50 has-checked:border-blue-600 has-checked:ring-1 has-checked:ring-blue-600",
                r#for: "{index}",
                p { class: "text-gray-700", "{answer}" }

                input {
                       required: true,
                    checked: index == 0,
                    class: "",
                    id: "{index}",
                    name: "answer",
                    r#type: "radio",
                    value: "{index}",
                }
            }
        }

    }
    });

    if done() {
        return rsx! {
            QuizEnd{score: score(), out_of: quiz.questions.len()}
        };
    }

    rsx! {

        div {

            p { class: "text-2xl", "{quiz.name}" }

            p { "{question.q}" }

            form {

                onsubmit: move |evt: FormEvent| async move {
                    // Prevent the browser from navigating away.
                    evt.prevent_default();


                    match evt.parsed_values::<QuestionForm>() {
                        Ok(QuestionForm {answer}) => {
                           if answer == q2.correct.to_string() {
                                response.set(Some("correct!"));
                                score.set(score()+1);
                            }else {
                                response.set(Some("you LOSE, good DAY!"));
                            }


                        }
                        _ => {}
                    }





                },

                {response}
                {answers}
                 if response() == None {
                button { class:"border-2 p-1 m-1 hover:bg-blue-400 hover:text-white ", r#type: "submit", "check",  }
                    }
            }
            if response() == None {

            }else
            if (quest as usize + 1) < quiz.questions.len() {

                Link {
                    onclick: move |_| {
                      response.set(None);
                    },
                   class:"border-2 p-1 m-1 hover:bg-blue-400 hover:text-white ",
                    id: "link_id",
                    new_tab: false,
                    rel: "link_rel",
                    to: Route::Quiz {
                        id,
                        quest: quest + 1,
                    },
                    {}
                    "Next"
                }
            } else {
                button {
                     onclick: move |_| {
                        done.set(true);
                      warn!("{}", score());
                    },
                 class:"border-2 p-1 m-1 hover:bg-blue-400 hover:text-white ",
                    id: "link_id",
                  {}
                    "Score"
                }
            }

        }
    }
}

#[component]
pub fn QuizEnd(score: u32, out_of: usize) -> Element {
    rsx! {
        h1{ class:" font-bold text-5xl text-blue-500 absolute top-1/3 left-1/3", "you got {score}/{out_of}!"}
         Link {

                   class:"border-2 p-1 m-1 hover:bg-blue-400 hover:text-white ",
                    id: "link_id",
                    new_tab: false,
                    rel: "link_rel",
                    to: Route::Home {

                    },
                    {}
                    "HOME"
                }
    }
}
