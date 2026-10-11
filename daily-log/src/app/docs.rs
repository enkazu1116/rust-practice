pub mod install;

use topcoat::{
    Result,
    router::{layout, page},
    view::view,
};

#[layout]
async fn docs_layout(slot: Result) -> Result {
    view! {
        <section>
            <p>"docs layout"</p>
            (slot?)
        </section>
    }
}

#[page]
pub async fn page() -> Result {
    view! {
        <h1>"docs"</h1>
        <p>"src/app/docs.rs -> /docs"</p>
    }
}
