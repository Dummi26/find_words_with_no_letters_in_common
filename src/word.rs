pub struct Word {
    /// The original string used to create this word.
    string: Vec<String>,
    /// If this is true at a certain index, it means that that letter was used in the word. By definition, if this Word was creates using from_string, exactly five elements will be true, while all others are false.
    letters: [bool;26]
}
impl Word {
    pub fn empty() -> Self {
        Self {
            string: Vec::new(),
            letters: [false;26],
        }
    }
    pub fn from_letters(l: [bool;26]) -> Self {
        Self {
            string: Vec::new(),
            letters: l,
        }
    }
    pub fn from_string(str: String) -> Option<Self> {
        let mut out = Self {
            string: vec![str],
            letters: [false;26],
        };
        let mut len = 0;
        for char in out.string[0].chars() { // iterate over the word's utf-8 chars
            match get_letter_index(char) { // see if the character is recognized, and if so, which letter it is
                Some(index) => { // the letter was recognized
                    if out.letters[index] {
                        return None; // because a letter was used twice (using this word makes no sense)
                    };
                    out.letters[index] = true;
                    len += 1;
                    if len > 5 {
                        return None; // because the word is too long
                    }
                },
                None => { // unknown letter
                    return None; // because the word contains unsupported characters
                },
            };
        };
        if len < 5 { return None; }; // because the word is too short
        Some(out) // word is valid, return it
    }

    pub fn add_str(&mut self, str: String) {
        self.string.push(str);
    }

    pub fn used_letters(w1: &Self, w2: &Self, w3: &Self, w4: &Self, w5: &Self) -> u8 {
        let mut used_letters = 0u8;
        for i in 0..26 {
            if w1.letters[i] || w2.letters[i] || w3.letters[i] || w4.letters[i] || w5.letters[i] {
                used_letters += 1;
            }
        };
        used_letters
    }
    pub fn has_no_duplicates(w1: &Self, w2: &Self, w3: &Self, w4: &Self, w5: &Self) -> bool {
        Self::used_letters(w1, w2, w3, w4, w5) == 25 // if 25 letters were used, since every word uses exactly 5 letters and duplicate letters will only add 1 to the count, there were no duplicates.
    }
    pub fn as_u32(&self) -> u32 {
        let mut o: u32 = 0;
        for i in 0..26 {
            o = o << 1; // shift left (also ensures that last bit is 0)
            if self.letters[i] {
                o += 1; // set last bit to 1
            }
        }
        o
    }
    pub fn clone_letters(&self) -> [bool;26] {
        [
            self.letters[0],
            self.letters[1],
            self.letters[2],
            self.letters[3],
            self.letters[4],
            self.letters[5],
            self.letters[6],
            self.letters[7],
            self.letters[8],
            self.letters[9],
            self.letters[10],
            self.letters[11],
            self.letters[12],
            self.letters[13],
            self.letters[14],
            self.letters[15],
            self.letters[16],
            self.letters[17],
            self.letters[18],
            self.letters[19],
            self.letters[20],
            self.letters[21],
            self.letters[22],
            self.letters[23],
            self.letters[24],
            self.letters[25],
        ]
    }
    pub fn combine_letters(&mut self, other: &Self) -> u8 {
        let mut changed = 0;
        for i in 0..26 {
            if self.letters[i] == false && other.letters[i] {
                self.letters[i] = true;
                changed += 1;
            }
        }
        changed
    }
}
impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..26 {
            if self.letters[i] != other.letters[i] { return false; }
        };
        true
    }
}
impl Eq for Word {}
impl std::fmt::Display for Word {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut text = String::new();
        for s in self.string.iter() {
            text += s;
            text += ", ";
        }
        write!(formatter, "{}", text);
        Ok(())
    }
}

pub fn get_letter_index(letter: char) -> Option<usize> {
    match letter {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        'i' => Some(8),
        'j' => Some(9),
        'k' => Some(10),
        'l' => Some(11),
        'm' => Some(12),
        'n' => Some(13),
        'o' => Some(14),
        'p' => Some(15),
        'q' => Some(16),
        'r' => Some(17),
        's' => Some(18),
        't' => Some(19),
        'u' => Some(20),
        'v' => Some(21),
        'w' => Some(22),
        'x' => Some(23),
        'y' => Some(24),
        'z' => Some(25),
        _ => None,
    }
}

pub fn get_letter(index: usize) -> char {
    letters[index]
}

const letters: [char;26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];