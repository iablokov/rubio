mod sequence;
use crate::sequence::{Seq, Sequence, DNA, RNA, Alphabet};


fn main() 
{
    let dna1 = Sequence::<DNA>::new("ATGC".to_owned()).unwrap();
    let dna2 = Sequence::<DNA>::new("CGAT".to_owned()).unwrap();
    let rna1 = dna1.to_rna();

    let sequences: Vec<Box<dyn Seq>> = vec![dna1, rna1, dna2];


    for seq in sequences 
    {
        println!("{} = {}", seq.seq_type(), seq.sequence());
        let seq_rev = seq.reverse();
        println!("{} = {}", seq_rev.seq_type(), seq_rev.sequence());
    }
}