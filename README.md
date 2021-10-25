# Fog of War Chess
![Fog of war banner](resources/banner.png)

## Introduction
An online chess game in the 'Fog of war variant'. You can only see squares/tiles which you can move to with your own pieces.

Players can create and join online game lobbies.
Game server repo (made in TypeScript): https://github.com/Hampfh/fog_of_war_server

## Background
Made as a collaborative project between Isak Einberg and Hampus Hallkvist as a part of the course "DD1349 Project in Introduction to Computer Science".

We initially coded the core of the game together by remote pair programming, and later went on to code in parallel. Isak focused mostly on implementing game rules, interactions and mechanics, while Hampus mainly focused on graphics and UI, as well as creating the server-lobby networking system.

The repository is cloned from an existing KTH GitHub Enterprise repository. Consequently, no issues, projects or pull requests are present in this cloned repo, and approximately half of my (Isak's) commits are not linked with my account as the author.

## Installation
* Make sure you have Rust and its prerequisites installed on your system.
* Open the terminal in the repository directory and write:
```
cargo run --release
```

## Collaborators
* Isak Einberg
* Hampus Hallkvist

## Dependencies:
* ggez: Game and graphics library.
* rust_socketio: Socket.io implementation for Rust.
* serde_json: JSON parsing.
* state: State manager.
