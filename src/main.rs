mod markov;
mod history;

use history::History;
use markov::Markov;

pub type HistoryType = char;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut hist: History<u8> = History::new(3);
    let mut markov: Markov<u8> = Markov::<u8>::new();
    let mut correct = 0;
    let mut wrong = 0;

    let text = std::fs::read(&args[1]).unwrap();

    for c in text {
        let prediction = markov.predict(&hist);
        let prediction_trimmed: Vec<_> = prediction.into_iter().take(2).collect();
        // println!("{} ({}) -> {:?}", hist, c, prediction_trimmed);
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
