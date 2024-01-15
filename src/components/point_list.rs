use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PointList(pub Vec<Vec2>);

impl PointList {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn push(&mut self, point: Vec2) {
        self.0.push(point)
    }
}
