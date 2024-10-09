use crate::{
    model::{Data, DeleteData, ListEvents, UpdateData},
    repository::{MutateState, ReadState, Repository},
};
use anyhow::{anyhow, Result};
use axum::{
    extract::{
        rejection::{JsonRejection, QueryRejection},
        Query, State,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use error::AppError;
use middleware::print_request_response;
use serde_json::json;
use std::{net::Ipv4Addr, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

pub(crate) mod error;
mod middleware;

pub(crate) async fn start(address: (Ipv4Addr, u16)) -> Result<()> {
    let listener = TcpListener::bind(address).await?;
    let shared_state = Arc::new(Repository::new());

    let router = initialize_router(shared_state);

    let (ip, port) = address;

    info!(ip = ?ip, port = ?port, "bound ip address and port");
    axum::serve(listener, router).await?;

    Ok(())
}

fn initialize_router(shared_state: Arc<Repository>) -> Router {
    axum::Router::new()
        .route("/create_event", post(create_event))
        .route("/update_event", post(update_event))
        .route("/delete_event", post(delete_event))
        .route("/events_for_day", get(events_for_day))
        .route("/events_for_week", get(events_for_week))
        .route("/events_for_month", get(events_for_month))
        .layer(axum::middleware::from_fn(print_request_response))
        .with_state(shared_state)
}

#[axum::debug_handler]
async fn create_event(
    State(repository): State<Arc<Repository>>,
    result: Result<Json<Data>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(data) = result?;
    let Data { user_id, date, .. } = data;

    repository.create_event(data).await;

    info!(user_id = ?user_id, date = ?date, "event created");

    Ok(Json(json!({"result": "event created"})))
}

#[axum::debug_handler]
async fn update_event(
    State(repository): State<Arc<Repository>>,
    result: Result<Json<UpdateData>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(data) = result?;

    let UpdateData {
        data: Data {
            user_id,
            date,
            description,
        },
        ulid,
    } = data;

    match repository.update_event(user_id, date, description, ulid) {
        MutateState::Success => {
            info!(user_id = ?user_id, date = ?date, ulid = ?ulid, "event updated");
            Ok(Json(json!({"result": "event updated"})))
        }
        MutateState::UlidNotFound => Err(AppError::service(anyhow!(
            "user {user_id} by date {date} no event found by ulid: {ulid}",
        ))),
        MutateState::DateNotFound => Err(AppError::service(anyhow!(
            "user {user_id} doesn't have any events by date: {date}",
        ))),
        MutateState::UserNotFound => {
            Err(AppError::service(anyhow!("user {user_id} doesn't exists")))
        }
    }
}

#[axum::debug_handler]
async fn delete_event(
    State(repository): State<Arc<Repository>>,
    result: Result<Json<DeleteData>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(data) = result?;

    let DeleteData {
        user_id,
        date,
        ulid,
    } = data;

    match repository.delete_event(data.user_id, data.date, data.ulid) {
        MutateState::Success => {
            info!(user_id = ?user_id, date = ?date, ulid = ?ulid, "event deleted");
            Ok(Json(json!({"result": "event deleted"})))
        }
        MutateState::UlidNotFound => Err(AppError::service(anyhow!(
            "user {user_id} by date {date} no event found by ulid: {ulid}",
        ))),
        MutateState::DateNotFound => Err(AppError::service(anyhow!(
            "user {user_id} doesn't have any events by date: {date}",
        ))),
        MutateState::UserNotFound => {
            Err(AppError::service(anyhow!("user {user_id} doesn't exists")))
        }
    }
}

#[axum::debug_handler]
async fn events_for_day(
    State(repository): State<Arc<Repository>>,
    result: Result<Query<ListEvents>, QueryRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Query(data) = result?;

    let ListEvents { user_id, date } = data;

    let events = match repository.events_for_day(user_id, date) {
        ReadState::Success(events) => {
            if events.is_empty() {
                return Err(AppError::service(anyhow!(
                    "no events found for day {}",
                    date
                )));
            }

            events
        }
        ReadState::DateNotFound => {
            return Err(AppError::service(anyhow!(
                "no events have been created for day {} before",
                date
            )));
        }
        ReadState::UserNotFound => {
            return Err(AppError::service(anyhow!(
                "cannot find user with id {}",
                user_id
            )));
        }
    };

    info!(user_id = ?user_id, date = ?date, "received {} events for user for specified day", events.len());
    Ok(Json(json!({"result": events})))
}

#[axum::debug_handler]
async fn events_for_week(
    State(repository): State<Arc<Repository>>,
    result: Result<Query<ListEvents>, QueryRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Query(data) = result?;

    let ListEvents { user_id, date } = data;

    let events = match repository.events_for_week(user_id, date) {
        ReadState::Success(events) => {
            if events.is_empty() {
                return Err(AppError::service(anyhow!(
                    "no events found for week {}",
                    date
                )));
            }

            events
        }
        ReadState::DateNotFound => {
            return Err(AppError::service(anyhow!(
                "no events have been created for week {} before",
                date
            )));
        }
        ReadState::UserNotFound => {
            return Err(AppError::service(anyhow!(
                "cannot find user with id {}",
                user_id
            )));
        }
    };

    info!(user_id = ?user_id, date = ?date, "received {} events for user for specified week", events.len());
    Ok(Json(json!({"result": events})))
}

#[axum::debug_handler]
async fn events_for_month(
    State(repository): State<Arc<Repository>>,
    result: Result<Query<ListEvents>, QueryRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Query(data) = result?;

    let ListEvents { user_id, date } = data;

    let events = match repository.events_for_month(user_id, date) {
        ReadState::Success(events) => {
            if events.is_empty() {
                return Err(AppError::service(anyhow!(
                    "no events found for month {}",
                    date
                )));
            }

            events
        }
        ReadState::DateNotFound => {
            return Err(AppError::service(anyhow!(
                "no events have been created for month {} before",
                date
            )));
        }
        ReadState::UserNotFound => {
            return Err(AppError::service(anyhow!(
                "cannot find user with id {}",
                user_id
            )));
        }
    };

    info!(user_id = ?user_id, date = ?date, "received {} events for user for specified month", events.len());
    Ok(Json(json!({"result": events})))
}
