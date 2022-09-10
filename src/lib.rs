use itertools::Itertools;
use regex::Regex; // -> added to Cargo.toml file
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs; // -> added to Cargo.toml file

pub mod day1 {
    pub fn part1(_file_name: &str, _target: &i32) -> Result<i32, &'static str> {
        let content: String = super::fs::read_to_string(_file_name).unwrap();

        let v: Vec<i32> = content.lines().map(|x| x.parse().unwrap()).collect();
        // What we did:
        // we started from a String type -> we created a Lines struct via string.lines(): this is very similiar to the Split struct
        // split implements the map method: we define in |x| the variable on which we shall iterate and we put the function right after it
        // since x is a &str type, it implements the parse() method
        // since parse returns a Result type (indeed the &str might not be parseable), we unwrap the result
        // now we have a Map struct of i32 values -> in order to convert it to a Vec type we use .collect()

        let hash_set: super::HashSet<i32> = v.iter().cloned().collect();
        // What we did:
        // we started from a Vec<i32> type -> we convert it into an Iter struct.
        // the iter contains references, in order to create an equivalent de-referenced array we need to use .cloned()
        // we apply .collect() in order to convert the Iter into an hashset

        // watch out in using .collect:
        // Map.collect() -> Vec
        // Iter.collect() -> HashSet

        // HashSet is useful thanks to methods such as .contains(), .intersection(), .union()
        let mut good_couple: (i32, i32) = (0, 0);
        for value in v {
            let complement = _target - value; //recall that value + complement = 2020
            if hash_set.contains(&complement) {
                good_couple.0 = value;
                good_couple.1 = complement;
                break;
            }
        }
        if good_couple != (0, 0) && *_target != 0 {
            return Ok(good_couple.0 * good_couple.1);
        } else {
            return Err("There isn't any good couple :(");
        }
        // What we did:
        // we iterated over each element of the vector and we compute the complement: this is the other item we're looking for
        // thanks to the HashSet, we are able to identify whether the complement is present in list or not
        // if so, we compute the product between the two numbers
    }

    pub fn part2(_file_name: &str, _target: &i32) -> Result<i32, &'static str> {
        let content: String = super::fs::read_to_string(_file_name).unwrap();
        let v: Vec<i32> = content.lines().map(|x| x.parse().unwrap()).collect();
        let hash_set: super::HashSet<i32> = v.iter().cloned().collect();
        //recalll that now value + complement1 + complement2 = 2020
        for value in &v {
            for possible_complement1 in &v {
                let complement2 = _target - (possible_complement1 + value);
                if hash_set.contains(&complement2) {
                    return Ok(value * possible_complement1 * complement2);
                }
            }
        }
        return Err("There isn't any good triplet :("); //we reach this part of code only if values are missing, otherwhise we exit inside the for loop
    }
}

pub mod day2 {
    pub fn part1(file_name: &str) -> Result<i32, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        let v: Vec<_> = content.lines().map(|x| x.split_whitespace()).collect();
        //We have now a vector of iterators
        let mut counter: i32 = 0;
        for it in v {
            let sub_vector: Vec<&str> = it.collect(); //Convert the iterator into a vector of &str
            let (range_text, character, context) =
                (sub_vector[0], sub_vector[1].replace(":", ""), sub_vector[2]);
            let range: Vec<i32> = range_text.split("-").map(|x| x.parse().unwrap()).collect(); //3-12 -> [3,12]
            let occurences: i32 = context.matches(&character).count().try_into().unwrap();
            // creates an iterator by filtering out all the characters different from the specific character
            // count the elements of this iteratpr
            // attempt the conversion from usize into i32. Since it is an attempt, it returns Result
            if (occurences >= range[0]) && (occurences <= range[1]) {
                counter += 1;
            }
        }
        return Ok(counter);
    }

    pub fn part2(file_name: &str) -> Result<i32, &'static str> {
        // Now range coontains the indexes (augmented by one) where we should see, at most once, the desired character
        // Recall that string indexing is not allowed in Rust because the loss of one-to-one correspondence between one character and one byte
        // We can build an iterator and access to the nth element of that iterator, indeed we assume to have just ascii characters
        let content: String = super::fs::read_to_string(file_name).unwrap();
        let v: Vec<_> = content.lines().map(|x| x.split_whitespace()).collect();
        let mut counter: i32 = 0;
        for value in v {
            let sub_vec: Vec<&str> = value.collect();
            let (indexes, character, context): (Vec<i32>, String, &str) = (
                sub_vec[0].split("-").map(|x| x.parse().unwrap()).collect(),
                sub_vec[1].replace(":", ""),
                sub_vec[2],
            );
            let string_iterator = context.chars(); // Recall that .count() is the "length" of the iterator
            if &context.len() != &string_iterator.clone().count() {
                //We're cloning the iterator because every iterator method requires the variablel to be mutable and we don't want that our iterator mutates
                return Err("Probably not ASCII type");
            }
            let first_match = &string_iterator
                .clone()
                .nth((indexes[0] - 1).try_into().unwrap())
                .unwrap()
                .to_string();
            let second_match = &string_iterator
                .clone()
                .nth((indexes[1] - 1).try_into().unwrap())
                .unwrap()
                .to_string();
            // We are unwrapping twice:
            // the first unwrap is in order to have a i32 to pass as argument of nth()
            // nth returns an option accessing to the nth position of the iterator (it might be out of bound), that's why it returns a Option
            // since in the good case nth() returns Some(char), we convett the char into a string via .to_string() (recall that it is equal to String::from(char))
            if (first_match == &character && second_match != &character)
                || (first_match != &character && second_match == &character)
            {
                counter += 1;
            }
        }
        return Ok(counter);
    }
}

