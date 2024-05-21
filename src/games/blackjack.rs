use std::fmt::Display;
use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::SeqCst;
use serenity::all::{Colour, CreateEmbed, Message};
use serenity::builder::CreateEmbedFooter;
use crate::{hey, SKEPZ_WIN_ALWAYS, SUPERBOOST, SUPERBOOST_MODE};
use crate::games::{Card, CardType, Deck, Suit};
use crate::userfile::UserValues;

fn is_10_value(card: &Card) -> bool {
    match card.card_type {
        CardType::Ten | CardType::Jack | CardType::Queen | CardType::King => true,
        _ => false
    }
}

fn card_value(card: &Card) -> u64 {
    match card.card_type {
        CardType::Ace => 11,
        CardType::Two => 2,
        CardType::Three => 3,
        CardType::Four => 4,
        CardType::Five => 5,
        CardType::Six => 6,
        CardType::Seven => 7,
        CardType::Eight => 8,
        CardType::Nine => 9,
        CardType::Ten | CardType::Jack | CardType::Queen | CardType::King => 10,
        _ => 0
    }
}

#[derive(Debug)]
pub struct BlackjackHand {
    pub cards: Vec<Card>,
}

impl BlackjackHand {
    pub fn new() -> BlackjackHand {
        BlackjackHand {
            cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn score(&self) -> u64 {
        let mut score = 0;
        let mut num_aces = 0;
        for card in &self.cards {
            match card.card_type {
                CardType::Ace => {
                    num_aces += 1;
                    score += 11;
                }
                CardType::Two => score += 2,
                CardType::Three => score += 3,
                CardType::Four => score += 4,
                CardType::Five => score += 5,
                CardType::Six => score += 6,
                CardType::Seven => score += 7,
                CardType::Eight => score += 8,
                CardType::Nine => score += 9,
                CardType::Ten | CardType::Jack | CardType::Queen | CardType::King => score += 10,
                _ => {} // joker not in deck
            }
        }
        while score > 21 && num_aces > 0 {
            score -= 10;
            num_aces -= 1;
        }
        score
    }

    pub fn is_blackjack(&self) -> bool {
        // if the first two cards are an Ace and a 10-value card
        self.cards.len() == 2 && self.score() == 21
    }

    pub fn is_winning(&self, other: &BlackjackHand) -> bool {
        self.score() > other.score()
    }

    pub fn is_21(&self) -> bool {
        self.score() == 21
    }

    pub fn is_bust(&self) -> bool {
        self.score() > 21
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn is_push(&self, other: &BlackjackHand) -> bool {
        self.score() == other.score()
    }
}

impl Display for BlackjackHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut printable = String::new();
        for x in 0..self.cards.len() {
            printable.push_str(&format!("{} ({})", self.cards[x], self.cards[x].display_no_suite()));
            if x != self.cards.len() - 1 {
                printable.push_str(", ");
            }
        }
        write!(f, "{}", printable)
    }
}

pub struct BlackjackPlayer {
    hands: Vec<BlackjackHand>, // allow up to 3 splits (4 hands total)
    bet: u64,
    playing_hand: u64,
}

impl BlackjackPlayer {
    pub fn new(bet: u64) -> BlackjackPlayer {
        let hands = vec![BlackjackHand::new()];
        BlackjackPlayer {
            hands,
            bet,
            playing_hand: 0,
        }
    }

    pub fn can_split(&mut self) -> bool {
        let hand_len = self.hands.len();
        let playing_hand = self.playing_hand();

        let is_2_cards = playing_hand.cards.len() == 2;
        let less_than_4_hands = hand_len < 4;
        let is_pair = card_value(&playing_hand.cards[0]) == card_value(&playing_hand.cards[1]);
        let is_10_value = is_10_value(&playing_hand.cards[0]) && is_10_value(&playing_hand.cards[1]);

        is_2_cards && less_than_4_hands && (is_pair || is_10_value)
    }

