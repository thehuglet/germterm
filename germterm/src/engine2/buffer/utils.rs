use std::io::Write;

use super::Buffer;
use crate::engine2::Position;

/// Dumps the contents of a buffer using a writer.
///
/// Each row is separated by a newline. Only the character of each cell is included.
pub fn dump_buffer(buffer: &dyn Buffer, writer: &mut dyn Write) -> std::io::Result<()> {
    let size = buffer.size();
    for y in 0..size.height {
        for x in 0..size.width {
            let cell = buffer.get_cell(Position::new(x, y));
            let mut buf = [0u8; 4];
            let s = cell.ch.encode_utf8(&mut buf);
            writer.write_all(s.as_bytes())?;
        }
        if y < size.height - 1 {
            writer.write_all(b"\n")?;
        }
    }
    Ok(())
}

/// Dumps the contents of a buffer to a string.
///
/// Preallocates the estimated capacity for better performance.
pub fn dump_buffer_to_string(buffer: &dyn Buffer) -> String {
    let size = buffer.size();
    let mut result = Vec::with_capacity((size.width as usize + 1) * size.height as usize);
    let _ = dump_buffer(buffer, &mut result);
    String::from_utf8(result).unwrap_or_default()
}