pub mod day3 {
    pub fn part1(file_name: &str, plus_index: &i32, jump: &i32) -> Result<i32, &'static str> {
        let content: String = super::fs::read_to_string(file_name).unwrap();
        let v: Vec<&str> = content.lines().collect();
        // slope of 3-right-1-down is equivalent to 3-right of the next element of the vector.
        //iterating over the vector, we need to check what's the character of the +3th position of the next vector
        let plus_index: usize = TryInto::<usize>::try_into(*plus_index).unwrap();
        let mut next_iten_index: usize = 0;
        let mut next_index: usize = 0; //since you're starting from the top left corner
        let mut counter: i32 = 0;
        for _el in &v {
            //we stop if we are going to get an out of bound -> recall that if the index is equal to the length of the vector you're out of bound
            //Recall that we're jumping from where we jumped in the previous cycle, we do not iterate naively over the vector
            next_iten_index += TryInto::<usize>::try_into(*jump).unwrap();
            if next_iten_index >= v.len() {
                break;
            }
            let next_item = v[next_iten_index];
            next_index += plus_index; //index of the character of the next_item
            if next_index >= next_item.len() {
                //We capture the out of bound case (referred to next_item)
                next_index -= next_item.len(); //the next_index will be the difference of before that makes us out of bound (recall that if next_index==next_item.len() -> next_index will be 0)
            }
            //We check if we're allowed to index over the string thanks to the absence of ASCII characters>1 byte
            if next_item.len() != next_item.chars().count() {
                return Err("Bad string");
            }
            //Recall that .nth() is valid with iterators,
            let jumped_element = next_item.chars().nth(next_index).unwrap();
            //We've taken the element: is it a square or a tree?
            if jumped_element == '#' {
                //Recall that "" is for chars while '' is for &str
                counter += 1;
            }
        }
        return Ok(counter);
    }

    pub fn part2(file_name: &str) -> Result<u128, &'static str> {
        let mut cumulative_prod: u128 = 1;
        for right in (1..=7).step_by(2) {
            //We put equal in order to include the right extremum
            let ith_result: u128 = part1(file_name, &right, &1).unwrap().try_into().unwrap();
            cumulative_prod *= ith_result;
        }
        let last_result: u128 = part1(file_name, &1, &2).unwrap().try_into().unwrap();
        return Ok(last_result * cumulative_prod);
    }
}

pub mod day4 {

