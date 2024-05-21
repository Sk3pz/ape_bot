use rand::{thread_rng};
use rand::prelude::SliceRandom;
use serenity::all::{CreateEmbed, Message, UserId};
use crate::games::{Card, Deck};

struct Player {
    user: UserId,
    hand: (Card, Card),
    current_bet: u64,
    folded: bool,
    played_round: bool,
}

pub struct TexasHoldem {
    buy_in: u64,
    players: Vec<Player>,
    community_cards: Vec<Card>,
    deck: Deck,
    current_bet: u64,
    round: u8,
}

impl TexasHoldem {

    pub fn new(buy_in: u64) -> Self {
        let mut deck = Deck::new(2, false);
        deck.shuffle();
        Self {
            buy_in,
            players: Vec::new(),
            community_cards: Vec::new(),
            deck,
            current_bet: 0,
            round: 0,
        }
    }

    pub fn add_player(&mut self, user: UserId) {
        self.players.push(Player {
            user,
            hand: (self.deck.deal(), self.deck.deal()),
            current_bet: 0,
            folded: false,
            played_round: false,
        });
    }

    pub fn start(&mut self) {
        self.round = 1;
        self.community_cards = Vec::new();

        // shuffle the players
        self.players.shuffle(&mut thread_rng());

        // assign blinds randomly 2% for big blind
        let big_blind_amount = self.buy_in / 50;
        // 1% for small blind
        let small_blind_amount = big_blind_amount / 2;

        self.players.get_mut(0).unwrap().current_bet = small_blind_amount;
        self.players.get_mut(1).unwrap().current_bet = big_blind_amount;

        // set current bet
        self.current_bet = big_blind_amount;
    }



    pub fn can_join(&self) -> bool {
        self.players.len() < 10 && self.round == 0
    }

    pub fn can_start(&self) -> bool {
        self.players.len() >= 2
    }

    pub fn get_hand(&self, user: UserId) -> Option<Vec<Card>> {
        self.players.iter().find(|p| p.user == user).map(|p| p.hand).map(|(c1, c2)| vec![c1, c2])
    }

    pub fn determine_winner(&self) -> Vec<&Player> {
        todo!()
    }

    pub fn handle_message(&mut self, msg: &Message) -> (CreateEmbed, bool) {
        todo!()
    }
}