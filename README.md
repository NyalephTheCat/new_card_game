# My untitled card game

This is a card game I'm working on. It's a work in progress.

## How to play

To play you have to have cargo installed, then run `cargo install trunk` if you don't have it installed.

To launch the game, launch a terminal at the root of the project and run

```bash
./dev.sh
```

This will launch a web server and open a browser window to the game.

## How to build

To build the game, run

```bash
./prod.sh
```

This will build the game and put it in the `dist` folder.

## What is each of the directories?

- `ansuz`: The frontend
- `eihwaz`: The backend
- `dist`: The built game