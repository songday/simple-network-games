# simple-network-games

## How to start?

### Rust Backend
1. Download [Here](https://github.com/songday/simple-network-games/releases)
1. Double click
1. Open a browser and visit [http://localhost:3000](http://localhost:3000)
1. Have fun

### Java Backend
1. Run [GameApplication.java](https://github.com/songday/simple-network-games/blob/main/backend/gameserver-java/src/main/java/com/songday/game/GameApplication.java) `Java standalone backend will available later`
1. Open `frontend/html/lobby.html` in any browser (Supports `Firefox`, `Chrome`, `Microsoft Edge`) and create a game room by clicking `Create a new room` button
1. Open `frontend/html/lobby.html` in any other type of browser, not the same as above one or in `Private mode`
1. Have fun

## Games List
Currently working on:
1. Draw and guess
1. Snake

Planing:
1. Mine sweeper
1. Tetris
2. Rock Paper Scissors (Sometimes this solves arguments)


## Just need a backend (without games frontend)? Sure!
Function | Rust backend | Java Backend
-----|-----|-----
Create a new room| 1. Connect to ws://localhost:3000/room/unknown?roomType=UNKONWN&roomName=<Name a room>&player=<playerName>&capacity=<player amount, should greater than 1>&extraData= | 1. Connect to ws://localhost:3000 2. After connected, send: {"":} | N/A         |N/A
Join a room| 1. Connect to ws://localhost:3000?roomId=<roomId>&player=<playerName> | 
