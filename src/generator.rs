use std::{env, fs::{self, read_to_string}};

use once_cell::sync::Lazy;

use crate::markov::MarkovChain;

/// Markov Chain that will do the generation. Encapsulated in `Lazy` to allow it to be global.
pub static GENERATOR: Lazy<MarkovChain> = Lazy::new(|| train_markov());

/// Makes necessary preperations and trains the Markov Chain
fn train_markov() -> MarkovChain {
    let home_dir = env::var("HOME").expect("HOME Environment Variable not found");
    let training_path = home_dir + "/markov_chain" + "/training";

    // Gets the paths of evey file and directory in the training_path.
    let tpaths =
        fs::read_dir(&training_path).expect(&format!("Can't read files from: {}", training_path));

    // Only the files remain
    let files = tpaths
        .filter_map(|f| f.ok())
        .filter(|f| match f.file_type() {
            Err(_) => false,
            Ok(f) => f.is_file(),
        });

    // Reads every file into a string
    let contents = files.filter_map(|f| read_to_string(f.path()).ok());

    // Creating the Markov Chain
    let markov_chain = contents.fold(MarkovChain::with_capacity(2, 8_000_000), |mut a, s| {
        a.add_text(&s);
        a
    });

    markov_chain
}
