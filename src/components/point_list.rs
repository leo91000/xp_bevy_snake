use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Debug, Deserialize)]
pub struct PointList(pub Vec<Vec2>);

impl PointList {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    #[allow(dead_code)]
    pub(crate) fn push(&mut self, point: Vec2) {
        self.0.push(point)
    }
}
