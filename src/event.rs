pub trait EventLoop {
    fn start(&self);
    fn stop(&self);
}