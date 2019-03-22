use crate::properties::Selected;
use crate::theme::Selector;
use crate::widget::{Button, Template, Widget};

/// The `ToggleButton` widget can be switch its selected state. It derives from `Button` widget.
///
/// # Properties
///
/// * `Selected` - Bool value represents the selected state of the widget.
/// * `Selector` - CSS selector with  element name `togglebutton`, used to request the theme of the widget.
pub struct ToggleButton;

impl Widget for ToggleButton {
    fn create() -> Template {
        Button::create()
            .with_property(Selected(false))
            .with_property(Selector::from("togglebutton"))
            .with_debug_name("ToggleButton")
    }
}
