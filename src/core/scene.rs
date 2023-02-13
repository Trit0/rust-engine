use crate::core::game_object::GameObject;


pub struct Scene<'a> {
    pub game_objects: Vec<Box<GameObject<'a>>>
}

impl<'a> Scene<'a> {
    pub fn initialize(&self) {
        for game_object in &self.game_objects {
            game_object.initialize();
        }
    }
    pub fn start(&self) {
        for game_object in &self.game_objects {
            game_object.start();
        }
    }
    pub fn update(&self, ellapsed_time: f32) {
        for game_object in &self.game_objects {
            game_object.update(ellapsed_time);
        }
    }
    pub fn late_update(&self) {
        for game_object in &self.game_objects {
            game_object.late_update();
        }
    }
    pub fn close(&self) {
        for game_object in &self.game_objects {
            game_object.close();
        }
    }
    pub fn dispose(&self) {
        for game_object in &self.game_objects {
            game_object.dispose();
        }
    }
}