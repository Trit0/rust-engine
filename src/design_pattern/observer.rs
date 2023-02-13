pub struct Observer;

impl Observer {
    fn notification() {}
}


pub struct Observable<'a> {
    observers: Vec<&'a Observer>
}

impl<'a> Observable<'a> {
    fn add_observer(&mut self, observer: &'a Observer) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: &'a Observer) {

    }
}