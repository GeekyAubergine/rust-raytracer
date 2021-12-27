use std::sync::Arc;

use crate::geom::shapes::shape::Shape;

type SyncedShaped = dyn Shape + Send + Sync;

type ArcShape = Arc<SyncedShaped>;

#[derive(Clone)]
pub struct Scene {
    pub shapes: Vec<ArcShape>,
}

impl Scene {
    pub fn new() -> Scene {
        return Scene { shapes: Vec::new() };
    }
    pub fn add_shape(&mut self, shape: ArcShape) {
        self.shapes.push(shape)
    }
}