    pub fn split(&mut self) {
        let hands_len = self.hands.len();
        let second_card = self.hands[self.playing_hand as usize].cards.pop().unwrap();
        self.hands.push(BlackjackHand::new());
        self.hands.last_mut().unwrap().add_card(second_card);
        // let hand = self.hands.pop().unwrap();
        // self.hands.push(BlackjackHand::new());
        // self.hands.push(BlackjackHand::new());
        // self.hands[0].add_card(hand.cards[0]);
        // self.hands[1].add_card(hand.cards[1]);
    }

    pub fn double_down(&mut self, amt: u64) {
        self.bet += amt;
    }

    pub fn hit(&mut self, card: Card) {
        self.hands[self.playing_hand as usize].add_card(card);
    }

    pub fn playing_hand(&mut self) -> &BlackjackHand {
        &self.hands[self.playing_hand as usize]
    }

    // returns false if no hand is available
    pub fn next_hand(&mut self) -> bool {
        if self.playing_hand < self.hands.len() as u64 - 1 {
            self.playing_hand += 1;
            true
        } else {
            false
        }
    }
}

pub struct BlackJack {
    pub player: BlackjackPlayer,
    dealer: BlackjackHand,
    deck: Deck,
    pub offered_insurance: bool,
    player_blackjack: bool,
    turn: u64,
    original_bet: u64,
}

impl BlackJack {
    pub fn new(bet: u64) -> BlackJack {
        let mut deck = Deck::new(6, false);
        deck.shuffle();

        let player = BlackjackPlayer::new(bet.clone());
        let dealer = BlackjackHand::new();

        BlackJack {
            player,
            dealer,
            deck,
            offered_insurance: false,
            player_blackjack: false,
            turn: 0,
            original_bet: bet
        }
    }

    pub fn deal(&mut self) {
        self.player.hands[0].add_card(self.deck.deal());
        self.dealer.add_card(self.deck.deal());
        self.player.hands[0].add_card(self.deck.deal());
        self.dealer.add_card(self.deck.deal());

        // insurance check
        self.offered_insurance = self.dealer_card().card_type == CardType::Ace || is_10_value(&self.dealer_card());

        // check for player blackjack
        self.player_blackjack = self.player.hands[0].is_blackjack();
    }

    pub fn next_player_hand(&mut self) -> bool {
        self.player.bet = self.original_bet;
        let worked = self.player.next_hand();
        self.hit();
        self.turn = 0;
        self.player_blackjack = self.player.playing_hand().is_blackjack();
        worked
    }

    pub fn hit(&mut self) {
        self.player.hit(self.deck.deal());
        self.turn += 1;
    }

    pub fn dealer_card(&self) -> Card {
        self.dealer.cards[0]
    }

    pub fn stand(&mut self) {
        self.turn += 1;
        // Dealer's turn
        self.dealer_turn();
    }

    pub fn double_down(&mut self, amt: u64) {
        // increase bet and end game
        self.player.double_down(amt);
        self.hit();
        self.stand();
    }

    // tell the user what they can currently do
    pub fn give_options(&mut self) -> String {
        let can_split = self.player.can_split();
        let playing_hand = self.player.playing_hand();
        let mut options = "Options:".to_string();
        if self.offered_insurance {
            options.push_str(" `\"yes\"` or `\"no\"`");
            return options;
        }
        if !playing_hand.is_blackjack() {
            options.push_str(" `\"hit\"`");
        }
        options.push_str(" `\"stand\"`");
        if can_split {
            options.push_str(" `\"split\"`");
        }
        if self.turn == 0 && !playing_hand.is_blackjack() {
            options.push_str(" `\"double\"`");
        }
        options
    }

    pub fn dealer_turn(&mut self) {
        while self.dealer.score() < 17 {
            self.dealer.add_card(self.deck.deal());
        }
    }

