# Amazons AI

Simple min-max AI for a modified
[Game of the Amazons](https://en.wikipedia.org/wiki/Game_of_the_Amazons)

The last player to take a turn wins. A turn consists of moving a piece like a
chess queen, and then placing a 'wall' out from the piece that moved like a queen.

The heuristic used to score a game-board is based on which player can move
their pieces to each open square in fewer moves. A Breadth-First-Search is
performed for both teams, the score is based on how many squares the team can
get to in fewer moves than the opponent.

The AI has some interesting quirks.

The UI is limited, but functional. Some possible improvements

* 'help' command to list available commands
* Better cli printing to mention other available commands
* Alternate move input methods (TUI?)
* Multi-threaded AI (rayon?)
* Other heuristics?
* Min-Max search to more depth later in the game

![Screenshot showing color board and row column input](/amazons.png)
