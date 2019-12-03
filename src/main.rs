mod arcoder;
mod markov;
mod history;

use history::History;
use markov::Markov;

pub type HistoryType = char;

const HIST_LEN: usize = 16;

const PREDICT_TRIES: usize = 64;


fn main() {
    println!("{}", std::mem::size_of::<std::collections::HashMap<char, f64>>());
    
    let args: Vec<String> = std::env::args().collect();
    

    let fname = match args.get(1) {
        Some(f) => f,
        None => "data/enwik5",
    };

    let text: Vec<_> = std::fs::read(fname).unwrap().into_iter()
        .map(|c| c as u8)
        .collect();

    arcoder::compress(text, std::io::BufWriter::new(std::fs::File::create(format!("{}.arc", fname)).unwrap()));
    arcoder::decompress(std::io::BufReader::new(std::fs::File::open(format!("{}.arc", fname)).unwrap()),
            std::io::BufWriter::new(std::fs::File::create(format!("{}.drc", fname)).unwrap())
    );
    // let reader = std::io::BufReader::new(std::fs::File::open(fname).unwrap());
    // let text = reader.bytes();
    // let text_str = std::fs::read_to_string(&args[1]).unwrap();
    // let text = text_str.chars();

    // let mut hist = History::new(HIST_LEN);
    // let mut markov = Markov::with_capacity(text.len() * HIST_LEN / 64);
    // println!("Hashtable capacity: {}  ({} KB)", markov.get_capacity(), markov.get_capacity() * std::mem::size_of::<markov::MarkovValue>() / 1024);
    // // let mut markov = Markov::new();
    // let mut correct = [0; PREDICT_TRIES];
    // let mut wrong = 0;


    // for c in text {
    //     let prediction = markov.predict(&hist);
        
    //     match prediction.into_iter().take(PREDICT_TRIES).position(|(pc, _val):(u8, u32)| pc == c) {
    //         Some(i) => correct[i] += 1,
    //         None => wrong += 1,
    //     }

    //     // println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());
    //     markov.train(&hist, c);
    //     hist.push(c);
    // }
    // // println!();


    // println!("Correct {:?}, wrong {}", correct, wrong);
    // // for (f, occs) in markov.get_entry_occs() {
    // //     println!("Frequency {} happened {} times", f, occs);
    // // }
    // // for (l, occs) in markov.get_entry_lens() {
    // //     println!("Len {} happened {} times", l, occs);
    // // }

    // println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());

    // // // println!("{:?}", markov);

    // std::io::stdin().read_line(&mut String::new()).unwrap();

    // println!("MarkovValue size: {}", std::mem::size_of::<markov::MarkovValue<char>>());
    // println!("OptionVec size: {}", std::mem::size_of::<Option<Vec<char>>>());
    // println!("OptionBox size: {}", std::mem::size_of::<Option<Box<[char]>>>());
    // println!("BoxArray size: {}", std::mem::size_of::<Box<[char]>>());
    

}
    