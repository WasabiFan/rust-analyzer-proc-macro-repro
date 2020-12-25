use mockall::mock;

trait Bar {
    fn do_stuff(&mut self);
}

mock! {
    pub BarImpl { }

    impl Bar for BarImpl {
        fn do_stuff(&mut self);
    }
}
