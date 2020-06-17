// Represent cards from a desk
// A standard deck of cards has 52 cards: 4 suits and 13 cards per suit

// Start by creating the `Suit` enum and implement the associated
// function `random` which returns a random `Suit` (`Heart`,
// `Diamond`, `Spade` or `Club`)

// Then create the `Rank` enum that can have the value
// `Ace`, `King`, `Queen`, `Jack`, and `Number` associated to an `u8`
// value to represent the ranks 2 through 10
// After create an associated function to `Rank` called `Random` that
// returns a random `Rank`

// Finally create a structure name `Card` which has the fields `suit`
// and `rank`

// Write a program that takes that returns a random card in the deck
fn main() {
	let your_card =;

	println!("You're card is a {:?}", your_card);

	// Now if the card is an Ace of Spades print "You are the winner"
	{
		println!("You are the winner!");
	}
}