    pub fn part1(file_name: &str) -> Result<usize, &'static str> {
        const MANDATORY_KEYS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        // let content = include_str!("../inputDay4.txt"); //Fastest way to include a string: it works at compile time so we can't put a string variable, jsut a literal string (=hardcoding string)
        let content = super::fs::read_to_string(file_name).unwrap();
        let final_documents = content
            .split("\r\n\r\n") //we split according to each blank line
            .map(
                |fields| {
                    fields
                        .split_ascii_whitespace() //now we have a map of key:value
                        .map(
                            |field| {
                                field
                                    .split(":") //we divide key and value
                                    .next() // we take the first element (this would be equal to [0])
                                    .unwrap()
                            }, //Since next() returns option
                        )
                        .collect::<super::HashSet<_>>()
                }, //we arrange all the keys into a hashset
            )
            .filter(
                |passport| //return iterator whose elements satisifes the predicate
            MANDATORY_KEYS
            .iter()
            .all( // returns true if all the elements of the iterator satisfy the predicate 
                |item|
                passport.contains(item)
            ),
            )
            .count();
        return Ok(final_documents);
    }

    fn validate_field(key: &str, value: &str) -> Result<bool, &'static str> {
        //result returning boolean value or literal string
        //Each arm returns true
        let eye_color: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match key {
            "byr" => {
                let val: i32 = value.parse().unwrap();
                return Ok(val >= 1920 && val <= 2002);
            }
            "iyr" => {
                let val: i32 = value.parse().unwrap();
                return Ok(val >= 2010 && val <= 2020);
            }
            "eyr" => {
                let val: i32 = value.parse().unwrap();
                return Ok(val >= 2020 && val <= 2030);
            }
            "hgt" => {
                if value.contains("cm") {
                    let val: usize = value
                        .replace("cm", "")
                        .parse::<i32>()
                        .unwrap()
                        .try_into()
                        .unwrap(); //we use usize because wrapping_sub is a usize method
                    return Ok(val.wrapping_sub(150) <= 43); //we convert 150<val<193 into val-150<193-150:
                } else if value.contains("in") {
                    let val: usize = value
                        .replace("in", "")
                        .parse::<i32>()
                        .unwrap()
                        .try_into()
                        .unwrap(); //we use usize because wrapping_sub is a usize method
                    return Ok(val.wrapping_sub(59) <= 17);
                } else {
                    return Ok(false);
                }
            }
            "hcl" => Ok(value.len() == 7),
            "ecl" => Ok(eye_color.iter().any(|x| x == &value)), //iterating over eye_color, at least one occurrence match with the
            "pid" => Ok(value.len() == 9),
            "cid" => Ok(true), //cid can be whatever, we don't care
            _ => Err("Unrecognizable key"),
        }
    }

    pub fn part2(file_name: &str) -> Result<usize, &'static str> {
        let mandatory_keys: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let content = super::fs::read_to_string(file_name).unwrap();
        let final_documents = content
            .split("\r\n\r\n")
            .map(
                |fields| {
                    fields //we iterate over blank spaces
                        .split_ascii_whitespace()
                        .map(|field| {
                            field
                                .split_once(":") //if the character is present more than once, it splits just the first occurence
                                .unwrap()
                        })
                        .collect::<super::HashMap<_, _>>()
                }, //we split keys and values and we wrap them into a hasmap (not a hashset)
            )
            .filter(|passport| //recall that we're iterating over hashmap
            mandatory_keys.iter().all(|item| //we're converting array -> iterator -> we iterate over each element making sure that the predicate returns true for each element
                passport.contains_key(item) //we check that the mandatory keys are all present
            )) //so far we checked the valid passports like part1
            .filter(|passport| {
                //Now we check the integrity of each field: we use all() and a match structure
                return passport.iter().all(
                    |(key,value)| //recall that we're iterating over the hashmap, so we use the tuple (not two separated variables)
                validate_field(key,value).unwrap(),
                );
            })
            .count();
        return Ok(final_documents);
    }
}

pub mod day5 {
    // #[derive(Debug)]
    // struct Seat{
    //     row:i32,
    //     col:i32,
    //     id:i32
    // }

