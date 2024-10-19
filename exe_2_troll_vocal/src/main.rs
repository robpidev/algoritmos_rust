// Trolls are attacking your comment section!
//A common way to deal with this situation is to
//remove all of the vowels from the trolls'
//comments, neutralizing the threat.
//Your task is to write a function that
//takes a string and return a new string with all vowels removed.
//For example, the string "This website is for losers LOL!"
//would become "Ths wbst s fr lsrs LL!".
//Note: for this kata y isn't considered a vowel.

fn disemvowel(s: &str) -> String {
    let mut result = "".to_string();
    for c in s.chars() {
        if !"aeiouAEIOU".contains(c) {
            result += &c.to_string();
        }
    }

    result
}

// Solucion 2
fn disemvowel_2(s: &str) -> String {
    s.chars()
        .map(|c| {
            if !"aeiouAEIOU".contains(c) {
                c.to_string()
            } else {
                "".to_string()
            }
        })
        .collect()
}

// Solucion 3
fn disemvowel_3(s: &str) -> String {
    s.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect()
}

fn main() {
    println!("{}", disemvowel("This website is for losers LOL!"));
    println!("{}", disemvowel_2("This website is for losers LOL!"));
    println!("{}", disemvowel_3("This website is for losers LOL!"))
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
