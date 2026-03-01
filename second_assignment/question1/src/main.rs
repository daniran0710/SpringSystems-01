fn most_frequent_word(text: &str) -> (String, usize) {
    let mut words: Vec<&str> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    for w in text.split_whitespace() {
        let mut found_index: Option<usize> = None;
    
        for i in 0..words.len(){
            if words[i] == w{
                found_index = Some(i);
                break;
            }
        }

        match found_index {
            Some(i) => {
                let c = &mut counts[i];
                *c += 1;
            }

            None=> {
                words.push(w);
                counts.push(1);
            }
        }
    }
    let mut max_index = 0;
    for i in 1..counts.len() {
        if counts[i] > counts[max_index] {
            max_index = i;
        }
    }
    (words[max_index].to_string(), counts[max_index])
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}