    pub fn part1(file_name: &str) -> Result<i32, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        let max_seat_id = &content
            .lines()
            .map(|b_p| {
                let mut cap_row = 127;
                let mut start_row = 0;
                let mut cap_col = 7;
                let mut start_col = 0;
                let charz = b_p.chars();
                for c in charz {
                    match c {
                        //We take the median value
                        'B' => start_row = (start_row + cap_row + 1) >> 1,
                        'F' => cap_row = (start_row + cap_row + 1) >> 1,
                        'R' => start_col = (start_col + cap_col + 1) >> 1,
                        'L' => cap_col = (start_col + cap_col + 1) >> 1,
                        _ => (),
                    }
                }
                let id = (cap_row.min(start_row) << 3) + cap_col.min(start_col);
                // let _seat = Seat{
                //     row:cap_row.min(start_row),
                //     col:cap_col.min(start_col),
                //     id
                // };
                return id;
            })
            .max()
            .unwrap();
        return Ok(*max_seat_id);
    }

    pub fn part2(file_name: &str) -> Result<i32, &'static str> {
        //We order seat ids and we check wheteher ther is a jump in the list
        let content = super::fs::read_to_string(file_name).unwrap();
        let mut seat_ids = content
            .lines()
            .map(|b_p| {
                let mut cap_row = 127;
                let mut start_row = 0;
                let mut cap_col = 7;
                let mut start_col = 0;
                let charz = b_p.chars();
                for c in charz {
                    match c {
                        //We take the median value
                        'B' => start_row = (start_row + cap_row + 1) >> 1,
                        'F' => cap_row = (start_row + cap_row + 1) >> 1,
                        'R' => start_col = (start_col + cap_col + 1) >> 1,
                        'L' => cap_col = (start_col + cap_col + 1) >> 1,
                        _ => (),
                    }
                }
                let id = (cap_row.min(start_row) << 3) + cap_col.min(start_col);
                return id;
            })
            .collect::<Vec<_>>();
        //Recall that sort_by doesn't return anything and the self argument is taken as mutable;
        seat_ids.sort_by(|a, b| a.cmp(&b));
        //Now the array seat_ids is sorted
        for (i, elem) in seat_ids.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if *elem != seat_ids[i - 1] + 1 {
                return Ok(*elem - 1); //586...588
            }
        }
        return Err("Seat not found!");
    }
}

pub mod day6 {

    pub fn part1(file_name: &str) -> Result<i32, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        let total_num_quest: i32 = content
            .split("\r\n\r\n")
            .map(|x| {
                // Recall that hashset is a collection that eliminates duplicates -> we create a hashet from chars()
                let answers = x
                    .replace("\r\n", "")
                    .chars()
                    .collect::<super::HashSet<_>>()
                    .iter()
                    .count();
                return TryInto::<usize>::try_into(answers).unwrap();
            })
            .sum::<usize>()
            .try_into()
            .unwrap();
        return Ok(total_num_quest);
    }

    pub fn part2(file_name: &str) -> Result<usize, &'static str> {
        //Now it's about counting the number of words that are present in each line of a given block -> this is equivalent to say how many question have been answered with "yes" by each member of a given group
        let content = super::fs::read_to_string(file_name).unwrap();
        let total_num_quest: usize = content
            .split("\r\n\r\n") //We create an array of groups;
            .map(|x| {
                let raw_answers = x.split_ascii_whitespace(); //we take each line of a given group
                let people = raw_answers.count(); //we count the number of lines
                let mut big_counter: super::HashMap<char, usize> = super::HashMap::new(); //we init a table for counting same characters in a given group (not line!)
                for c in x.replace("\r\n", "").chars() {
                    //we avoid to count ascii whitespaces
                    let particular_counter = big_counter.entry(c).or_insert(0); //we take the value associated to the character in the mapping, if nothing is present we initialize it with zero
                                                                                //By de-referencing the particular counter, we are able to update that value
                    *particular_counter += 1;
                }
                //Now we filter out all the keys (=questions) whose value is different from the numebr of ppl (=number of lines)
                return big_counter.values().filter(|x| *x == &people).count();
            })
            //Each item in this array is the number of same questions answered affermatively by each memeber of the group, now we sum them
            .sum();
        return Ok(total_num_quest);
        // return total_num_quest;
    }
}

pub mod day7 {

