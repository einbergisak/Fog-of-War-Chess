# projinda

# Description
An online chess game in the Fog-of-war variant. You can only see squares which you can move to and/or attack with your own pieces.

Players can create and join online game lobbies.
Game server repo (made in TypeScript): https://gits-15.sys.kth.se/hallkvi/fog_of_war_server

Installation: Open the repository folder and write "cargo run --release" in the terminal.

Libraries:
* ggez: Game and graphics library.
* rust_socketio: Socket.io implementation for Rust.
* serde_json: JSON parsing.
* state: State manager.

## Collaborators
* Isak Einberg
* Hampus Hallkvist

Upplägg:

* Första veckan:
  * ~Implementera pjäser och bräde~
  * ~Designa brädet och pjäser (grafikdelen)~
  * ~Kunna dra och släppa pjäser~
  * ~Piece logic & Move validation~
  * ~Nätverk (server NodeJS)~

* Andra veckan:
  * ~Player implementation~
    * ~Turn-system: Spelare kan endast flytta sina egna pjäser på sin egen turn.~
    * ~Win/lose system~
  * Main menu
  * ~Special rules (rockad/castling, promotion, en passant)~
  * Fog of war-system
    * (Pjäser kan “se” de rutor som de kan attackera och/eller gå till)
  * Movement indication
  * Select by clicking


* Om tid finns:
  * Schackklocka
  * Pre-moves (Isak)
  * Joina lobby som åskådare (Hampus)
  * Ange namn när man skapar/joinar en lobby.
  * See captured pieces
  * Resign-knapp
  * Navigera genom historik av drag
  * Ändra färg på brädet
