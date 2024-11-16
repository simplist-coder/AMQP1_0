/// From: <https://www.reddit.com/r/rust/comments/kul4qz/vec_prepend_insert_from_slice>/
pub trait VecExt<T>: AsMut<Vec<T>> {
    fn prepend(&mut self, other: &mut Vec<T>) {
        self.as_mut().splice(..0, other.drain(..));
    }
}

impl<T> VecExt<T> for Vec<T> {}
