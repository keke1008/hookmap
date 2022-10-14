use std::sync::Arc;

use hookmap_core::button::Button;

use crate::layer::LayerIndex;

use super::layer::Layer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ButtonArgs {
    Each(Vec<Button>),
    Not(Arc<Vec<Button>>),
}

impl From<Button> for ButtonArgs {
    fn from(button: Button) -> Self {
        ButtonArgs::Each(vec![button])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModifierArgType {
    Layer(LayerIndex),
    Button(Button),
}
impl From<LayerIndex> for ModifierArgType {
    fn from(index: LayerIndex) -> Self {
        ModifierArgType::Layer(index)
    }
}
impl From<&Layer> for ModifierArgType {
    fn from(layer: &Layer) -> Self {
        ModifierArgType::Layer(layer.index())
    }
}
impl From<Button> for ModifierArgType {
    fn from(button: Button) -> Self {
        ModifierArgType::Button(button)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModifierArg {
    pub arg: ModifierArgType,
    pub invert: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModifierArgs {
    pub args: Vec<ModifierArg>,
}

impl From<Button> for ModifierArgs {
    fn from(button: Button) -> Self {
        let arg = ModifierArg {
            arg: button.into(),
            invert: false,
        };
        Self { args: vec![arg] }
    }
}

impl From<&Layer> for ModifierArgs {
    fn from(layer: &Layer) -> Self {
        let arg = ModifierArg {
            arg: layer.into(),
            invert: false,
        };
        Self { args: vec![arg] }
    }
}

impl ModifierArgs {
    pub(crate) fn iter_buttons(&self) -> impl Iterator<Item = (Button, bool)> + '_ {
        self.args.iter().filter_map(|arg| match arg.arg {
            ModifierArgType::Button(button) => Some((button, arg.invert)),
            ModifierArgType::Layer(_) => None,
        })
    }

    pub(crate) fn iter_layers(&self) -> impl Iterator<Item = (LayerIndex, bool)> + '_ {
        self.args.iter().filter_map(|arg| match arg.arg {
            ModifierArgType::Layer(layer) => Some((layer, arg.invert)),
            ModifierArgType::Button(_) => None,
        })
    }
}
