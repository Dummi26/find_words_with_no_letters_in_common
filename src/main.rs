use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use crate::word::Word;

mod word;

fn main() {
    println!("Initializing...");
    let word_list: Vec<String> = {
        // ABSOLUTE FILE PATH HERE --- CHANGE THIS BEFORE RUNNING
        match File::open("/home/mark/Documents/code/rust/find_words/word_list.txt") {
            Ok(file) => {
                let mut wl = Vec::<String>::new();
                for line in BufReader::new(file).lines() {
                    match line {
                        Ok(line) => { wl.push(line); },
                        Err(_) => {},
                    };
                }
                wl
            },
            Err(error) => {
                let url = "https://gist.githubusercontent.com/cfreshman/cdcdf777450c5b5301e439061d29694c/raw/b8375870720504ecf89c1970ea4532454f12de94/wordle-allowed-guesses.txt";
                println!("Could not read word list from file. Please get the list and save it, then change the path in /src/.\nError: {}\nURL: {})", error, url);
                panic!();
            },
        }
    };
    println!("\nAbstracting words... (timer starts here)\n");
    let start_time = std::time::Instant::now();
    let mut words: Vec<Word> = Vec::new();
    /* Convert strings to the abstract word::Word type. */{
        for word_str in word_list.into_iter() {
            match Word::from_string(word_str.clone()) {
                Some(word) => { // the word consisted of 5 unique, recognized letters
                    let mut ok = true;
                    for w in words.iter_mut() {
                        if *w == word { // a word with the same signature (using the same 5 letters in any order) already exists.
                            w.add_str(word_str); //  add the string to the word object. This means that the word will be treated as one
                            ok = false;           // during processing, speeding everything up, but if this word is part of a match, all possible
                            break;                // arrangements of these five letters that were in the word list will be displayed to the user.
                        };
                    };
                    if ok { // the word was valid and did not yet exist in the words vec,
                        words.push(word); //  so we have to add it. Vec::push is slow, but since the second part of my algorithm
                    };                     // takes so much longer, this is almost insignificant and not worth optimizing out.
                },
                None => {
                    println!("Ignored word '{}'.", word_str);
                },
            };
        };
    };
    println!("\nSearching for solutions...\n");
    /*
    Expected solution:
        FJORD
        GUCKS
        NYMPH
        VIBEX
        WALTZ
    Missing: Q
    */
    /* Find matches */
    let word_pairs = {
        let len = words.len();
        let mut percent_previous = 0;
        let len = if len <= 5 {
            println!("Too few distinct, valid words were found ({}, to be exact). Please expand your word list!", len);
            println!("Words:");
            for w in words.iter() {
                println!("  {}", w);
            }
            0 // prevents the loop from ever starting, therefor preventing the division by zero (which would happen at len=5 when calculating percent_current)
        } else { len };
        let mut word_pairs: Vec<[&Word;5]> = Vec::new();
        for i in 4..len {

            let percent_current = 100 * (i-4) / (len-5);
            if percent_current != percent_previous {
                println!("{}%", percent_current);
                percent_previous = percent_current;
            };

            let w1 = &words[i];
            let W1 = Word::from_letters(w1.clone_letters());

            //println!("[1] Checking {}.", w1);

            for i in 0..i {
                let w2 = &words[i];
                let mut W2 = Word::from_letters(W1.clone_letters());

                //println!(" [2] Checking {}.", w2);
                
                if W2.combine_letters(w2) == 5 {
                    for i in 0..i {
                        let w3 = &words[i];
                        let mut W3 = Word::from_letters(W2.clone_letters());
                        
                        //println!("  [3] Checking {}.", w3);
                        
                        if W3.combine_letters(w3) == 5 {
                            for i in 0..i {
                                let w4 = &words[i];
                                let mut W4 = Word::from_letters(W3.clone_letters());
                                
                                //println!("   [4] Checking {}.", w4);
                                
                                if W4.combine_letters(w4) == 5 {
                                    for i in 0..i {
                                        let w5 = &words[i];
                                        let mut W5 = Word::from_letters(W4.clone_letters());
                                        
                                        //println!("    [5] Checking {}.", w5);
                                        
                                        if W5.combine_letters(w5) == 5 {
                                            println!(" - - -\n {}\n {}\n {}\n {}\n {}", w1, w2, w3, w4, w5);
                                            word_pairs.push([w1, w2, w3, w4, w5]);
                                        };
                                    };
                                };
                            };
                        };
                    };
                };
            };
        };
        word_pairs
    };
    println!("\nDone after {} seconds.\n", start_time.elapsed().as_secs_f64());
    println!("Found the following pairs:");
    for pair in word_pairs {
        println!("\n-> {}\n-> {}\n-> {}\n-> {}\n-> {}", pair[0], pair[1], pair[2], pair[3], pair[4]);
    }
}