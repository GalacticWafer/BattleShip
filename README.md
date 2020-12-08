# BattleShip
A simple console based implementation of the classic BattleShip game in Rust,
with an AI. The AI uses a stack to hold adjacent coordinates to any point
that was successfully attacked. AI guesses are pulled from the stack as long
as it is not empty. Otherwise, a point is randomly chosen. The board will be
re-printed every round to update the graphic. Here is an example:
![Gameplay](https://raw.githubusercontent.com/GalacticWafer/BattleShip/master/images/gameplay.png)
