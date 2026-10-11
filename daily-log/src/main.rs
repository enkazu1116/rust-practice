mod app;

#[tokio::main]
asyncfn main() {
    topcaot::start(app:router()).await.unwrap();
}
