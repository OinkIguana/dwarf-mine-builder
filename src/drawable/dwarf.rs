use std::any::Any;
use game_engine::prelude::*;
use crate::model::DwarfDescriptor;

#[derive(Debug)]
pub struct DwarfDrawable {
    pub dwarf: DwarfDescriptor,
    pub position: Point,
    pub depth: i32,
    pub visible: bool,
}

impl DwarfDrawable {
    pub fn boxed(dwarf: DwarfDescriptor) -> Box<dyn Drawable> {
        Box::new(DwarfDrawable {
            dwarf,
            position: Point::default(),
            depth: 0,
            visible: true,
        })
    }
}

impl Drawable for DwarfDrawable {
    fn depth(&self) -> i32 {
        self.depth
    }

    fn render(&self, canvas: &mut dyn Canvas) -> game_engine::Result<()> {
        if self.visible {
            if let Some(sprite) = self.dwarf.sprite() {
                canvas.draw_sprite(self.position, 0, sprite)?;
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