    pub fn give_help(&mut self) -> String {
        match self.player.playing_hand().score() {
            4..=5 => {
                "Me think you hit until big numba".to_string()
            }
            7..=9 => {
                "Me thinks you should hit big friend".to_string()
            }
            10 => {
                if self.turn == 0 {
                    "Me thinks you should hit or double for big nanner".to_string()
                } else {
                    "Me thinks you hit".to_string()
                }
            }
            11 => {
                if self.turn == 0 {
                    "Me think it time for BIG DUblE".to_string()
                } else {
                    "Me thinks you hit".to_string()
                }
            }
            12 if self.player.playing_hand().cards[0].card_type == CardType::Ace && self.player.playing_hand().cards[1].card_type == CardType::Ace => { // pair of aces
                "Me thinks you should split".to_string()
            }
            12..=15 => {
                match self.dealer_card().card_type {
                    CardType::Seven | CardType::Eight | CardType::Nine | CardType::Ten | CardType::King | CardType::Queen | CardType::Jack => {
                        "Me thinks u only option to hit".to_string()
                    }
                    _ => {
                        "Me think u in no gud spot maybe stand?".to_string()
                    }
                }
            }
            16 => {
                if self.dealer_card().card_type == CardType::Two || self.dealer_card().card_type == CardType::Three
                    || self.dealer_card().card_type == CardType::Four || self.dealer_card().card_type == CardType::Five || self.dealer_card().card_type == CardType::Six {
                    "Me thinks hit would not be smartest option".to_string()
                } else {
                    "Me thinks u shud to hit".to_string()
                }
            }
            17..=19 => {
                "me thinks u shud stand".to_string()
            }
            20 => {
                "if u no stand u dumber than me".to_string()
            }
            21 => {
                "u so stoopid if u need help wit this".to_string()
            }
            _ => {
                "ur nanners gonna taste so good".to_string()
            }
        }
    }

    pub fn craft_embed(&mut self, username: &String, message: String) -> CreateEmbed {
        let playing_hand = self.player.playing_hand();
        CreateEmbed::new()
            .title(format!("Blackjack ({}'s Game)", username))
            .description(format!("{}", message))
            .thumbnail("attachment://monkey.png")
            .color(Colour::GOLD)
            .field(format!("Your Hand ({})", playing_hand.score()), format!("{}", playing_hand), false)
            .field("George's hand".to_string(), format!("{} ({})", self.dealer_card(),
                                                      self.dealer_card().display_no_suite()), false)
            .field("Options", format!("{}", self.give_options()), false)
            .footer(CreateEmbedFooter::new(format!("George Advice: {}", self.give_help())))
    }

    pub fn end_embed(&self, msg: &Message, end_message: String, toast: String, user_values: &mut UserValues) -> CreateEmbed {
        CreateEmbed::new()
            .title(format!("Blackjack ({}'s Game)", msg.author.global_name.clone().unwrap()))
            .description(end_message)
            .thumbnail("attachment://monkey.png")
            .field(format!("George's hand ({})", self.dealer.score()), format!("{}", self.dealer), false)
            .field("Balance:", format!("{}:banana:", user_values.get_bananas()), false)
            .color(Colour::GOLD)
            .footer(CreateEmbedFooter::new(toast))
    }

