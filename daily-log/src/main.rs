mod app;

#[tokio::main]
async fn main() {
    topcaot::start(app::router()).await.unwrap();
}
