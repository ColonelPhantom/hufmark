use std::fs::File;
use std::io::{BufReader, BufWriter};
use arcode::bitbit::{BitReader, MSB, BitWriter};
use arcode::util::source_model::SourceModel;
use arcode::encode::encoder::ArithmeticEncoder;
use arcode::decode::decoder::ArithmeticDecoder;

use crate::{HIST_LEN, PREDICT_TRIES, History, Markov};

#[inline(never)]
pub fn compress(text: Vec<u8>, mut output: BufWriter<File>) {
    let decode_start = std::time::Instant::now();

    let num_symbols = 257 + crate::PREDICT_TRIES;
    let precision = 60;
    
    let mut encoder = ArithmeticEncoder::new(precision);
    let mut model = SourceModel::new(num_symbols as u32, 256);

    let mut hist = History::new(HIST_LEN);
    let mut markov = Markov::with_capacity(text.len() * HIST_LEN / 16);
    println!("Hashtable capacity: {}  ({} KB)", markov.get_capacity(), markov.get_capacity() * std::mem::size_of::<crate::markov::MarkovValue>() / 1024);
    // let mut markov = Markov::new();

    let mut out_writer = BitWriter::new(&mut output);
    for c in text {
        // Use context predictor
        //let symbol: u32 = c as u32;
    
        let prediction = markov.predict(&hist);
        
        let symbol = match prediction.into_iter().take(PREDICT_TRIES).position(|(pc, _val):(u8, u32)| pc == c) {
            Some(i) => 257 + i as u32,
            None => c as u32,
        };

        // println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());
        markov.train(&hist, c);
        hist.push(c);


        // Update arithmetic coder
        encoder.encode(symbol, &model, &mut out_writer).unwrap();
        model.update_symbol(symbol);
    }
    encoder.encode(model.get_eof(), &model, &mut out_writer).unwrap();
    encoder.finish_encode(&mut out_writer).unwrap();
    out_writer.pad_to_byte().unwrap();

    // for (f, occs) in markov.get_entry_occs() {
    //     println!("Frequency {} happened {} times", f, occs);
    // }
    // for (l, occs) in markov.get_entry_lens() {
    //     println!("Len {} happened {} times", l, occs);
    // }

    println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());

    // // println!("{:?}", markov);

    let finished = decode_start.elapsed().as_millis();
    println!("{} seconds", finished as f64 / 1000.00);
}

#[inline(never)]
pub fn decompress(input: BufReader<File>, mut output: BufWriter<File>) {
    let decode_start = std::time::Instant::now();

    let num_symbols = 257 + crate::PREDICT_TRIES;
    let precision = 60;
    
    let mut decoder = ArithmeticDecoder::new(precision);
    let mut model = SourceModel::new(num_symbols as u32, 256);

    let mut hist = History::new(HIST_LEN);
    // let mut markov = Markov::with_capacity( * HIST_LEN / 64);
    let mut markov = Markov::new();
    println!("Hashtable capacity: {}  ({} KB)", markov.get_capacity(), markov.get_capacity() * std::mem::size_of::<crate::markov::MarkovValue>() / 1024);
    // let mut markov = Markov::new();

    let mut input: BitReader<_, MSB> = BitReader::new(input);

    let mut out_writer = BitWriter::new(&mut output);
    while !decoder.is_finished() {
        // Use context predictor
        let symbol = decoder.decode(&model, &mut input).unwrap();

        let output = match symbol {
            0..=255 => symbol as u8,
            256 => break,
            _ => {
                assert!(symbol >= 257 && symbol < num_symbols as u32);
                let index = symbol - 257;
                markov.predict(&hist)[index as usize].0
            }
        };
    
        out_writer.write_byte(output).unwrap();
        
        // println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());
        markov.train(&hist, output);
        hist.push(output);


        // Update arithmetic coder
        model.update_symbol(symbol);
    }
    // for (f, occs) in markov.get_entry_occs() {
    //     println!("Frequency {} happened {} times", f, occs);
    // }
    // for (l, occs) in markov.get_entry_lens() {
    //     println!("Len {} happened {} times", l, occs);
    // }

    println!("Table pressure: {}/{}", markov.get_len(), markov.get_capacity());

    // // println!("{:?}", markov);

    let finished = decode_start.elapsed().as_millis();
    println!("{} seconds", finished as f64 / 1000.00);
}
