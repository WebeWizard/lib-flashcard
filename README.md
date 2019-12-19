# lib-flashcard
Rust library for working with flashcards (Decks and Cards).

Uses lib-webe::webe_auth to handle authentication. (Only user who created the Deck can create/edit cards in that Deck. etc.)

Uses lib-webe::webe_web for http.  Provides Responders for manipulating Decks and Cards. 