#[derive(Default, Debug)]
pub struct AnimationState {
    pub playing: bool,
    pub looping: bool,
    pub current_frame: i32,
    pub progress_ticks: i32,
    pub scale: i32,
    pub x: i32,
    pub y:i32,
}