mod markov;
mod history;
mod tuplemap;

use history::History;
use markov::Markov;

pub type HistoryType = char;


fn main() {
    println!("{}", std::mem::size_of::<std::collections::HashMap<char, f64>>());
    
    let args: Vec<String> = std::env::args().collect();
    
    let mut hist = History::new(20);
    let mut markov = Markov::new();
    let mut correct = 0;
    let mut wrong = 0;

    let fname = match args.get(1) {
        Some(f) => f,
        None => "data/enwik5",
    };

    let text: Vec<_> = std::fs::read(fname).unwrap().into_iter()
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
        // match prediction_trimmed.get(0) {
        //     Some((pc, _val)) => match *pc == c {
        //         true => print!("1"),
        //         false => print!("0"),
        //     }
        //     None => {}
        // }

        markov.train(&hist, c);
        hist.push(c);
    }
    println!("Correct {}, wrong {}", correct, wrong);
    for (f, occs) in markov.get_entry_occs() {
        println!("Frequency {} happened {} times", f, occs);
    }
    for (l, occs) in markov.get_entry_lens() {
        println!("Len {} happened {} times", l, occs);
    }

}
