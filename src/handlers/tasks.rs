use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::models::{CreateTask, Task, UpdateTask};
use crate::state::AppState;

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let mut task = state.tasks
}
 