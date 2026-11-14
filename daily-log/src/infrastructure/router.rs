use axum::{Router, routing::get};

// applicationのルーティング管理
let app = Router::new()
    .route("/", get(root))
    // .route("/login", get(login))
    // .route("/sign-up", get(sign_up))
    // .route("/logout")
    .route("/api/v1.0.0/user/create", post(create_user));

async fn root() {}