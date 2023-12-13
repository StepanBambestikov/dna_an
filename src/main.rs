mod sequence_analyzer;
mod nucleotide_frequency_analyzer;
mod length_distribution_analyzer;
mod unique_analyzer;

use std::fs;
use std::io::Write;
use crate::length_distribution_analyzer::LengthDistributionAnalyzer;
use crate::sequence_analyzer::SequenceAnalyzer;
use crate::nucleotide_frequency_analyzer::NucleotideFrequencyAnalyzer;
use crate::unique_analyzer::UniqueAnalyzer;

fn main() {
    let file_name = "correct_predicted_nodes.txt";
    let file_content = fs::read_to_string(file_name).expect("Unable to read file");
    let mut analyzers: Vec<Box<dyn SequenceAnalyzer>> = vec![
        NucleotideFrequencyAnalyzer::new(),
        LengthDistributionAnalyzer::new(),
        UniqueAnalyzer::new(),
    ];
    // let mut count = 0;
    for line in file_content.lines() {
        // if line.contains('>'){
        //     continue;
        // }
        for current_analyzer in &mut analyzers{
            current_analyzer.process_line(line);
        }
        // count += 1;
        // if count % 1000 == 0{
        //     println!("{}\n", count)
        // }
    }
    let mut file = fs::File::create(format!("{}_output.txt", file_name)).expect("Unable to create file for saving");
    for current_analyzer in &mut analyzers{
        let str = current_analyzer.get_final_data();
        // println!("{}", str)
        file.write_all(str.as_ref()).expect("Unable to write data");
    }
}