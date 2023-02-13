// #[derive(Copy, Clone)]
// pub struct Component;
//
// impl Component {
//     pub fn initialize(&self) {}
//     pub fn start(&self) {}
//     pub fn update(&self) {}
//     pub fn late_update(&self) {}
//     pub fn close(&self) {}
//     pub fn dispose(&self) {}
// }

pub trait Component {
    fn initialize(&self) {}
    fn start(&self) {}
    fn update(&self, ellapsed_time: f32) {}
    fn late_update(&self) {}
    fn close(&self) {}
    fn dispose(&self) {}
}
