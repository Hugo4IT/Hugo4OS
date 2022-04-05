/// Wrapper to add trait implementation support to spin::Mutex.
pub struct Locked<A>(spin::Mutex<A>);
impl<A> Locked<A> {
    pub const fn new(inner: A) -> Locked<A> { Locked(spin::Mutex::new(inner)) }
    pub fn lock(&self) -> spin::MutexGuard<A> { self.0.lock() }
}