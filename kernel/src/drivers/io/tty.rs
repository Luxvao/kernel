pub trait Console {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn draw_char(&self, pos: (usize, usize), character: char);
}

pub struct Tty<T: Console> {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    start: usize,
    history: [char; 20480],
    sink: T,
}
