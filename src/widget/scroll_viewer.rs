use crate::enums::ParentType;
use crate::layout_object::ScrollLayoutObject;
use crate::widget::{Template, Widget};
use crate::properties::Offset;

/// The `ScrollViewer` represents a layout widget that adds vertial and horizontal offset to its perent. 
/// It is used to scroll the content if the content's width or height is greater than the ScrollViewers width or height.
/// 
/// # Properties
/// 
/// * `Offset` - Represents the vertial and horizontal scroll offset.
/// 
/// # Others
/// 
/// * `ParentType`- Single.
/// * `ScrollLayoutObject` - Used to layout the widget.
pub struct ScrollViewer;

impl Widget for ScrollViewer {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Offset::default())
            .with_layout_object(ScrollLayoutObject::default())
            .with_debug_name("ScrollViewer")
    }
}
