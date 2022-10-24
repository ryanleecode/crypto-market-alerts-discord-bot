use super::{super::db::alert::ActiveModel as AlertModel, models::Alert};
use anyhow::Context;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

#[derive(Debug)]
struct InternalError;

impl warp::reject::Reject for InternalError {}

pub async fn create_alert(
    alert: Alert,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_alert = AlertModel {
        id: ActiveValue::NotSet,
        ticker: ActiveValue::Set(alert.ticker.clone()),
        signal: ActiveValue::Set(alert.signal.clone()),
        category: ActiveValue::Set(alert.category.clone()),
        timestamp: ActiveValue::Set(alert.timestamp.naive_utc()),
        interval: ActiveValue::Set(alert.interval.clone()),
    };

    let insert_result = db_alert
        .insert(&db)
        .await
        .with_context(|| "failed to insert alert into db");
    if let Err(e) = &insert_result {
        tracing::error!("{}", e);
    };

    insert_result.map_err(|_| warp::reject::custom(InternalError))?;

    Ok(warp::http::StatusCode::ACCEPTED)
}
