use std::borrow::Borrow;
use glium::glutin;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use crate::core::scene::Scene;
use crate::core::timer::Timer;

pub struct Engine<'a> {
    pub scenes: Vec<Scene<'a>>,
    pub timer: Timer
}


impl<'a> Engine<'a> {
    // start
    pub fn start(scene: Scene) {
        let engine = Engine {
            scenes: vec![scene],
            timer: Timer::new()
        };
        engine.run();
    }

    fn run(self) {
    }


    // close
}