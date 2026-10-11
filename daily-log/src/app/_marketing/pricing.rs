use topcoat::{Result, router::page, view::view};

// `_marketing` は URL に出ないので、このページは /pricing。
#[page]
pub async fn page() -> Result {
    view! {
        <h1>"pricing"</h1>
        <p>"src/app/_marketing/pricing.rs -> /pricing"</p>
    }
}
