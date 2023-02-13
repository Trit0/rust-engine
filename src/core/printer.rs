use crate::core::component::Component;

pub struct Printer {
    pub(crate) name: String
}

impl Component for Printer {
    fn start(&self) {
        println!("started {}", self.name)
    }

    fn update(&self, ellapsed_time: f32) {
        println!("name is {}, ellapsed_time = {}", self.name, ellapsed_time);
    }
    fn close(&self) {
        println!("closed {}", self.name);
    }
}