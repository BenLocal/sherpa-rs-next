pub struct Vad<T> {
    options: T,
}

impl<T> Vad<T> {
    pub fn new(options: T) -> Self {
        Self { options }
    }
}
