# Amazons AI

Simple min-max AI for a modified
[Game of the Amazons](https://en.wikipedia.org/wiki/Game_of_the_Amazons)

The last player to take a turn wins. A turn consists of moving a piece like a
chess queen, and then placing a 'wall' out from the piece that moved like a queen.

The heuristic used to score a game-board is based on which player can move
their pieces to each open square in fewer moves. A Breadth-First-Search is
performed for both teams, the score is based on how many squares the team can
get to in fewer moves than the opponent.

The AI has some interesting quirks. I have been soundly beaten in the mid-game,
but it has a tendency to blunder the end game. I know there are some bugs in
the win-condition for the endgame because AI-AI battles record the wrong team
winning.

The UI is limited, but functional. Some improvements I'm considering:

* 'undo' command to go back a turn
* 'ai' command to let the ai play for you one turn
* See the heuristic evaluation score for each board position
* Alternate move input methods

```
$ cargo run --release
White is controlled by? [human, ai]
ai
Black is controlled by? [human, ai]
human
White to go, controlled by Ai
##########
#........#
#........#
#..W..W..#
#........#
#........#
#..B..B..#
#........#
#........#
##########

Choosing among 676 moves with 4 depth
Black to go, controlled by Human
##########
#........#
#........#
#..W.....#
#....#...#
#.....W..#
#..B..B..#
#........#
#........#
##########

Choose move for team Black in format 'RowCol RowCol RowCol'
63 43 54
White to go, controlled by Ai
##########
#........#
#........#
#..W.....#
#..B.#...#
#...#.W..#
#.....B..#
#........#
#........#
##########

```
