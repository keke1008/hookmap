use hookmap_core::Button;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ButtonSet {
    Single(Button),
    Any(Vec<Button>),
    All(Vec<Button>),
}

impl From<Button> for ButtonSet {
    fn from(button: Button) -> Self {
        Self::Single(button)
    }
}

impl From<&ButtonSet> for ButtonSet {
    fn from(button_set: &ButtonSet) -> Self {
        button_set.clone()
    }
}
