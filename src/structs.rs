use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QuestionTwo {
    pub(crate) b: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionTwoAnswer {
    pub(crate) answer: u32,
}
