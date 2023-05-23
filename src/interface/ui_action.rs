use crate::interface::ui_error::UiError;
use crate::util::vector::Vector2;

#[derive(Clone)]
pub enum UiAction {
    AddUiText(String, Vector2, Vector2),
    WriteError(UiError),
}