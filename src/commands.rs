pub enum Command {
    NormalCursorLeft(u32),
    NormalCursorDown(u32),
    NormalCursorUp(u32),
    NormalCursorRight(u32),
    NormalEnter,
    NormalInsert,
}
