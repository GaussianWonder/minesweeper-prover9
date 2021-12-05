# Minesweeper - Prover9

This is a simple configurable minesweeper game that you can play.

The objective of this project is to solve the puzzle using **Prover9**.

## Notes

**Flags** and **Unknown** cells will sometimes be reffered as the user's **hints** on the current unrevealed frontier of the board.

## Controls

|              |              |
| ------------ | ------------ |
| _**Hotkey**_ | _**Action**_ |
| Left click   | Reveal cell  |
| Right click  | Flag cell    |
| Middle click | Unknown cell |

## Usage

You must break the ice before asking Mace4 to solve the board or Prover9 for hints.

### Mace4 button

This button will take the currently unrevealed frontier and solve it for you regardless of what you hinted on the frontier.

If it cannot determine the state of the whole frontier it will:

- flag cells that are 100% bombs
- reveal cells that are 100% not bombs
- mark as unknown all of the other cells

### Prover9 button

This button will take all of your hints on the current frontier and tell you whether or not you are correct
