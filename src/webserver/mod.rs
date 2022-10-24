use sea_orm::DatabaseConnection;
use warp::Filter;

mod handlers;
mod models;

pub fn setup_routes(
    db: DatabaseConnection,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    alerts_post(db).with(warp::trace::request())
}

fn alerts_post(
    db: DatabaseConnection,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("alerts")
        .and(warp::post())
        .and(warp::body::content_length_limit(u64::MAX))
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_alert)
}

fn with_db(
    db: DatabaseConnection,
) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
