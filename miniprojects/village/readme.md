# What is this

This is an old project they gave me in school to improve my C# skills, that I a now rewriting in Rust to improve my skills.

## project idea

This is a small village, which looks like this topologically:

|  | House |  |  |  |  |
|---|---|---|---|---|---|
| Market | Square | Tavern |  |  |  |
| Fields | Road | Road | Customs | Road | Souvenir shop |
|  | Castle |  |  |  |  |


We also give the places a number, as it just makes life easier:

| 0 | Square |
|---|---|
| 1 | Road (1) |
| 2 | Castle |
| 3 | Market |
| 4 | Field |
| 5 | House |
| 6 | Tavern |
| 7 | Road(2) |
| 8 | Customs |
| 9 | Road(3) |
| 10 | Souvenir shop |

Our goal is to be able to make a 'player' move throught the landscape, add a small inventory system, and make it so the player can't access everything without the right items.

To walk through the customs tile, we need an id card, which we can find in the house.

The goal is to get to the souvenir shop, and get the souvenir.

Everything is to be done through a console interface.