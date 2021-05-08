# projinda

# Description 
A chess game in the Fog-of-war variant. You can only see squares which you can move to and/or attack with your own pieces.

Installation: write "cargo run --release" in the terminal

Uses the game library 'ggez'
## Collaborators
* Isak Einberg
* Hampus Hallkvist

Upplägg:

* Första veckan:
  * Implementera pjäser och bräde
  * Designa brädet och pjäser (grafikdelen)
  * Kunna dra och släppa pjäser
  * Piece logic & Move validation
  * Nätverk (server NodeJS)

* Andra veckan:
  * Movement indication
  * Player implementation
    * Turn-system: Spelare kan endast flytta sina egna pjäser på sin egen turn.
    * Win/lose system
  * Special rules (rockad/castling, promotion, en passant)
  * Fog of war-system
    * (Pjäser kan “se” de rutor som de kan attackera och/eller gå till)
  * Main menu
  
* Om tid finns:
  * Inställningsbar Schackklocka
  * Ändra färg på brädet
  * Pre-moves
  * See captured pieces
  * Resign-knapp
  * Navigera genom historik av drag
  * Joina lobby som åskådare
