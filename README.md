# **Invaders**: Terminal Based Game Built on _Rust_

- Features of Rust extensively used in this project
    - Channel
    - Thread
    - Struct
    - Trait
    - Ownership and Borrowing [References]
    - Closures
    - Control Flows
    - Standard Libraries
    - External Modules

- **Dependencies**
    - crossterm: Terminal utility to manage enable/disable alternate screen, ready/map KeyCode event inputs & Show/ Hide cursor
    - rusty_audio: Enable audio playback and manage audio files
    - rusty_time: Timer utility 
    
- **Modules**
    - main
    - lib
    - frame
    - render
    - player
    - shot
    - invaders
    
- **Module Usage**
    - _main_
        - Orchestration manager of the "Invaders" module
        - Define terminal for game [crossterm]
        - Setting up channel [mpsc] and thread to render game output [render]
        - Define Player and Invaders [player & invader] 
        - Run un-defined loop to lookup keyboard events during game [crossterm]
        - Manage module updates
        - Cleanup
    - _lib_
        - Repository of modules defined in the project 
        - Define constants to be used across the project
        - Exposes modules [frame, render, player, shot & invaders]
        - Exposes constants [NUM_ROWS & NUM_COLS]
    - _frame_
        - Define the game play area on the screen   
    - _render_
        - Print the co-ordinates on the screen
    - _player_
        - Struct which initialises the player with co-ordinates and the shots
        - Functions
            - _move_left_: On KeyCode "Left Arrow" moves the player left
            - _move_right_: On KeyCode "Right Arrow" moves the player right
            - _shoot_: Calls the shot module to shoot at the invaders
            - _update_: Updates the co-ordinates of the bullet based on timer
            - _detect_hits_: If the bullet hit an invader in a shot, remove the invader
    - _shot_
        - Struct which initialises the shot with co-ordinates, a boolean explode and a timer
        - Functions
            - _update_: Move the bullet on the y axis if the game has begun and if the bullet has not exploded
            - _explode_: If bullet hits an invader, set explode flag to true and update timer
            - _dead_: if the bullet has hit an invader, remove the bullet from the screen
    - _invaders_
        - Struct which initialises the army [array of invader with co-ordinates], timer and direction [right]
        - Functions
            - _update_: At initialization, the direction is set to "right", so the army moves to the right till the end of the frame
                    : At the end of frame, the direction is reversed and the army makes a step downward with increased speed [timer]
            - _all_killed_: Check if the player has killed all the invaders [based on array count]
            - _reached_bottom_: Check if the army has reached the bottom of the frame
            - _kill_invader_at_: If the bullet has hit an invader, remove from the frame
            
- **Playing Invaders**
    - Clone the project and run the command "_cargo run_" from the terminal windows.
    - The module will download the dependencies, build the project and make an executable based on your OS
    - Voila!, you will now be on the alternate screen with the GAME ON!
    
# Attribution: The project was inspired from _Nathan Stocks_

Feel free to checkout the code and play-around. I will be adding features as I get familiar with the concepts of Rust.

- **Unplanned Upgrades**
    - Audio Playback for events [Audio playback features has been implemented in the project but commented for enhancements]
    - Welcome/ Exit screens
    - Multi-player
                      
