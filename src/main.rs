mod markov;
mod history;

use history::History;
use markov::Markov;

pub type HistoryType = char;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut hist = History::new(20);
    let mut markov = Markov::new();
    let mut correct = 0;
    let mut wrong = 0;

    let text: Vec<_> = std::fs::read(&args[1]).unwrap().into_iter()
        .map(|c| c as char)
        .collect();
    // let text_str = std::fs::read_to_string(&args[1]).unwrap();
    // let text = text_str.chars();

    for c in text {
        let prediction = markov.predict(&hist);
        let prediction_trimmed: Vec<_> = prediction.into_iter().take(2).collect();
        // println!("{} ({}) -> {:?}", hist, c as char, prediction_trimmed);
        match prediction_trimmed.get(0) {
            Some((pc, _val)) => match *pc == c {
                true => correct += 1,
                false => wrong += 1,
            }
            None => {}
        }
        markov.train(&hist, c);
        hist.push(c);
    }
    println!("Correct {}, wrong {}", correct, wrong);
}
