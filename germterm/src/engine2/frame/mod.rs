use crate::engine2::{buffer::Buffer, Position};

trait Framer<Buf: Buffer> {}

struct Frame {
    cursor_position: Position,
}
