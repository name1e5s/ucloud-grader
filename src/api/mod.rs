use once_cell::sync::OnceCell;

pub mod list;
pub mod paper;
pub mod submit;

static EXAM_ID: OnceCell<String> = OnceCell::new();
static QUESTION_ID: OnceCell<String> = OnceCell::new();

pub fn set_exam_id(exam_id: String) {
    let _ = EXAM_ID.set(exam_id);
}

pub fn get_exam_id() -> String {
    EXAM_ID.get().map(|s| s.to_string()).unwrap_or_default()
}

pub fn set_question_id(question_id: String) {
    let _ = QUESTION_ID.set(question_id);
}

pub fn get_question_id() -> String {
    QUESTION_ID.get().map(|s| s.to_string()).unwrap_or_default()
}
