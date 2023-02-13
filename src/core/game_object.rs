use crate::core::transform::Transform;
use super::component::Component;

pub struct GameObject<'a> {
    pub transform: Transform,
    pub components: Vec<Box<dyn Component + 'a>>
}

impl<'a> GameObject<'a> {
    pub fn initialize(&self) {
        for component in &self.components {
            component.initialize();
        }
    }
    pub fn start(&self) {
        for component in &self.components {
            component.start();
        }
    }
    pub fn update(&self, ellapsed_time: f32) {
        for component in &self.components {
            component.update(ellapsed_time);
        }
    }
    pub fn late_update(&self) {
        for component in &self.components {
            component.late_update();
        }
    }
    pub fn close(&self) {
        for component in &self.components {
            component.close();
        }
    }
    pub fn dispose(&self) {
        for component in &self.components {
            component.dispose();
        }
    }
}