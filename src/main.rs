mod sequence;
use crate::sequence::{Sequence, DNA};


fn main() 
{
    let dna = Sequence::<DNA>::new("ATGC".to_owned()).unwrap();
    println!("DNA: {}", dna.sequence);
    let rna = dna.to_rna();
    println!("RNA: {}", rna.sequence);
    let dna = rna.to_dna();
    println!("DNA: {}", dna.sequence);
}