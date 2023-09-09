# enigma
This project is a simple implementation of the Enigma machine used by the Germans during World War II.
It's writen in Rust and uses [bevy](https://github.com/bevyengine/bevy) for the UI.
You can clone the repository and run it with `cargo run` or `cargo run --release` or you can download the latest release from the [releases](https://github.com/Arkitu/enigma/releases) page.

## Usage
The machine simulates 3 rotors and a reflector. You can type what you want to encrypt it or type an encrypted message to decrypt it.
The rotors are the I, II and III from the Enigma D, and the reflector comes from the same machine (https://www.cryptomuseum.com/crypto/enigma/wiring.htm#8).

You can press enter to reset the machine to its initial state.