    //Via lazy static we are capable of defining "global" variables
    lazy_static::lazy_static! {
        //Understanding regex: https://regexr.com/

        //We define a regex rule: it returns an Option because you might write wrongly the regex rule
        //the content of every regex rule is inside r#" "#
        //Our rule is set to be a string with these three pieces:
        // ([a-z ]+) means at least one (or more) character (including space) -> recall that + means at least one; ? means at most one; * zero or more
        // "bags contains" must be included
        // (.*)$ everything until the end of the line
        static ref RE_RULE:super::Regex=super::Regex::new(r#"([a-z ]+) bags contain (.*)$"#).unwrap();
        //The follwoing rule instead:
        // (\d) means exactly one digit number [0-9]
        // ([a-z ]+) means at least one (or more) character (including space) -> recall that + means at least one; ? means at most one; * zero or more
        // b means exactly one char 'b'
        static ref RE_CONT: super::Regex = super::Regex::new(r#"(\d) ([a-z ]+) b"#).unwrap();
        //Recall that the 0 element is always the full string while 1, 2, ...n is the expression contained respectively in the first, second, ...n-th brackets

    }

    pub fn part1(file_name: &str) -> Result<usize, &'static str> {
        // One strategy might split the string when "contain" occur
        // The tricky thing is that you must map also all the bags that can contain the shiny gold bag, because if such bags can be contained by other bags, then such bags can contain the shiny gold bag
        let content = super::fs::read_to_string(file_name).unwrap();
        let mut rules_count: super::HashSet<_> = super::HashSet::new();
        let mut examined: super::HashSet<_> = super::HashSet::new();
        let mut next_bag = "shiny gold"; //occurences are mixed between "bag" and "bags": better filtering just according to the color
        loop {
            //equivalent of while true
            let new_list = content
                .lines()
                .filter(|x| {
                    x.split("contain")
                        .nth(1)
                        .unwrap()
                        .contains(&next_bag.replace("bags", "").replace("bag", ""))
                })
                .map(|x| x.split("contain").nth(0).unwrap().trim());
            new_list.for_each(|x| {
                rules_count.insert(x);
            }); //we put the curlies in order to convert the predicate into a function
            let local = examined.clone(); //rust doesn't like temporaries (=references to functions), better defining a local variable
            let mut bags_left = rules_count.difference(&local);
            if bags_left.clone().collect::<super::HashSet<_>>().is_empty() {
                break;
            }
            examined.insert(next_bag); //when you incurr in mutable-immutable reference conflict, drop the immutable reference and clone the item (is there a more efficient solution for that?)
            next_bag = bags_left.next().unwrap();
        }
        return Ok(rules_count.iter().count());
    }

    // Since "shiny gold": {"bright tomato": 3, "mirrored maroon": 4, "bright beige": 4, "dull crimson": 3}
    // where "bright tomato": {"vibrant red": 3, "muted gray": 3}
    // you have
    // 1 + 3*(3*(...)+3*(...)) + 4*(...) + 4*(...) + 3*(...)

    pub fn parse_bag(rule: &str) -> (&str, Vec<(&str, usize)>) {
        //Given the bag, it returns a double with the bag and a vector containing included bags and number
        let captures = RE_RULE.captures(rule).unwrap(); // -> it returns an Option (Some(content) if the string matches with the regex, None else)
                                                        //captures.get(0) correspond to the full string
                                                        //captures.get(1) correspond to ([a-z ]+), so the type of bag
                                                        //captures.get(2) correspond to (.*)$ so the bags included in the current bag

        let current_col = captures.get(1).unwrap().as_str(); //we take the current bag (first piece)
        let correspondence: Vec<(&str, usize)> = RE_CONT
            //We apply RE_CONT on the second element, that is (.*)$, so all the bags that are included
            //The char 'b' at the end of the regex rule is to drop the word "bag";
            //every line that contains "no other bag" is dropped because .captures() would return None
            .captures_iter(
                //returns an iterator
                captures.get(2).unwrap().as_str(), //second piece
            )
            .map(|cond| {
                let num = cond[1].parse().unwrap(); //regex doesn't have method "parse"
                let color = cond.get(2).unwrap().as_str();
                return (color, num);
            })
            .collect(); //Since we're returning a vector

        return (current_col, correspondence);
    }

    pub fn bags(color: &str, rules: &super::HashMap<&str, Vec<(&str, usize)>>) -> usize {
        let res = 1 + rules[color]
            .iter() //we convert the vector (color, number) into an iterator
            .map(|(col, count)| bags(col, rules) * count) //we check the bag until is possible
            .sum::<usize>();
        return res;
    }

    pub fn part2(file_name: &str) -> Result<usize, &'static str> {
        //NESTED HASMPAS ARE EVIL: BETTER HASMAP WHERE VALUES ARE VECTOR OF TUPLES
        let content = super::fs::read_to_string(file_name).unwrap();

        let correspondences: super::HashMap<_, _> = content
            .lines()
            .map(parse_bag) //wre don't use the brackets as an alternative for .map(|x| parse_bag(x))
            //Now we have an iterator of double (bag, Vector<included bag, amount>)
            //It is now easy to convert such iterator in a HashMap where the key is the bag and the value is Vector<included bag, amount>
            .collect();

        let number_of_contained_bags = bags("shiny gold", &correspondences) - 1;

        return Ok(number_of_contained_bags);
    }
}

