### "my stamina is about at negative 9765726 so no, i dont have to kill a snake but they are all super friendly and dont seem like such type of ppl."

^ sample output


# Markov-Chain Sentence Generator

This sentence generator uses markov chains to extract the probability of a particular word following a particular set of words.

It requires massive amounts of data to actually generate something impressive.

Otherwise it will just copy the input.

It is still fun to play around with though.

You can change `WORD_MATCH_COUNT`. More matched words will follow the input more strictly. Less matched words will seem more random.

It's set to `2`, which makes it possible to still get something entertaining with a limited dataset.

## How to use

* Firstly, compile it. Run `cargo build --release`
* Make sure that you have a text file ready, with sentences seperated by '.'
* Now that it's compiled, you can either drag a text file onto it, or you can run the program and enter the path to the textfile yourself
* The program will generate 200 words at a time
* Press enter to generate more. You can do this 100 times until it quits.


Please open an issue for any bugs you encounter.