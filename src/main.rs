mod card_stuff;

fn main() {
    println!("Hello, world!");
    let card: card_stuff::Card = (
        card_stuff::Edition::Normal,
        card_stuff::Rank::Seven,
        card_stuff::Suit::Club,
    );

    let hand: [card_stuff::Card; 5] = [card; 5];
}
