use topcaot::{
    Result,
    router::{layout, page},
    view::view
};

// ルーター定義
pub fn router() -> topcoat::router::Router {
    topcaot::router::module_router!().build{}
}