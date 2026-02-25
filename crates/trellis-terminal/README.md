# Terminal Interfacing

This crate provides a encapsulation of the raw terminal access required for
trellis to function. Currently this crate depends on crossterm but in the
future it might be educational to implement crossplatform terminal interfacing
from scratch, which would also make this project completely dependency free.

## Exposed Capabilities

The major reason for breaking this out into it's own crate is to only expose
the minimal needed subset of crossterm for the sake of simplicity and
cleanliness. The exposed capabilities are as follows:

- Event Handling
  - Read key presses and report terminal resizing.
- State Queries
  - Window size, rows, columns.
- Cursor Operations
  - Show, hide, move, save position, restore position.
- I/O
  - Write arbitrary bytes, strings, and chars.
  - Buffered write and flush.
  - Enabling and disabling raw mode.
- Commands
  - Leave/exit alternate screen.
