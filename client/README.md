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
  * Fixa structs, pjäsarray.
  * Designa brädet och pjäser (grafikdelen)
  * Kunna dra och släppa pjäser
  * Piece logic & Move validation (behöver inte fixa schack eller schackmatt)
* Andra veckan:
  * Special rules (rokad, promotion, en passant)
  * Fog of war-system
    * (Pjäser kan “se” de rutor som de kan attackera och/eller gå till)
  * Main menu
  * Nätverk (server NodeJS)
  * Om tid finns:
    * Inställningsbar Schackklocka
    * Ändra färg på brädet
    * Pre-moves
    * See captured pieces
