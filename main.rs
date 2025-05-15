//Create a function that takes a string and returns the number of vowels in it
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|&c| "aeiouAEIOU".contains(c)).count()
}

fn main() {
    let text = "Bonjour, Rust!";
    let nb_voyelles = count_vowels(text);
    println!("Nombre de voyelles : {}", nb_voyelles);
}