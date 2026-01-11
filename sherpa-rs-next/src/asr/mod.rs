use serde::{Deserialize, Serialize};

pub mod offline;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecognizerJsonResult {
    pub text: String,
    pub tokens: Vec<String>,
    pub timestamps: Vec<f32>,
    pub ys_probs: Vec<f32>,
    pub lm_probs: Vec<f32>,
    pub context_scores: Vec<f32>,
    pub segment: i32,
    pub words: Vec<Word>,
    pub start_time: f32,
    pub is_final: bool,
    pub is_eof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Word {
    pub word: String,
    pub start: f32,
    pub end: f32,
}
