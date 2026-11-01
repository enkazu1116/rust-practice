mod app;
mod infrastructure;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
