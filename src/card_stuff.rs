pub type Card = (Edition, Rank, Suit);

#[derive(Copy, Clone)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Copy, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Copy, Clone)]
pub enum Edition {
    Normal,
    Foil,
    Holographic,
    Polychrome,
}
