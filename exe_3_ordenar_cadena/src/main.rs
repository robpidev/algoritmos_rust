fn order(sentence: &str) -> String {
    let mut words = sentence.split(" ").collect::<Vec<&str>>();

    let mut word_change;

    for i in 0..words.len() {
        for (j, word) in words.clone().into_iter().enumerate() {
            if word.contains(&i.to_string()) {
                word_change = words[i];
                words[i] = words[j];
                words[j] = word_change;
            }
        }
    }

    words.join(" ")
}

fn main() {
    println!("{}", order("is2 thi1s t4est 3a"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 thi1s t4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
