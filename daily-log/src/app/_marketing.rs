pub mod pricing;

use topcoat::{Result, router::layout, view::view};

#[layout]
async fn marketing_layout(slot: Result) -> Result {
    view! {
        <section>
            <p>"marketing group layout"</p>
            (slot?)
        </section>
    }
}
