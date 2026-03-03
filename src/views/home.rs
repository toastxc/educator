use crate::api::quiz::quizzes_get;
use crate::Route;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let quizzes = match use_resource(move || async move { quizzes_get().await })
        .read_unchecked()
        .clone()
    {
        Some(Ok(quizzes)) => quizzes,
        Some(Err(error)) => return rsx! { "{error}" },
        None => return rsx! { "loading..." },
    };

    let quizzes_rendered = quizzes.into_iter().map(|quiz| {
        rsx! {
            div {
                Link {
                    class: "border-2 p-1 m-1 hover:bg-blue-400 hover:text-white hover:translate-1",
                    id: "link_id",
                    new_tab: false,
                    rel: "link_rel",
                    to: Route::Quiz { id: 0, quest: 0 },
                    {}

                    "{quiz.name}"
                }
            }
        }
    });

    rsx! {

        h1 { class: "text-2xl", "quizes (select one)" }
        div { class: "flex-col", {quizzes_rendered} }

    }
}
