use crate::util::vector::Vector2;

#[derive(Clone)]
pub enum UiAction {
    AddUiText(String, Vector2, Vector2),
}