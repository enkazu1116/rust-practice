use topcoat::{
    Result,
    router::{
        Router,
        RouterBuilderDiscoverExt,
        layout,
        page,
    }
    view::view,
};

#[topcoat::main]
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
                <title>"Daily Log"</title>
                topcoat::dev::script()
            </head>

            <body>
                <nav>
                    <a href="/">"Home"</a>
                </nav>
                (slot?)
            </body>
    }
}

#[page("/")]
async fn root() -> Result {
    view! {
        <main>
            <h1>"Daily Log"</h1>
            <p>"日々のエンジニアの軌跡を残し、振り返りを行いましょう！"</p>
        </main>
    }
}