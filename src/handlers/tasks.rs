use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    middleware::Next,
};

use crate::models::{CreateTask, Task, UpdateTask};
use crate::state::AppState;

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let mut tasks = state.tasks.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let task = Task {
        id: *next_id,
        title: payload.title,
        done: false,
    };

    *next_id += 1;
    tasks.push(task.clone());

    (StatusCode::CREATED, Json(task))
}
