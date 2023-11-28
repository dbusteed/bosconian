# Bosconian

My take on the classic arcade game, [Bosconian](https://en.wikipedia.org/wiki/Bosconian). Built with [Bevy](https://bevyengine.org/).

Play at [https://dbusteed.github.io/bosconian](https://dbusteed.github.io/bosconian), or run locally with `cargo run --release`./wa

<br>

## Future Improvements
- General
    - if player hits two collidables at same time (should only lose one life)
    - audio
        - more (and better) sound effects
        - background music
    - star hatch / missle
    - possible FPS issues in WASM?
    - entity not found warnings
    - gamepad
- Classic
    - player might spawn on an enemy    
    - more levels
    - formations
- Endless
    - add ptype
    - star spawn
        - dynamic max stars
        - better spawn locations
    - pick up time bonuses
    - more and more fighters, spawn off screen so no timer necessary
    - random rocks
    - enemy ship drops mines