    pub fn determine_winner(&mut self, userfile: &mut UserValues, msg: &Message) -> (CreateEmbed, bool) {
        let old_score = self.player.playing_hand().score();
        if msg.author.id.get() == 318884828508454912 && SKEPZ_WIN_ALWAYS.load(SeqCst) { // skepz wins
            let mut payout = self.player.bet * 2;
            if SUPERBOOST_MODE.load(Ordering::SeqCst) {
                payout *= SUPERBOOST;
            }
            userfile.add_bananas(payout);
            // add bananas to fed
            if self.next_player_hand() {
                return (self.craft_embed(&msg.author.global_name.clone().unwrap(),
                                         "**NEXT HAND** I forfeit.".to_string()),
                        false);
            }
            (self.end_embed(msg, "I forfeit.".to_string(),
                            "Me George, me forfeit".to_string(), userfile), true)
        } else if self.player.playing_hand().is_bust() { // PLAYER BUST
            // add bananas to fed
            if self.next_player_hand() {
                return (self.craft_embed(&msg.author.global_name.clone().unwrap(),
                                         format!("**NEXT HAND** You bust with {}. Me win! Me eat good tonight!", old_score)),
                        false);
            }
            (self.end_embed(msg,
                           format!("You bust with {}. Me win! Me eat good tonight!", old_score),
                           "Me George the monkey, me win".to_string(), userfile), true)
        } else if self.dealer.is_push(self.player.playing_hand()) { // PUSH
            userfile.add_bananas(self.player.bet);
            if self.next_player_hand() {
                return (self.craft_embed(&msg.author.global_name.clone().unwrap(),
                                         format!("**NEXT HAND** We tie at {}. Me no like tie. Me hungry for nanners!", self.dealer.score())),
                        false);
            }
            (self.end_embed(msg,
                           format!("We tie at {}. Me no like tie. Me hungry for nanners!", self.dealer.score()),
                           "Please play again! I hungry for nanners!".to_string(), userfile), true)

        } else if self.dealer.is_bust() || self.player.playing_hand().is_winning(&self.dealer) { // DEALER LOOSE
            if self.player_blackjack {
                let mut payout = (self.player.bet as f32 * 2.5).round() as u64;
                if SUPERBOOST_MODE.load(Ordering::SeqCst) {
                    payout *= SUPERBOOST;
                }
                userfile.add_bananas(payout);
            } else {
                let mut payout = self.player.bet * 2;
                if SUPERBOOST_MODE.load(Ordering::SeqCst) {
                    payout *= SUPERBOOST;
                }
                userfile.add_bananas(payout);
            }
            if self.next_player_hand() {
                return (self.craft_embed(&msg.author.global_name.clone().unwrap(),
                                         "**NEXT HAND** Me no like when you win. Now me gonna starve!".to_string()),
                        false);
            }
            (self.end_embed(msg,
                           "Me no like when you win. Now me gonna starve!".to_string(),
                           "Please play again! I hungry for nanners!".to_string(), userfile), true)
        } else { // DEALER WINS
            if self.next_player_hand() {
                return (self.craft_embed(&msg.author.global_name.clone().unwrap(),
                                         format!("**NEXT HAND** Me win with {}! You loose! Me eat good tonight!", self.dealer.score())),
                        false);
            }
            (self.end_embed(msg,
                           format!("Me win with {}! You loose! Me eat good tonight!", self.dealer.score()),
                           "Me George, me win".to_string(), userfile), true)
        }
    }

