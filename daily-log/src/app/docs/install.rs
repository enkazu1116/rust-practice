use topcoat::{Result, router::page, view::view};

#[page]
pub async fn page() -> Result {
    view! {
        <h1>"install"</h1>
        <p>"src/app/docs/install.rs -> /docs/install"</p>
    }
}
