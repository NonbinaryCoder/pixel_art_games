use std::time::Duration;

use bevy::prelude::*;

pub const FORWARD_KEYS: [KeyCode; 5] = [
    KeyCode::Space,
    KeyCode::W,
    KeyCode::D,
    KeyCode::Up,
    KeyCode::Right,
];

pub const RIGHT_KEYS: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];
pub const LEFT_KEYS: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];

pub const EXIT_KEYS: [KeyCode; 1] = [KeyCode::Escape];

pub const WAIT_REPEAT_DURATION: Duration = Duration::from_millis(500);

pub const REPEAT_STEP_DURATION: Duration = Duration::from_millis(1000 / 30);
