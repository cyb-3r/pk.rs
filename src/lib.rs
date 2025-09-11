/*!
# A safe and robust picross library

Pkrs (pronounced "Picross") provides the core types and tools to create a picross game in Rust!
It handles the puzzle logic and state, enabling you to focus on building the user
experience ontop of it.

## Pkrs handles

* Puzzle representation, validation and solving;
* Clue generation;

However, UI, Gameplay, Rendering, and other aspects of the game are left for you to implement.
This design gives you the freedom to implement a picross game for any platform like
the web, the console as a TUI, desktop environment or anywhere else Rust can compile to.

## How to use this library

A simple and straightforward way of using this library will be added soon!
But right now, you can create a simple puzzle by doing this:

```no_run
use pkrs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // load the solution pattern from a text file
  let mat = binmat::BinMat::from_file("path_to_mat.txt")?;

  // create the puzzle's components
  let board = board::Board::from_mat(&mat)?;
  let clues = clues::PuzzleClues::from_mat(&mat);

  /* then do whatever you want with that */

  Ok(())
}
```
*/

pub mod binmat;
pub mod board;
pub mod clues;
mod utils;
