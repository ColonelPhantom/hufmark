mod markov;
mod history;

use history::History;
use markov::Markov;

pub type HistoryType = char;

const HIST_LEN: usize = 10;


fn main() {
    println!("{}", std::mem::size_of::<std::collections::HashMap<char, f64>>());
    
    let args: Vec<String> = std::env::args().collect();
    

    let fname = match args.get(1) {
        Some(f) => f,
        None => "data/enwik5",
    };

    let text: Vec<_> = std::fs::read(fname).unwrap().into_iter()
        .map(|c| c as char)
        .collect();
    // let text_str = std::fs::read_to_string(&args[1]).unwrap();
    // let text = text_str.chars();

    let mut hist = History::new(HIST_LEN);
    let mut markov = Markov::with_capacity(HIST_LEN/2 * text.len());
    // let mut markov = Markov::new();
    let mut correct = 0;
    let mut wrong = 0;
    let mut unknown = 0;


    for c in text {
        let prediction = markov.predict(&hist);
        let prediction_trimmed: Vec<_> = prediction.into_iter().take(2).collect();
        // println!("{} ({}) -> {:?}", hist, c as char, prediction_trimmed);
        match prediction_trimmed.get(0) {
            Some((pc, _val)) => match *pc == c {
                true => correct += 1,
                false => wrong += 1,
            }
            None => unknown += 1,
        }
        // match prediction_trimmed.get(0) {
        //     Some((pc, _val)) => match *pc == c {
        //         true => print!("1"),
        //         false => print!("0"),
        //     }
        //     None => {}
        // }

        // println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());
        markov.train(&hist, c);
        hist.push(c);
    }
    // println!();


    println!("Correct {}, wrong {}, unknown {}", correct, wrong, unknown);
    // for (f, occs) in markov.get_entry_occs() {
    //     println!("Frequency {} happened {} times", f, occs);
    // }
    // for (l, occs) in markov.get_entry_lens() {
    //     println!("Len {} happened {} times", l, occs);
    // }

    println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());

    // std::io::stdin().read_line(&mut String::new());

}
