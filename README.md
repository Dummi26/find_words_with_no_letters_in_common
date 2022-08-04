# find_words_with_no_letters_in_common

Inspired by Matt Parker's video (https://youtu.be/_-AfhLQfb6w), this is a rather simple program written in rust that reads a word list from a txt file and then iterates over it, trying to find five words of five letters with no letters in common as fast as possible.


# What you should know

In /src/main.rs, there is an absolute path that specifies the location of word_list.txt. Please adjust this path according to where your word list is. If the file cannot be found, access to it is denied or there is any other io error, the programm will tell you to download the word list from a url (which is copied from matt parker's video description).

Don't actually do this - for whatever reason, not all of the 5 words matt found seem to be in the file behind that url. I have manually added the remaining ones in my word_list.txt file (at random locations in the file, to test the program's functionality).


# How

The first step is to abstract all the words into the Word struct. It stores 26 booleans, one for each letter (only a-z supported). By default, all of these will be false. By initializing a Word using Word::from_string, exactly 5 of them will be set to true. These are the letters that the word contains. If the word does not contain exactly 5 different letters, Word::from_string returns None, meaning that the Word was not valid. This creates a list of Words which we can iterate over.

To ensure that all words will be checked and none will be missed, there are 5 nested for loops. The outermost one starts at 4, as any other value would result in a range of 0..i with i being less than or equal to 0 at the innermost loop. Still, if my brain has not completely stopped working, all combinations will be checked properly.

Because the inner for loops use 'i in 0..i' to iterate, there will never be a situation where two 'i's will be the same (i.e. we are comparing one word to itself) nor will there be a situation where a comparison will take part twice (one time comparing a to b, then later b to a).

If two words are found which have any overlap in letters, we immedeately stop searching and move on to the next word pair.

My solution to making this rather efficient is the following: (my code here is not super expressive with W1-5, the following explanation will completely disregard why there are 5 of these)

Instead of comparing words to one another, we create a new 'word'. (In code, this is done using Word::from_letters)
This word breaks the rule normal words have, in that it is never required to have only five letters. Its string field is also empty, as we won't be needing it.

Given any number of words n with 5 letters each, this special word will consist of all of the letters. If the letters in the special word are 5n, all underlying words must have had different letters, as any double letter would have been counted only once in this new word, meaning its letter count would be lower than the expected 5n.

Conveniently, this counting process can happen in the same function (Word::combine_letters) as the merging of an actual word with out custom word.
This function sets all values to true which
 - were false in the custom word (the one that can consist of more than 5 letters)
 - were true in the actual word (from the wordlist)
This is the same as ORing the two words:
false/false -> false, as the second condition fails
true/false and true/true -> true, as the value was already true and (because the first condition fails) was not changed
false/true -> true, as both conditions apply
It also counts how many times it changed a value from false to true, which gives it the amount of letters in the actual word that were not present in the custom word before. If 5 letters were added, none of the actual word's letters were present in the custom word before, as the actual word has exactly 5 letters. In any other case, there were overlapping letters. (In code, this is written as W5.combine_letters(w5) == 5)

# Result

Running this algorithm on my laptop (Lenovo IdeaPad 5, AMD Ryzen 5 5500U) took almost exactly 16 minutes. I'm certain this can be optimized further, probably far below the 5 minute mark, but my program doesn't take an entire month to run, which is already pretty good. Implementing some parallelism here seems pretty simple, but i have no real reason to spend any more time on this.

And yes, it does actually work. This is the output:

# Output

```
cargo run --release
```

```
Done after 960.053075315 seconds.

Found the following pairs:

-> waltz, 
-> vibex, 
-> nymph, 
-> gucks, 
-> fjord, 

-> waqfs, 
-> vozhd, 
-> grypt, 
-> cimex, 
-> blunk, 

-> waqfs, 
-> vozhd, 
-> grypt, 
-> clunk, 
-> bemix, 

-> waqfs, 
-> vozhd, 
-> kempt, 
-> cylix, xylic, 
-> brung, 

-> waqfs, 
-> vozhd, 
-> kreng, 
-> jumby, 
-> clipt, 

-> waqfs, 
-> vozhd, 
-> treck, 
-> pling, 
-> jumby, 
```

Notice that cylix and xylic both consist of the same letters, which is why they were combined into one "word".
