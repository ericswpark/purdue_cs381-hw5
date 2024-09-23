use crate::structs::*;
use cs381_hw4::*;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

fn do_question_two(b: Vec<u32>) -> Result<u32, ()> {
    let result = valid_tours(&b);
    Ok(result)
}
pub async fn question_two(Json(payload): Json<QuestionTwo>) -> impl IntoResponse {
    match do_question_two(payload.b) {
        Ok(result) => (StatusCode::OK, Json(QuestionTwoAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

fn do_question_three(c: Vec<u32>) -> Result<u32, ()> {
    let result = sand_dunes_merging(&c);
    Ok(result)
}
pub async fn question_three(Json(payload): Json<QuestionThree>) -> impl IntoResponse {
    match do_question_three(payload.c) {
        Ok(result) => {
            (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response()
        }
        Err(e) => e.into_response(),
    }
}
