use once_cell::sync::OnceCell;
use tokio::sync::mpsc::Sender;

#[derive(Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct StepPayload {
    pub row: usize,
    pub col: usize,
    pub value: u8,
    pub grid: [[u8; 9]; 9],
}


#[allow(dead_code)]
pub static STEP_SENDER: OnceCell<Sender<StepPayload>> = OnceCell::new();