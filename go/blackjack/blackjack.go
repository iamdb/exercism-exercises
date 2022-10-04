package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
	case "ace":
		return 11
	case "king", "queen", "jack", "ten":
		return 10
	case "nine":
		return 9
	case "eight":
		return 8
	case "seven":
		return 7
	case "six":
		return 6
	case "five":
		return 5
	case "four":
		return 4
	case "three":
		return 3
	case "two":
		return 2
	case "one":
		return 1
	}

	return 0
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	dealerCard_val := ParseCard(dealerCard)

	if SplitAces(card1, card2) {
		return "P"
	}

	if BlackJack(card1, card2) {
		if dealerCard_val < 10 {
			return "W"
		} else {
			return "S"
		}
	}

	if Hit(card1, card2, dealerCard) {
		return "H"
	}

	return "S"
}

func Hit(card1 string, card2 string, dealerCard string) bool {
	card1_val := ParseCard(card1)
	card2_val := ParseCard(card2)
	hand_val := card1_val + card2_val
	dealerCard_val := ParseCard(dealerCard)

	return (hand_val >= 12 && hand_val <= 16 && dealerCard_val >= 7) || hand_val <= 11

}

func BlackJack(card1 string, card2 string) bool {
	card1_val := ParseCard(card1)
	card2_val := ParseCard(card2)

	return card1_val+card2_val == 21
}

func SplitAces(card1 string, card2 string) bool {
	card1_val := ParseCard(card1)
	card2_val := ParseCard(card2)

	return card1_val+card2_val == 22
}
