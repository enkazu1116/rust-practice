use topcoat::{
    Result,
    router::{
        Router,
        RouterBuilderDiscoverExt,
        layout,
        page
    },
    view::view,
};

#[tokio::main]
async fn main() -> topcoat::Result<()> {
    let router = Router::builder().discover().build();
    topcoat::start(router).await?;
    Ok(())
}

#[layout("/")]
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
                    <a href="/">"Home"</a>
                    " "
                    <a href="/about">"About"</a>
                </nav>
                (slot?)
            </body>
        </html>
    }
}

#[page("/")]
async fn home() -> Result {
    view! {
        <main>
            <h1>"Hello Topcoat"</h1>
            <p>"最初のページ"</p>
        </main>
    }
}

#[page("/about")]
async fn about() -> Result {
    view! {
        <main>
            <h1>"About"</h1>
            <p>"Topcoatを使用した検証"</p>
        </main>
    }
}