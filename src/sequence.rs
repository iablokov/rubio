use std::ops::Range;

const ALPHABET_SIZE: usize = 128;

// This macro creates a boolean map for a given alphabet
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
    fn seq_type(&self) -> &str;
    const SYMBOLS: [bool; ALPHABET_SIZE];
    const ALLOWED: [bool; ALPHABET_SIZE];

    fn is_word(sequence: &String) -> bool
    {
        sequence.chars().all(|s| Self::SYMBOLS[s as usize])
    }
}

pub struct Sequence<A: Alphabet>
{
    sequence : String,
    pub alphabet : A,
}

#[derive(Default)]
pub struct DNA {}
impl Alphabet for DNA
{
    fn seq_type(&self) -> &str { "DNA" }
    const SYMBOLS: [bool; ALPHABET_SIZE] = alphabet_map!['A', 'T', 'G', 'C'];
    const ALLOWED: [bool; ALPHABET_SIZE] = alphabet_map!['N'];
}

#[derive(Default)]
pub struct RNA {}
impl Alphabet for RNA
{
    fn seq_type(&self) -> &str { "RNA" }
    const SYMBOLS: [bool; ALPHABET_SIZE] = alphabet_map!['A', 'U', 'G', 'C'];
    const ALLOWED: [bool; ALPHABET_SIZE] = alphabet_map!['N'];
}


pub trait Seq
{
    fn sequence(&self)     -> String;
    fn sequence_ref(&self) -> &str;
    fn seq_type(&self)     -> &str;
    fn reverse(&self)      -> Box<dyn Seq>;
}

impl<A: Alphabet + Default + 'static> Seq for Sequence<A>
{
    fn sequence(&self)     -> String { self.sequence.clone() }
    fn sequence_ref(&self) -> &str   { &self.sequence }
    fn seq_type(&self)     -> &str   { self.alphabet.seq_type() }
    fn reverse(&self)      -> Box<dyn Seq> 
    {
        Box::new( Sequence { sequence : self.sequence.chars().rev().collect(), alphabet : A::default() } )
    }
}


impl<A: Alphabet + Default> Sequence<A> 
{
    pub fn new(sequence: String) -> Option<Box<Self>>
    {
        if A::is_word(&sequence)
        {
            Some(Box::new( Sequence { sequence : sequence, alphabet : A::default() } ))
        }
        else { None }
    }
}

/// DNA-specific methods
impl Sequence<DNA>
{
    pub fn to_rna(&self) -> Box<Sequence<RNA>>
    {
        Box::new(Sequence::<RNA> { sequence: self.sequence.replace("T", "U"), alphabet: RNA::default() })
    }
}

/// RNA-specific methods
impl Sequence<RNA>
{
    pub fn to_dna(&self) -> Box<Sequence<DNA>>
    {
        Box::new(Sequence::<DNA> { sequence: self.sequence.replace("U", "T"), alphabet: DNA::default() })
    }
}
