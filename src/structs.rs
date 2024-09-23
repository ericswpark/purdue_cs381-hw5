use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QuestionTwo {
    pub(crate) b: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionTwoAnswer {
    pub(crate) answer: u32,
}

#[derive(Deserialize)]
pub struct QuestionThree {
    pub(crate) c: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionThreeAnswer {
    pub(crate) answer: u32,
}
