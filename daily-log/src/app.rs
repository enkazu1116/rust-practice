pub mod shared;
pub mod user;

use topcoat::{
    Result,
    router::{layout, page},
    view::view
};

// ルーター定義
pub fn router() -> topcoat::router::Router {
    topcoat::router::module_router!().build()
}