pub mod day8 {
    lazy_static::lazy_static! {
        static ref RE:super::Regex = super::Regex::new(r#"([a-z]+) (.*)$"#).unwrap();
    }

    pub fn op(
        parsed_cont: &Vec<(&str, i32)>,
        i: i32,
        acc: i32,
        hashset_counter: &mut super::HashSet<i32>,
        mut it: i32,
    ) -> (i32, i32, i32) {
        //we take the mutable reference for the hashset because we need to update the hashset registering new indexes
        it += 1;
        if !hashset_counter.insert(i) || parsed_cont.iter().len() == i.try_into().unwrap() {
            //insert returns false if the element was in the hashset
            //in the second case we exit peacefully because we reached the bottom of our list
            return (acc, i, it);
        }
        let el = parsed_cont.iter().nth(i.try_into().unwrap()).unwrap();
        let (opcode, value) = (el.0, el.1);
        match opcode {
            "acc" => op(parsed_cont, i + 1, acc + value, hashset_counter, it), //we increase the index by 1, and we increase the accumulator by the value associated
            "jmp" => op(parsed_cont, i + value, acc, hashset_counter, it), //we increase the index by value, and we DO NOT increase the accumulator
            "nop" => op(parsed_cont, i + 1, acc, hashset_counter, it), //we increase the index by 1, and we DO NOT increase the accumulator
            _ => todo!(),                                              //will never be called;
        }
    }

    pub fn part1(file_name: &str) -> Result<i32, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        let parsed_cont: Vec<(&str, i32)> = content
            .lines()
            .map(|x| {
                //we use i32 because we're dealing with negative numbers
                let line = RE.captures(x).unwrap();
                let (opcode, value): (&str, i32) =
                    (line.get(1).unwrap().as_str(), line[2].parse().unwrap()); //recall that regex::Match<'_> doesnt have the parse method and that we have to convert it as string
                return (opcode, value);
            })
            .into_iter()
            .collect(); //we create a vector of doubles, containing opcode and vaalue associated
                        //Now we use the fold method from an iterator
        let mut hashset_counter: super::HashSet<i32> = super::HashSet::<i32>::new();
        //this function will be self-invoked in the match structure until we meet an index that is registered in the hashset, in that case it returns the accumulator.
        let (res, index, it) = op(&parsed_cont, 0, 0, &mut hashset_counter, 0);
        println!(
            "Last index of the first cycle was {};\nExited after {} iterations;",
            index, it
        );
        return Ok(res);
    }

    pub fn part2(file_name: &str) -> Result<i32, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        //We can bruteforce the change from nop to jmp (and viceversa) resulting in a fail if we start the loop again
        //we start from listing how many jmp and nop are present
        //We create an iterator that tells us what is the position on jmps and nops
        let mut ind = vec![];
        let parsed_cont: Vec<(&str, i32)> = content
            .lines()
            .enumerate()
            .map(|(i, x)| {
                //we use i32 because we're dealing with negative numbers
                let line = RE.captures(x).unwrap();
                let (opcode, value): (&str, i32) =
                    (line.get(1).unwrap().as_str(), line[2].parse().unwrap()); //recall that regex::Match<'_> doesnt have the parse method and that we have to convert it as string
                if opcode == "jmp" || opcode == "nop" {
                    ind.push(i);
                }
                return (opcode, value);
            })
            .into_iter()
            .collect();
        //now we iterate over each index changing jmp into nop and vice versa: if the finale index matches with the last index of parsed_cont we get out from the loop
        let exit_ind = parsed_cont.len();
        let mut result = 0;
        for i in ind {
            let mut parsed_cont_it_mut = parsed_cont.clone();
            let el = parsed_cont_it_mut[i]; //nth(i.try_into().unwrap()).unwrap(); //recall that all the preceeding elements and also the selected one will be discarded from teh iterator, so that's why you need it as mutable
            match el.0 {
                "jmp" => {
                    let tmp = ("nop", el.1);
                    parsed_cont_it_mut[i] = tmp;
                }
                "nop" => {
                    let tmp = ("jmp", el.1);
                    parsed_cont_it_mut[i] = tmp;
                }
                _ => todo!(),
            };
            let mut hashset_counter: super::HashSet<i32> = super::HashSet::<i32>::new();
            let (res, index, _it) = op(&parsed_cont_it_mut, 0, 0, &mut hashset_counter, 0);
            // println!("Result {}; Last index of the first cycle was {};Exited after {} iterations;",res,index,it);
            if index == exit_ind.try_into().unwrap() {
                result = res;
                println!("Changed {} at line {}", el.0, i + 1);
                break;
            }
        }
        return Ok(result);
    }
}

