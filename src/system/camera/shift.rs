use game_engine::{system, prelude::*};
use crate::resource::VisibleRange;
use crate::model::Point3D;

#[derive(Default, Debug)]
pub struct ShiftCamera;

system! {
    impl ShiftCamera {
        fn run(
            &mut self,
            keyboard_events: &Resource<KeyboardEvents>,
            visible_range: &mut Resource<VisibleRange>,
        ) {
            for event in keyboard_events.iter() {
                match event {
                    KeyboardEvent::Press(Key::Q) => visible_range.shift(Point3D::y(-1)),
                    KeyboardEvent::Press(Key::W) => visible_range.shift(Point3D::z(-1)),
                    KeyboardEvent::Press(Key::E) => visible_range.shift(Point3D::x(1)),
                    KeyboardEvent::Press(Key::A) => visible_range.shift(Point3D::y(1)),
                    KeyboardEvent::Press(Key::S) => visible_range.shift(Point3D::z(1)),
                    KeyboardEvent::Press(Key::D) => visible_range.shift(Point3D::x(-1)),
                    _ => {},
                }
            }
        }
    }
}
