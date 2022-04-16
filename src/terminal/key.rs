#[derive(PartialEq, Debug)]
pub enum Key {
    Exit,
    Up,
    Down,
    Left,
    Right,
    Other(u8),
    Buffer([u8; 3]),
    None,
}

impl Default for Key {
    fn default() -> Self {
        Key::None
    }
}

impl From<u8> for Key {
    fn from(byte: u8) -> Self {
        match byte {
            3 => Key::Exit,
            4 => Key::Exit,
            97 => Key::Left,
            100 => Key::Right,
            113 => Key::Left,
            115 => Key::Down,
            119 => Key::Up,
            122 => Key::Up,
            _ => Key::Other(byte),
        }
    }
}

impl From<[u8; 3]> for Key {
    fn from(buf: [u8; 3]) -> Self {
        match buf {
            [27, 91, 65] => Key::Up,
            [27, 91, 66] => Key::Down,
            [27, 91, 67] => Key::Right,
            [27, 91, 68] => Key::Left,
            _ => Key::Buffer(buf),
        }
    }
}
