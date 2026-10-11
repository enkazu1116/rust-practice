use topcoat::{
    Result,
    router::{layout, page},
    view::view,
};

mod _marketing;
mod api;
mod docs;

pub fn router() -> topcoat::router::Router {
    topcoat::router::module_router!().build()
}

#[layout]
async fn root_layout(slot: Result) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"Hello Topcoat"</title>
                topcoat::dev::script()
            </head>
            <body>
                <nav>
                    <a href="/">"home"</a>
                    " | "
                    <a href="/about">"about"</a>
                    " | "
                    <a href="/docs">"docs"</a>
                    " | "
                    <a href="/docs/install">"install"</a>
                    " | "
                    <a href="/pricing">"pricing"</a>
                </nav>
                (slot?)
            </body>
        </html>
    }
}

#[page]
async fn home() -> Result {
    view! {
        <main>
            <h1>"Hello Topcoat"</h1>
            <p>"src/app.rs -> /"</p>
        </main>
    }
}

mod about {
    use topcoat::{Result, router::page, view::view};

    #[page]
    async fn page() -> Result {
        view! {
            <h1>"about"</h1>
            <p>"src/app.rs -> /about"</p>
        }
    }
}
