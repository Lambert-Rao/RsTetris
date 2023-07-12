pub const NEXT_TIP_POS: [u16;2] = [26, 2];
pub const INIT_LAST_POS: [i16;2] = [12,1];
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}
pub const GAME_AREA_POSITION : [u16;2] = [2, 1];
//multiply 2 to make it looks like a square in the precision of rendering
pub const GAME_AREA_SIZE : [u16;2] = [10,20];