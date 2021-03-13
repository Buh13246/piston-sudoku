# A simple Sudoku based on the piston game engine.
## This is created with the tutorial on piston from @bvssvnl https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/sudoku

## Thanks a lot :)

The endproduct saves the gamestate via serde_json. The Game panics on win.

The `saves/current_save.json` will be loaded as default on startup.

A game can be saved with the `S` Key, and always overrides the `saves/current_save.json`. There is no autosave. To load a specific save, you need to copy it as `saves/current_save.json` currently.

All in all I'm very happy with the result and the game engine is really easy but powerfull to use.

# Licenses
The Fonts have a special license, the license is located at `assets/LICENSE`.