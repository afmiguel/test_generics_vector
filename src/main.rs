// Unify the two functions into a generic one
fn first_i32(list: &Vec<i32>) -> &i32 {
    list.get(0).expect("The list is empty")
}
fn first_string(list: &Vec<String>) -> &String {
    list.get(0).expect("The list is empty")
}

fn main() {
    let numbers = vec![1, 2, 3];
    let first_number = first_i32(&numbers);
    println!("First number: {:?}", first_number);

    let words = vec!["hello".to_string(), "world".to_string()];
    let first_word = first_string(&words);
    println!("First word: {:?}", first_word);
}
