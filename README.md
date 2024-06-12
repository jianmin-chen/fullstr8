programming lang around a deck of cards! similar to brainf where it operates on an array of numbers

there are 52 cards in a deck:

* 2 - 10 for spades, hearts, clubs, and diamonds
* king, queen, jack, and ace
* joker as a wildcard? shuffling cards? idk

```
(2 of spades) -- move 2 to the right
(2 of hearts) -- move 2 to the left
(2 of clubs) -- add 2 to current position
(2 of diamonds) -- sub 2 from current position
(king of spades) -- repr. > operator
(king of hearts) -- repr. < operator
(king of clubs) -- repr. == operator
(king of diamonds) -- repr != operator
(queen of spades (2 of spades) (king of spades) (jack of spades)) -- while the value of 2 to the left is > the value of 1 to the right
(queen of hearts) -- end of while. could do what lisp does but eh
(queen of clubs) -- input character and store at current
(queen of diamonds) -- output current ascii value
(jack of spades) -- move 1 to the right
(jack of hearts) -- move 1 to the left
(jack of clubs) -- add 1 to current position
(jack of diamonds) -- sub 1 from current position
(ace of spades) -- start of if
(ace of hearts) -- end of if
(ace of clubs (14)) -- parameter in joker wildcard, takes number and stores it at current, in this case 14
(ace of diamonds) -- else 
(joker () () ()) -- function. if statements are provided overwrites the current joker and stores this inside, otherwise runs what's inside joker.
```

hello world in this lang:

```
(joker (ace of clubs) (jack of spades) (ace of diamonds))
(joker (ace of clubs (104)))
(joker (ace of clubs (101)))
(joker (ace of clubs (108)))
(joker (ace of clubs (108)))
(joker (ace of clubs (111)))
(2 of hearts)(2 of hearts)(jack of spades)
(jack of spades)
(joker (ace of clubs (5)))
-- while 2 left < 1 left 
(queen of spades (2 of diamonds) (king of hearts) (jack of spades))
(ace of spades (2 of diamonds) (king of diamonds) (4 of diamonds))
-- move x amount right and store 3 left
(ace of hearts)
(queen of diamonds)
(queen of hearts)
```

spin up axum server sometime to get actual cards working lol
