use std::ops::Range;

// This macro creates a boolean map for a given alphabet
const ALPHABET_SIZE: usize = 128;
macro_rules! alphabet_map 
{
    ($($symbol:expr),*) => {
    {
        let mut arr = [false; ALPHABET_SIZE];
        $(
            arr[$symbol as usize] = true;
        )*
        arr
    }};
}


pub trait Alphabet
{ 
    const SYMBOLS: [bool; ALPHABET_SIZE];
    const ALLOWED: [bool; ALPHABET_SIZE];

    fn is_word(sequence: &String) -> bool
    {
        sequence.chars().all(|s| Self::SYMBOLS[s as usize])
    }
}

pub struct Sequence<A: Alphabet>
{
    pub sequence : String,
    seq_type     : A,
}

#[derive(Default)]
pub struct DNA {}
impl DNA { const name: &str = "DNA";}

impl Alphabet for DNA
{
    const SYMBOLS: [bool; ALPHABET_SIZE] = alphabet_map!['A', 'T', 'G', 'C'];
    const ALLOWED: [bool; ALPHABET_SIZE] = alphabet_map!['N'];
}

#[derive(Default)]
pub struct RNA {}
impl RNA { const name: &str = "RNA";}

impl Alphabet for RNA
{
    const SYMBOLS: [bool; ALPHABET_SIZE] = alphabet_map!['A', 'U', 'G', 'C'];
    const ALLOWED: [bool; ALPHABET_SIZE] = alphabet_map!['N'];
}

impl<A: Alphabet + Default> Sequence<A> 
{
    pub fn new(sequence: String) -> Option<Self>
    {
        if A::is_word(&sequence)
        {
            Some( Sequence { sequence : sequence, seq_type : A::default() } )
        }
        else { None }
    }
}

impl Sequence<DNA>
{
    pub fn to_rna(&self) -> Sequence<RNA>
    {
        Sequence::<RNA> { sequence: self.sequence.replace("T", "U"), seq_type: RNA::default() }
    }
}

impl Sequence<RNA>
{
    pub fn to_dna(&self) -> Sequence<DNA>
    {
        Sequence::<DNA> { sequence: self.sequence.replace("U", "T"), seq_type: DNA::default() }
    }
}
