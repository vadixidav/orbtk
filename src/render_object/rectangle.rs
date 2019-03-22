use crate::backend::Renderer;
use crate::render_object::RenderObject;
use crate::properties::{Point, Rect};
use crate::theme::{Selector, Theme};
use crate::widget::WidgetContainer;

pub struct RectangleRenderObject;

impl Into<Box<RenderObject>> for RectangleRenderObject {
    fn into(self) -> Box<RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        global_position: &Point,
    ) {
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                if let Ok(parent_bounds) = widget.borrow_parent_property::<Rect>() {
                    renderer.render_rectangle(
                        bounds,
                        parent_bounds,
                        global_position,
                        theme.uint("border-radius", selector),
                        theme.color("background", selector),
                        theme.uint("border-width", selector),
                        theme.color("border-color", selector),
                        theme.float("opacity", selector),
                    );
                }
            }
        }
    }
}