pub mod day9 {
    pub fn abs_diff(a: &usize, b: &usize) -> usize {
        if a > b {
            return a - b;
        } else {
            return b - a;
        }
    }

    pub fn part1(file_name: &str) -> Result<usize, &'static str> {
        let content: String = super::fs::read_to_string(file_name).unwrap();
        let values: Vec<usize> = content.lines().map(|x| x.trim().parse().unwrap()).collect();
        // We might iterate over the numbers, frozing the current one and vectorizing the previous 25 numbers.
        // Then, we iterate over this vector subtracting the i-th value of the vector and checking if such value is present in the vector, caring that it is not equal to the i-th value
        // Indeed recall that el = x + ? => ? = el-x
        // to do so, we need to start the iteration from index 24
        let preamble_length = 25;
        for (i, el) in values.iter().enumerate() {
            if i < preamble_length {
                continue;
            }
            let subv = &values[i - preamble_length..i]; //recall that right extremum is not included
            let ok = subv.iter().any(|x| {
                let dif = abs_diff(x, el);
                return subv.contains(&dif) && &dif != el;
            });
            if !ok {
                return Ok(*el);
            }
        }
        return Err("Number not found!");
    }

    pub fn part2(file_name: &str) -> Result<usize, &'static str> {
        // Now we need to iterate over the full list and find those contiguous numbers whose sum is the faulty number we got in part1
        //The range is defined by all those values such as the contiguous sum is the faulty number
        // We need to return the sum between the max and min (extrema included) of such subrange
        let content = super::fs::read_to_string(file_name).unwrap();
        let values: Vec<usize> = content.lines().map(|x| x.trim().parse().unwrap()).collect();
        let faulty_num = part1(file_name).unwrap();
        //The problem with this is that it doesn't return the right extrema of the subrange
        for (i, el) in values.iter().enumerate() {
            let start_el = el;
            let mut end_el: usize = 0;
            let res = values[i..].iter().fold(0, |acc, x| {
                if acc < faulty_num {
                    end_el = *x; //it will mutate until the end
                    acc + x
                } else {
                    acc
                }
            });
            if res == faulty_num {
                let start = values.iter().position(|x| x == start_el).unwrap();
                let end = values.iter().position(|x| x == &end_el).unwrap();
                let res = values[start..=end].iter().min().unwrap()
                    + values[start..=end].iter().max().unwrap();
                return Ok(res);
                //Define the range with these two numbers and return their sum
            }
        }
        return Err("Numbers not found!");
    }
}

pub mod day10 {
    use core::num;
    use std::hash;

    use itertools::Itertools;

