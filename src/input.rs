use macroquad::input::{is_key_down, KeyCode};

use nanoserde::{DeBin, SerBin};

#[derive(Default, Debug, Clone, Copy, DeBin, SerBin)]
pub struct Input {
    pub jump: bool,
    pub throw: bool,
    pub fire: bool,
    pub slide: bool,

    pub left: bool,
    pub right: bool,
    pub down: bool,
}

pub fn collect_input(controller_id: usize) -> Input {
    let mut input = Input::default();

    if controller_id == 1 {
        input.jump = is_key_down(KeyCode::Space) || is_key_down(KeyCode::W);
        input.throw = is_key_down(KeyCode::R);
        input.fire =
            is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::F) || is_key_down(KeyCode::L);

        input.left = is_key_down(KeyCode::A);
        input.right = is_key_down(KeyCode::D);
        input.down = is_key_down(KeyCode::S);

        input.slide = is_key_down(KeyCode::C);
    }

    if controller_id == 0 {
        input.throw = is_key_down(KeyCode::K);
        input.jump = is_key_down(KeyCode::Up);
        input.left = is_key_down(KeyCode::Left);
        input.right = is_key_down(KeyCode::Right);
        input.down = is_key_down(KeyCode::Down);
        input.slide = is_key_down(KeyCode::RightControl);
    }

    input
}
