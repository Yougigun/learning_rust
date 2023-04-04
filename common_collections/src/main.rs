fn main() {
    // example to use vector
    println!("example to use vector");
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    println!("v = {:?}", v);
    println!("v[0] = {}", v[0]);
    println!("v[1] = {}", v[1]);

    // example to use Vec::new()
    println!("example to use Vec::new()");
    let mut v: Vec<i32> = Vec::new();
    // add element to vector
    v.push(3);
    println!("v = {:?}", v);

    // example to use string
    println!("example to use string");
    let mut s = String::from("Hello, ");
    s.push_str("world!");
    println!("s = {}", s);

    // example to use hash map
    println!("example to use hash map");
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);
    println!("scores[\"Blue\"] = {}", scores["Blue"]);
    println!("scores[\"Yellow\"] = {}", scores["Yellow"]);

    // example to use tree map
    println!("example to use tree map");
    use std::collections::BTreeMap;
    let mut scores = BTreeMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);
    println!("scores[\"Blue\"] = {}", scores["Blue"]);
    println!("scores[\"Yellow\"] = {}", scores["Yellow"]);

    // example of iterating over values in a vector
    println!("example of iterating over values in a vector");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // example of using an enum to store multiple types and iterating over enum values in a vector
    println!("example of using an enum to store multiple types");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row = {:?}", row);
    for i in &row {
        println!("{:?}", i);
    }

    // example of using sort_by() method to sort a vector of strings by length
    println!("example of using sort_by() method to sort a vector of strings by length");
    let mut v = vec!["hello", "world", "of", "rust"];
    v.sort_by(|a, b| a.len().cmp(&b.len()));

    // 8.2 Storing UTF-8 Encoded Text with Strings
    println!("8.2 Storing UTF-8 Encoded Text with Strings");
    // new a String object
    let mut s = String::new();
    s.push_str("Hello");
    // slice a string
    println!("slice a string");
    let s1 = &s[2..4];
    println!("s1 = {}", s1);

    // create a string from a string literal
    println!("create a String from a string literal");
    let s2 = "initial contents";
    let s2 = s2.to_string();
    println!("s2 = {}", s2);
    let s3 = "initial content".to_string();
    println!("s3 = {}", s3);

    // use String::from() to create a String from a string literal
    println!("use String::from() to create a String from a string literal");
    let s4 = String::from("initial contents");
    println!("s4 = {s4}");

    // update a String
    println!("update a String");
    // appending to a string with push_str and push
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    // appending string with another string
    let mut s6 = String::from("foo");
    let mut s7 = String::from("bar");
    s6.push_str(&s7);
    println!("s6 = {}", s6);
    // appending a single character with push
    let mut s8 = String::from("lo");
    s8.push('l');
    println!("s8 = {}", s8);

    // concatenate two strings with the + operator or the format! macro
    println!("concatenate two strings with the + operator or the format! macro");
    let s9 = String::from("Hello, ");
    let s10 = String::from("world!");
    let s11 = s9 + &s10;
    println!("s11 = {}", s11);
    // println!("s9 = {}", s9); // s9 is moved to s11
    println!("s10 = {}", s10);
    // format! macro
    let s12 = String::from("tic");
    let s13 = String::from("tac");
    let s14 = String::from("toe");
    let s15 = format!("{}-{}-{}", s12, s13, s14);
    println!("s12 = {}", s12); // s12 is not moved to s15
    println!("s13 = {}", s13); // s13 is not moved to s15
    println!("s14 = {}", s14); // s14 is not  moved to s15
    println!("s15 = {}", s15);

    // indexing into strings
    println!("indexing into strings");
    let s16 = String::from("hello");
    // let h = s16[0]; // error: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
    // println!("h = {}", h);
    let my_string = String::from("我是台灣人");
    let my_chars: Vec<char> = my_string.chars().collect();
    println!("first_char = {:?}", my_chars[0]);

    // internal representation
    println!("internal representation");
    let s17 = String::from("hello");
    println!("s17 = {}", s17);
    println!("s17.len() = {}", s17.len());
    let hello = "Здравствуйте";
    println!("hello = {}", hello);
    println!("hello.len() = {}", hello.len());
    // println!("hello = {}", &hello[0]); // cannot compile because rust prevents misunderstandings
    let hello = "Здравствуйте";
    // let s = &hello[0]; // cannot compile because the first byte in "З" is not a valid ASCII byte
    // println!("s = {}", s);

    // slicing strings
    println!("slicing strings");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s = {}", s);
    // let s = &hello[0..1]; // cannot compile because the first character was two bytes long
    // println!("s = {}", s);

    // Methods for Iterating Over Strings
    println!("Methods for Iterating Over Strings");
    // using chars() method to iterate over characters
    // output:
    // З
    // д
    for c in "Зд".chars() {
        println!("{}", c);
    }
    // using bytes() method to iterate over bytes
    // output:
    // 208
    // 151
    // 208
    // 180
    for b in "Зд".bytes() {
        println!("{}", b);
    }

    // Stings Are Not So Simple
    println!("Stings Are Not So Simple");
    // - Rust has chosen to make the correct handling of String data the default
    // behavior for all Rust programs, which means programmers have to put more
    // thought into handling UTF-8 data upfront.
    // - The good news is that the standard library offers a lot of functionality built off the String and &str types
    // to help handle these complex situations correctly.
    // hello world string. like contains, replace, split, etc.

    // 8.3 Storing Keys with Associated Values in Hash Maps
    println!("8.3 Storing Keys with Associated Values in Hash Maps");
    // create a new hash map
    println!("create a new hash map");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);
    // get value from hash map
    println!("get value from hash map");
    let team_name = String::from("Blue");
    let score1 = scores.get(&team_name); // return Option<&V>
    let score2 = scores.get(&("Yellow".to_string())); // return Option<&V>
    println!("score1 = {:?}", score1); // cannot compile because score1 is Option<&V>
    println!("score2 = {:?}", score2); // cannot compile because score1 is Option<&V>
    let score2_val = score2.copied().unwrap_or(0);
    println!("score2Val = {}", score2_val);
    // iterate over each key-value pair in a hash map
    println!("iterate over each key-value pair in a hash map");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Hash Maps and Ownership
    println!("Hash Maps and Ownership");
    // For types that implement the `Copy trait`, like i32, the values are `copied`
    // into the hash map. For `owned values` like String, the values will be
    // `moved` and the `hash map will be the owner of those values`.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("field_name = {}", field_name); // cannot compile because field_name is moved to map
    // println!("field_value = {}", field_value); // cannot compile because field_value is moved to map

    // If we insert references to values into the hash map, the `values won’t be moved into the hash map`.
    // The values that the references point to must be valid for at least as long as the hash map is valid.
    println!("insert references to values into the hash map");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("map = {:?}", map);
    println!("field_name = {}", field_name); // field_name is not moved to map
    println!("field_value = {}", field_value); // field_value is not moved to map

    // Updating a Hash Map
    println!("Updating a Hash Map");
    // Overwriting a Value
    println!("Overwriting a Value");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // insert Blue: 10
    scores.insert(String::from("Blue"), 25); // overwrite Blue: 10 to Blue: 25
    println!("scores = {:?}", scores);
    // Adding a Key and Value Only If a Key Isn’t Present
    println!("Adding a Key and Value Only If a Key Isn’t Present");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // Yellow does not exist, so this line inserts it
    scores.entry(String::from("Blue")).or_insert(50); // Blue already exists, so this line does nothing
    println!("scores = {:?}", scores);
    // Updating a Value Based on the Old Value
    println!("Updating a Value Based on the Old Value");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);

    // Summary
    // Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode
    // (the value that occurs most often; a hash map will be helpful here) of
    //  the list.
    println!("Given a list of integers, use a vector and return the median");
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (median, mode) = find_median_and_mode(&mut v);
    // Convert strings to pig latin. The first consonant of each word is
    //  moved to the end of the word and “ay” is added, so “first” becomes 
    // “irst-fay.” Words that start with a vowel have “hay” added to the 
    // end instead (“apple” becomes “apple-hay”). Keep in mind the details 
    // about UTF-8 encoding!
    println!("Convert strings to pig latin");
    let s = "hello world wonderful world";
    let pig_latin = convert_to_pig_latin(s);
    println!("pig_latin = {}", pig_latin);

    // Using a hash map and vectors, create a text interface to allow a user to 
    // add employee names to a department in a company. For example, “Add Sally
    // to Engineering” or “Add Amir to Sales.” Then let the user retrieve a 
    // list of all people in a department or all people in the company by 
    // department, sorted alphabetically.




}

// fn find median and mode of a vector
fn find_median_and_mode(v: &mut Vec<i32>) -> (i32, i32) {
    // sort the vector
    v.sort();
    // find median
    let median = v[v.len() / 2];
    // find mode
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (key, value) in &map {
        if *value > max_count {
            max_count = *value;
            mode = **key;
        }
    }
    (median, mode)
}

// fn convert string to pig latin
fn convert_to_pig_latin(s: &str) -> String {
    fn is_ascii_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
    let mut result = String::new();
    for word in s.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if first_char.is_ascii_alphabetic() {
            if is_ascii_vowel(first_char) {
                result.push_str(word);
                result.push_str("-hay");
            } else {
                result.push_str(&word[1..]);
                result.push('-');
                result.push(first_char);
                result.push_str("ay");
            }
        } else {
            result.push_str(word);
        }
        result.push(' ');
    }
    result
}

// fn add employee to a department
