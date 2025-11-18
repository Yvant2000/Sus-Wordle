# Sus Wordle

<img width="318" height="333" alt="image" src="https://github.com/user-attachments/assets/6df6ff46-4feb-4eaa-9bab-2a954f45f643" />


Sus Wordle is a CLI tool to draw a crewmate (or an impostor, I don't know, please don't vote me out) in the Wordle play-board.

## Compilation

You need at least cargo version 1.87.0

```
$ git clone git@github.com:Yvant2000/Sus-Wordle.git
$ cd Sus-Wordle
$ cargo build --release
```

## Usage

```
$ SusWordle <word>
```

<img width="363" height="273" alt="image" src="https://github.com/user-attachments/assets/99051203-156a-440f-9051-f047b9817c9c" />

## TODO

* Automatic fetch of today's solution using New York Times API
* Search for sideways crewmates if no standing crewmate is found
* Allow green background (this means the first line cannot be empty as this will end immediatly the game).
The easiest implementation would be to remove the first line from the solution if it's green
