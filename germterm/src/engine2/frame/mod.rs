use crate::engine2::{Position, buffer::Buffer};

trait Framer<Buf: Buffer> {}

struct Frame {
    cursor_position: Position,
}
