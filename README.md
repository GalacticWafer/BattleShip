# BattleShip
A simple console based implementation of the classic BattleShip game in Rust,
with an AI. The AI uses a stack to hold adjacent coordinates to any point
that was successfully attacked. AI guesses are pulled from the stack as long
as it is not empty. Otherwise, a point is randomly chosen. The board will be
re-printed every round to update the graphic. Here is an example:

Round 8:
Please input your guess (x and y, separated by a space).
2 5
You guessed: (5,2)
a ship has been sunk!
HIT!
Computer attacked (3,4), and missed!
       A    B    C    D    E    F    G    H    I    J 
     __________________________________________________
1   |  .    .    .    .    .    .    .    .    .    .  |
2   |  .    .    .    .    .    .    .    o    .    .  |
3   |  .    .    .    .    .    .    .    .    .    .  |
4   |  .    .    o    .    o    .    .    .    .    .  |
5   |  .    X    X    X    X    .    .    .    .    .  |
6   |  .    .    .    .    .    .    .    .    .    .  |
7   |  .    .    .    .    .    .    .    .    .    .  |
8   |  .    o    .    .    .    .    .    .    .    .  |
9   |  .    .    .    .    .    .    .    .    .    .  |
10  |  .    .    .    .    .    .    .    .    .    .  |
     __________________________________________________


       A    B    C    D    E    F    G    H    I    J 
     __________________________________________________
1   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
2   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
3   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
4   |  ~    S    ✔    ✔    ~    ~    ~    ~    ~   ~  |
5   |  ~    ~    ~    D    D    ~    ~    ~    ~    ~  |
6   |  ~    ~    ~    C    C    C    C    C    ~    B  |
7   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    B  |
8   |  ~    ~    ~    ~    ~    ~    ~    ~    ~    B  |
9   |  ~    ~    ~    U    U    U    ~    ~    ~    B  |
10  |  ~    ~    ~    ~    ~    ~    ~    ~    ~    ~  |
     __________________________________________________

