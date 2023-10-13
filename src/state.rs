#[derive(PartialEq, Clone)]
pub struct StyewardState {
    pub current_schema: Option<String>,
    pub current_table: Option<String>,
}
