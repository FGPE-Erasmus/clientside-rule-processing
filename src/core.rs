mod rule;

trait Fire<T> {
    fn fired(&mut self, data: &T) -> Option<usize>;
}

trait Complete<T> {
    fn completed(&mut self, data: &T, data_pos: &Option<usize>) -> bool;
}