    // returns true if the game has ended
    pub fn handle_message(&mut self, msg: &Message) -> (CreateEmbed, bool) {
        let mut userfile = UserValues::get(&msg.author.id);

        let content = msg.content.as_str().to_lowercase();
        let mut words = content.split_whitespace();
        // parse first word of the message
        let first_word = words.next().unwrap();
        if self.offered_insurance {
            match first_word {
                "yes" => {
                    // ensure the player has enough for insurance
                    if userfile.get_bananas() < self.player.bet / 2 {
                        // reply with error
                        return (self.craft_embed(&msg.author.global_name.clone().unwrap(), "You do not have enough bananas for insurance!".to_string()), false);
                    }
                    if self.player.playing_hand().is_blackjack() {
                        // player has blackjack, pay out insurance
                        userfile.add_bananas(self.player.bet); // add bet back
                        return (self.end_embed(msg,
                                               "You go straight to point. I like that. We push.. this time".to_string(),
                                               "Give me nanners please!".to_string(), &mut userfile), true);
                    }
                    // if dealer has blackjack, pay out insurance
                    return if self.dealer.is_blackjack() {
                        // payout insurance and pay back bet
                        userfile.add_bananas(self.player.bet);
                        // reply with insurance payout
                        (self.end_embed(msg,
                                        "George has blackjack! You get your nanners back.".to_string(),
                                        "Give me nanners please!".to_string(), &mut userfile), true)
                    } else {
                        // player looses insurance
                        userfile.add_bananas(self.player.bet / 2); // add half of bet back
                        self.offered_insurance = false;
                        (self.craft_embed(&msg.author.global_name.clone().unwrap(), "George does not have blackjack. You loose half your bet".to_string()), false)
                    }
                }
                "no" => {
                    if self.dealer.is_blackjack() {
                        // todo: make this a chance of happening to all users
                        if msg.author.id.get() == 318884828508454912 && SKEPZ_WIN_ALWAYS.load(SeqCst) {
                            let mut payout = self.player.bet * 2;
                            if SUPERBOOST_MODE.load(Ordering::SeqCst) {
                                payout *= SUPERBOOST;
                            }
                            userfile.add_bananas(payout);
                            return(self.end_embed(msg,
                                                  "Me had blackjack but you look like you could use some nanners".to_string(),
                                                  "Give me nanners please!".to_string(), &mut userfile), true);
                        }
                        // dealer has blackjack, end game
                        return (self.end_embed(msg,
                                               "George has blackjack! You loose!".to_string(),
                                               "Give me nanners please!".to_string(), &mut userfile), true);
                    }
                    // continue game
                    self.offered_insurance = false;
                }
                _ => {
                    // reply with error
                    return (self.craft_embed(&msg.author.global_name.clone().unwrap(), "Me no understand. Do you want insurance?".to_string()), false);
                }
            }

            return (self.craft_embed(&msg.author.global_name.clone().unwrap(), "Game has begun!".to_string()), false);
        }
        return match first_word {
            "hit" if !self.player.playing_hand().is_blackjack() => {
                self.hit();
                if self.player.playing_hand().is_bust() {
                    return self.determine_winner(&mut userfile, msg);
                }
                (self.craft_embed(&msg.author.global_name.clone().unwrap(), "You hit! Me looking forward to stealing those nanners!".to_string()), false)
            },
            "stand" => {
                self.stand();
                // determine winner
                self.determine_winner(&mut userfile, msg)
            },
            "split" if self.player.can_split() => {
                // if can't afford
                if userfile.get_bananas() < self.player.bet {
                    // reply with error
                    return (self.craft_embed(&msg.author.global_name.clone().unwrap(), "You do not have enough bananas to split!".to_string()), false);
                }

                self.player.split();
                userfile.remove_bananas(self.player.bet);
                self.hit();

                (self.craft_embed(&msg.author.global_name.clone().unwrap(), "You split! Me for sure gonna win now!".to_string()), false)
            }
            "double" if self.turn == 0 && !self.player.playing_hand().is_blackjack() => {
                let mut amt = self.player.bet;
                if let Some(next) = words.next() {
                    let Ok(a) = next.parse::<u64>() else {
                        // reply with error
                        return (self.craft_embed(&msg.author.global_name.clone().unwrap(), "You must provide a valid amount to double down with!".to_string()), false);
                    };
                    // ensure amount is above 5 and below the player's current bet
                    if amt < 5 || amt >= self.player.bet {
                        // reply with error
                        return (self.craft_embed(&msg.author.global_name.clone().unwrap(),
                                                 format!("You must provide a valid amount to double down with! (5 to {})", self.player.bet)), false);
                    }

                    amt = a;
                }

                // ensure player has enough money
                if userfile.get_bananas() < amt {
                    // reply with error
                    return (self.craft_embed(&msg.author.global_name.clone().unwrap(), "You do not have enough bananas to double down with!".to_string()), false);
                }

                userfile.remove_bananas(amt);

                self.double_down(amt);
                self.determine_winner(&mut userfile, msg)
            },
            _ => {
                // reply with error
                (self.craft_embed(&msg.author.global_name.clone().unwrap(), "Me no understand what you say. Me only know the following options:".to_string()), false)
            }
        }
    }
}