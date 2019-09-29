mod markov;
mod history;

use history::History;
use markov::Markov;

pub type HistoryType = char;

const HIST_LEN: usize = 10;

const PREDICT_TRIES: usize = 4;


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
    let mut markov = Markov::with_capacity(HIST_LEN/4 * text.len());
    // let mut markov = Markov::new();
    let mut correct = [0; PREDICT_TRIES];
    let mut wrong = 0;


    for c in text {
        let prediction = markov.predict(&hist);
        
        match prediction.into_iter().take(PREDICT_TRIES).position(|(pc, _val):(char, u32)| pc == c) {
            Some(i) => correct[i] += 1,
            None => wrong += 1,
        }

        // println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());
        markov.train(&hist, c);
        hist.push(c);
    }
    // println!();


    println!("Correct {:?}, wrong {}", correct, wrong);
    // for (f, occs) in markov.get_entry_occs() {
    //     println!("Frequency {} happened {} times", f, occs);
    // }
    // for (l, occs) in markov.get_entry_lens() {
    //     println!("Len {} happened {} times", l, occs);
    // }

    println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());

    std::io::stdin().read_line(&mut String::new());

    println!("MarkovValue size: {}", std::mem::size_of::<markov::MarkovValue<char>>());
    println!("OptionVec size: {}", std::mem::size_of::<Option<Vec<char>>>());
    println!("OptionBox size: {}", std::mem::size_of::<Option<Box<[char]>>>());
    println!("BoxArray size: {}", std::mem::size_of::<Box<[char]>>());
    

}
    