mod markov;
mod history;

type Predicted = char;


fn main() {
    let mut hist = history::History::new(3);
    for c in "Hello World!".chars() {
        hist.push(c);
        println!("{:?}", hist);
    }
}
