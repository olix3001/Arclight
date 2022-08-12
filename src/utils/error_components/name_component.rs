use crate::utils::error::ErrorComponent;
use colored::*;

pub struct NameErrorComponent {
    text: String
}

impl ErrorComponent for NameErrorComponent {
    fn to_err_string(&self) -> String {
        return self.text.clone();
    }
}

impl NameErrorComponent {
    pub fn new(text: String) -> NameErrorComponent {
        NameErrorComponent { text }
    }
}
