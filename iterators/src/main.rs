fn main() {
    let sentence =  String:: from("hey, my name is kushagra");
    let first_word: String = get_first_word(sentence);

    let n: i32 = 1000;
    for i in 0..n {
        println!("Hello world!");
    }

    print!("First word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String { //always return a type
    let mut ans: String = String:: from("");
    for c in sentence.chars() {
        ans.push_str(c.to_string().as_str());
        if c == ' ' {
            break;
        }
    }
    return ans;
}