    pub fn part1(file_name: &str) -> Result<usize, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        let mut numbers: Vec<usize> = content.lines().map(|x| x.trim().parse().unwrap()).collect();
        //We need to sort such list, creating a HashMap in order to track jumps and then returning the priduct between 1 and jumps
        //Recall that sort() requires the vector to be mutable;
        numbers.sort();
        let mut counter = super::HashMap::<usize, usize>::new();
        numbers.iter().enumerate().for_each(|(i, x)| {
            let dif = if i > 0 { x - numbers[i - 1] } else { *x };
            let c = counter.entry(dif).or_insert(0);
            *c += 1;
        });
        //We hardcode another jump of 3 since our device adapter is +3 the highest one
        let c = counter.entry(3).or_insert(0);
        *c += 1;
        let res = counter.get(&3).unwrap() * counter.get(&1).unwrap();
        return Ok(res);
    }

    fn rope_algo(
        compare: &Vec<usize>,
        mut new_vec: &Vec<Vec<usize>>,
        cap: &usize,
    ) -> Vec<Vec<usize>> {
        let mut next_vec: Vec<Vec<usize>> = vec![];
        for el in new_vec.clone().iter() {
            let last_el = el.clone().pop().unwrap();
            let plus = [last_el + 1, last_el + 3];
            //now we append vectors in new_vec...
            if *cap >= plus[0].min(plus[1]) && plus.iter().any(|x| compare.contains(&x)) {
                for x in plus {
                    if compare.contains(&x) {
                        let mut tmp_el = el.clone();
                        tmp_el.push(x);
                        next_vec.push(tmp_el);
                    }
                }
            }
        }
        println!("{:?}", next_vec.len());
        return match next_vec.is_empty() {
            true => return new_vec.to_vec(),
            false => rope_algo(compare, &next_vec, cap),
        };
    }

    fn progressive_sum(
        hashmap: &super::HashMap<usize, Vec<usize>>,
        correspondences: &Vec<usize>,
    ) -> (usize, usize) {
        //(sum, value for the next call)
        //If there is at least one element in the array that has a length greater than 1, we re-invoke the function with the flattened array
        let mut next_vec: Vec<usize> = vec![];
        if correspondences.len() == 1
            || correspondences
                .iter()
                .all(|x| hashmap.get(x).unwrap().len() == 1)
        {
            return (
                correspondences.len(),
                *correspondences.iter().max().unwrap(),
            );
        }
        for el in correspondences {
            let values = hashmap.get(el).unwrap();
            if values.len() == 1 {
                next_vec.push(*el);
            } else {
                values.iter().for_each(|x| next_vec.push(*x));
            }
        }
        return progressive_sum(hashmap, &next_vec);
    }

    pub fn part2(file_name: &str) -> Result<usize, &'static str> {
        let content = super::fs::read_to_string(file_name).unwrap();
        let mut numbers: Vec<usize> = content.lines().map(|x| x.trim().parse().unwrap()).collect();
        // We include also the floor and the cap of the list
        numbers.push(0);
        numbers.sort();
        let cap = numbers.clone().pop().unwrap() + 3;
        numbers.push(cap);

        // now we need to define all the possible arrangements (diff must be 1 or 3)

        // law is a_n = {n+1||n+3}
        // given the length of the vector as n, we have n positions where we can inser either -1 or -3
        // we could create a big vector cotaining all the possible arrangements of [-1, ..., -1] [-3, ..., -3]
        // then we could subtract the starting array with each of these arrays, dropping duplicates from results, and then checking if all the elements are present in the starting array

        //... Or we might simply create the powerset of our previous vector: but this will return in a capacity overflow problem
        //We might solve it via a "rope" system (I imagine a stick figure swinging from number to number): it might work but it is super slow

        //What can we do? I'm having bad headache

        //My solution is to track in a hashmap value -> neighborhood_of_value where "neighborhood_of_value" is the vector of values reachable from value (because they are value+1 or value+3)
        //The total number of unique paths is the production of intermediate sums (where "intermediate sums" stands for the length of the neighborhood, including "nested neighborhhods").
        // Let f(n) = length of the neighborhood of the value n
        // You continue with the next element of the production if you have "flattened" all the previous neighborhoods, meaning that considering A the subset [0,n], you have that f(i) for every i belonging to A
        //Suppose for example that you have the following hashmap of neighbors:
        // {10: [11, 12], 19: [22], 6: [7], 7: [10], 4: [5, 6, 7], 5: [6, 7], 11: [12], 22: [], 12: [15], 1: [4], 0: [1], 16: [19], 15: [16]}
        //Independent paths = f(0) * f(1) * f(4)* ...
        // = f(0) * f(1) * (f(5)+f(6)+f(7))* ...
        // = f(0) * f(1) * ((f(6)+f(7))+f(6)+f(7))* ...
        // = f(0) * f(1) * ((f(6)+f(7))+f(6)+f(7))* f(10) * ...
        // = f(0) * f(1) * ((f(6)+f(7))+f(6)+f(7))* (f(11)+f(12)) * ...
        // = f(0) * f(1) * ((f(6)+f(7))+f(6)+f(7))* (f(11)+f(12)) * f(15) * f(16) * f(19)
        // = 1 * 1 * 4 * 2 * 1 * 1 * 1 = 8

        let mut hashmap: super::HashMap<usize, Vec<usize>> = super::HashMap::new();

        let jumps = [1, 2, 3];
        for el in numbers.iter() {
            let mut l: Vec<usize> = vec![];
            for j in jumps {
                if numbers.contains(&(el + j)) {
                    l.push(el + j);
                }
            }
            hashmap.entry(*el).or_insert(l);
        }
        let mut next_val: usize = 0;
        let mut counter: usize = 1;

        while next_val != cap {
            let res = progressive_sum(&hashmap, hashmap.get(&next_val).unwrap());
            counter = counter * res.0;
            next_val = res.1;
        }

        return Ok(counter);
    }
}
