# Wordle

This project is the frontend half of the wordle game written almost entirely in Rust using the Yew framework, which means this entire app compiles down to web assembly! The [wordle backend](https://github.com/seewishnew/wordle-backend.rs) project is required to be deployed for the fronend to work.

The frontend app consists a user registration page, a menu to create or join an existing game, a game creation page where the solution can be entered for a new game, a leaderboard page for the game creator to watch other players' scores, and the actual wordle page for the players to solve the puzzle.

## New User Registration

![New user registration page with a keyboard and a field to enter the username for the leaderboard](new_user.png?raw=true "New User Registration Page")

## Menu

![Menu page with an option to create a new game or to play an existing game by entering a valid Game ID](menu.png?raw=true "Menu Page")

## Create New Game

![Create game page with 5 cells in one row and a keyboard below to enter the 5 letter answer](create_game.png?raw=true "Create Game Page")

## Play Wordle

![Wordle page with 6 rows of 5 cells each with a keyboard below the game grid](wordle.png?raw=true "Wordle Page")

## Leaderboard

![Leaderboard page with details about the answer, the Game ID and two users, one of whom seems to have won after 5 out of 6 attempts for this game](leaderboard.png?raw=true "Leaderboard Page")
