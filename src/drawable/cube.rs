use std::any::Any;
use game_engine::prelude::*;
use crate::model::CubeDescriptor;

#[derive(Debug)]
pub struct CubeDrawable {
    pub cube: CubeDescriptor,
    pub position: Point,
    pub depth: i32,
    pub visible: bool,
    pub hovered: bool,
}

impl CubeDrawable {
    pub fn boxed(cube: CubeDescriptor) -> Box<dyn Drawable> {
        Box::new(CubeDrawable {
            cube,
            position: Point::default(),
            depth: 0,
            visible: true,
            hovered: false,
        })
    }
}

impl Drawable for CubeDrawable {
    fn depth(&self) -> i32 {
        self.depth
    }

    fn render(&self, canvas: &mut dyn Canvas) -> game_engine::Result<()> {
        if self.visible {
            if let Some(sprite) = self.cube.sprite() {
                canvas.draw_sprite(self.position, self.cube.frame(), sprite)?;
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
