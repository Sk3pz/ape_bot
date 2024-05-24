use std::collections::HashMap;
use std::fmt::Display;
use rand::random;
use serenity::all::{UserId};
use crate::games::blackjack::BlackJack;
use crate::games::mine_battle::MineBattle;
use crate::games::sludge_monster_battle::SludgeMonsterBattle;
use crate::games::texas_holdem::TexasHoldem;

pub mod blackjack;
pub mod texas_holdem;
pub mod sludge_monster_battle;
pub mod mine_battle;

#[derive(Clone, Copy, Debug)]
pub enum CardType {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

impl CardType {
    pub fn is_face_card(&self) -> bool {
        match self {
            CardType::Jack | CardType::Queen | CardType::King => true,
            _ => false,
        }
    }
}

impl PartialEq for CardType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CardType::Ace => match other {
                CardType::Ace => true,
                _ => false,
            },
            CardType::Two => match other {
                CardType::Two => true,
                _ => false,
            },
            CardType::Three => match other {
                CardType::Three => true,
                _ => false,
            },
            CardType::Four => match other {
                CardType::Four => true,
                _ => false,
            },
            CardType::Five => match other {
                CardType::Five => true,
                _ => false,
            },
            CardType::Six => match other {
                CardType::Six => true,
                _ => false,
            },
            CardType::Seven => match other {
                CardType::Seven => true,
                _ => false,
            },
            CardType::Eight => match other {
                CardType::Eight => true,
                _ => false,
            },
            CardType::Nine => match other {
                CardType::Nine => true,
                _ => false,
            },
            CardType::Ten => match other {
                CardType::Ten => true,
                _ => false,
            },
            CardType::Jack => match other {
                CardType::Jack => true,
                _ => false,
            },
            CardType::Queen => match other {
                CardType::Queen => true,
                _ => false,
            },
            CardType::King => match other {
                CardType::King => true,
                _ => false,
            },
            CardType::Joker => match other {
                CardType::Joker => true,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn is_red(&self) -> bool {
        match self {
            Suit::Hearts | Suit::Diamonds => true,
            _ => false,
        }
    }

    pub fn is_black(&self) -> bool {
        match self {
            Suit::Clubs | Suit::Spades => true,
            _ => false,
        }
    }
}

impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Suit::Hearts => match other {
                Suit::Hearts => true,
                _ => false,
            },
            Suit::Diamonds => match other {
                Suit::Diamonds => true,
                _ => false,
            },
            Suit::Clubs => match other {
                Suit::Clubs => true,
                _ => false,
            },
            Suit::Spades => match other {
                Suit::Spades => true,
                _ => false,
            },
        }
    }

}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub card_type: CardType,
    pub suit: Suit,
}

impl Card {
    pub fn new(card_type: CardType, suit: Suit) -> Card {
        Card { card_type, suit }
    }

    pub fn display_raw(&self) -> String {
        format!("{:?} of {:?}", self.card_type, self.suit)
    }

    pub fn display_no_suite(&self) -> String {
        format!("{:?}", self.card_type)
    }

    pub fn is_face_card(&self) -> bool {
        self.card_type.is_face_card()
    }

    pub fn show_hidden(&self) -> String {
        "🂠".to_string()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self.card_type {
            CardType::Ace => {
                match self.suit {
                    Suit::Hearts   => "🂱",
                    Suit::Diamonds => "🃁",
                    Suit::Clubs    => "🃑",
                    Suit::Spades   => "🂡",
                }
            }
            CardType::Two => {
                match self.suit {
                    Suit::Hearts   => "🂲",
                    Suit::Diamonds => "🃂",
                    Suit::Clubs    => "🃒",
                    Suit::Spades   => "🂢",
                }
            }
            CardType::Three => {
                match self.suit {
                    Suit::Hearts   => "🂳",
                    Suit::Diamonds => "🃃",
                    Suit::Clubs    => "🃓",
                    Suit::Spades   => "🂣",
                }
            }
            CardType::Four => {
                match self.suit {
                    Suit::Hearts   => "🂴",
                    Suit::Diamonds => "🃄",
                    Suit::Clubs    => "🃔",
                    Suit::Spades   => "🂤",
                }
            }
            CardType::Five => {
                match self.suit {
                    Suit::Hearts   => "🂵",
                    Suit::Diamonds => "🃅",
                    Suit::Clubs    => "🃕",
                    Suit::Spades   => "🂥",
                }
            }
            CardType::Six => {
                match self.suit {
                    Suit::Hearts   => "🂶",
                    Suit::Diamonds => "🃆",
                    Suit::Clubs    => "🃖",
                    Suit::Spades   => "🂦",
                }
            }
            CardType::Seven => {
                match self.suit {
                    Suit::Hearts   => "🂷",
                    Suit::Diamonds => "🃇",
                    Suit::Clubs    => "🃗",
                    Suit::Spades   => "🂧",
                }
            }
            CardType::Eight => {
                match self.suit {
                    Suit::Hearts   => "🂸",
                    Suit::Diamonds => "🃈",
                    Suit::Clubs    => "🃘",
                    Suit::Spades   => "🂨",
                }
            }
            CardType::Nine => {
                match self.suit {
                    Suit::Hearts   => "🂹",
                    Suit::Diamonds => "🃉",
                    Suit::Clubs    => "🃙",
                    Suit::Spades   => "🂩",
                }
            }
            CardType::Ten => {
                match self.suit {
                    Suit::Hearts   => "🂺",
                    Suit::Diamonds => "🃊",
                    Suit::Clubs    => "🃚",
                    Suit::Spades   => "🂪",
                }
            }
            CardType::Jack => {
                match self.suit {
                    Suit::Hearts   => "🂻",
                    Suit::Diamonds => "🃋",
                    Suit::Clubs    => "🃛",
                    Suit::Spades   => "🂫",
                }
            }
            CardType::Queen => {
                match self.suit {
                    Suit::Hearts   => "🂽",
                    Suit::Diamonds => "🃍",
                    Suit::Clubs    => "🃝",
                    Suit::Spades   => "🂭",
                }
            }
            CardType::King => {
                match self.suit {
                    Suit::Hearts   => "🂾",
                    Suit::Diamonds => "🃎",
                    Suit::Clubs    => "🃞",
                    Suit::Spades   => "🂮",
                }
            }
            CardType::Joker => {
                match self.suit {
                    Suit::Hearts | Suit::Diamonds => "🂿",
                    Suit::Spades | Suit::Clubs    => "🃏",
                }
            }
        };
        write!(f, "{}", printable)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card_type == other.card_type && self.suit == other.suit
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(decks: u8, jokers: bool) -> Deck {
        let mut cards = Vec::new();
        // six decks
        for _ in 0..decks {
            cards.push(Card::new(CardType::Ace, Suit::Hearts));
            cards.push(Card::new(CardType::Ace, Suit::Diamonds));
            cards.push(Card::new(CardType::Ace, Suit::Spades));
            cards.push(Card::new(CardType::Ace, Suit::Clubs));
            
            cards.push(Card::new(CardType::Two, Suit::Hearts));
            cards.push(Card::new(CardType::Two, Suit::Diamonds));
            cards.push(Card::new(CardType::Two, Suit::Spades));
            cards.push(Card::new(CardType::Two, Suit::Clubs));
            
            cards.push(Card::new(CardType::Three, Suit::Hearts));
            cards.push(Card::new(CardType::Three, Suit::Diamonds));
            cards.push(Card::new(CardType::Three, Suit::Spades));
            cards.push(Card::new(CardType::Three, Suit::Clubs));
            
            cards.push(Card::new(CardType::Four, Suit::Hearts));
            cards.push(Card::new(CardType::Four, Suit::Diamonds));
            cards.push(Card::new(CardType::Four, Suit::Spades));
            cards.push(Card::new(CardType::Four, Suit::Clubs));
            
            cards.push(Card::new(CardType::Five, Suit::Hearts));
            cards.push(Card::new(CardType::Five, Suit::Diamonds));
            cards.push(Card::new(CardType::Five, Suit::Spades));
            cards.push(Card::new(CardType::Five, Suit::Clubs));
            
            cards.push(Card::new(CardType::Six, Suit::Hearts));
            cards.push(Card::new(CardType::Six, Suit::Diamonds));
            cards.push(Card::new(CardType::Six, Suit::Spades));
            cards.push(Card::new(CardType::Six, Suit::Clubs));
            
            cards.push(Card::new(CardType::Seven, Suit::Hearts));
            cards.push(Card::new(CardType::Seven, Suit::Diamonds));
            cards.push(Card::new(CardType::Seven, Suit::Spades));
            cards.push(Card::new(CardType::Seven, Suit::Clubs));
            
            cards.push(Card::new(CardType::Eight, Suit::Hearts));
            cards.push(Card::new(CardType::Eight, Suit::Diamonds));
            cards.push(Card::new(CardType::Eight, Suit::Spades));
            cards.push(Card::new(CardType::Eight, Suit::Clubs));
            
            cards.push(Card::new(CardType::Nine, Suit::Hearts));
            cards.push(Card::new(CardType::Nine, Suit::Diamonds));
            cards.push(Card::new(CardType::Nine, Suit::Spades));
            cards.push(Card::new(CardType::Nine, Suit::Clubs));
            
            cards.push(Card::new(CardType::Ten, Suit::Hearts));
            cards.push(Card::new(CardType::Ten, Suit::Diamonds));
            cards.push(Card::new(CardType::Ten, Suit::Spades));
            cards.push(Card::new(CardType::Ten, Suit::Clubs));
            
            cards.push(Card::new(CardType::Jack, Suit::Hearts));
            cards.push(Card::new(CardType::Jack, Suit::Diamonds));
            cards.push(Card::new(CardType::Jack, Suit::Spades));
            cards.push(Card::new(CardType::Jack, Suit::Clubs));
            
            cards.push(Card::new(CardType::Queen, Suit::Hearts));
            cards.push(Card::new(CardType::Queen, Suit::Diamonds));
            cards.push(Card::new(CardType::Queen, Suit::Spades));
            cards.push(Card::new(CardType::Queen, Suit::Clubs));
            
            cards.push(Card::new(CardType::King, Suit::Hearts));
            cards.push(Card::new(CardType::King, Suit::Diamonds));
            cards.push(Card::new(CardType::King, Suit::Spades));
            cards.push(Card::new(CardType::King, Suit::Clubs));
            
            if jokers {
                cards.push(Card::new(CardType::Joker, Suit::Hearts));
                cards.push(Card::new(CardType::Joker, Suit::Diamonds));
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

pub enum Games {
    BlackJack(BlackJack),
    TexasHoldem(TexasHoldem),

    SludgeMonsterBattle(SludgeMonsterBattle),
    MineBattle(MineBattle),
}

pub struct GameHandler {
    // host can start the game, cancel the game, end the game, and skip a player?
    pub host: UserId,
    pub players: Vec<UserId>,
    pub game: Games,
}

impl GameHandler {
    pub fn new(host: UserId, game: Games) -> GameHandler {
        GameHandler { host, players: Vec::new(), game }
    }

    pub fn has_player(&self, player: &UserId) -> bool {
        self.players.contains(&player)
    }

    pub fn add_player(&mut self, player: UserId) {
        self.players.push(player);
    }
}

pub type GameCode = usize;

pub struct GamesManager {
    pub games: HashMap<GameCode, GameHandler>,
}

impl GamesManager {
    pub fn new() -> GamesManager {
        GamesManager { games: HashMap::new() }
    }

    pub fn insert(&mut self, game: GameHandler) -> GameCode {
        let code = self.generate_game_code();
        self.games.insert(code.clone(), game);
        code
    }

    pub fn get_game(&mut self, code: GameCode) -> Option<&mut GameHandler> {
        self.games.get_mut(&code)
    }

    pub fn end_game(&mut self, code: GameCode) {
        self.games.remove(&code);
    }

    pub fn generate_game_code(&self) -> GameCode {
        // 5 character game code
        let mut code = random::<GameCode>() % 100000;
        while self.code_exists(code) {
            code = random::<GameCode>() % 100000;
        }
        code
    }

    pub fn code_exists(&self, code: GameCode) -> bool {
        self.games.contains_key(&code)
    }

    pub fn get_player_game(&mut self, player: &UserId) -> Option<GameCode> {
        for (code, game) in self.games.iter() {
            if game.has_player(player) {
                return Some(code.clone());
            }
        }
        for (code, game) in self.games.iter() {
            if game.host == *player {
                return Some(code.clone());
            }
        }
        None
    }

    pub fn get_player_game_instance(&mut self, player: &UserId) -> Option<&mut GameHandler> {
        let code = self.get_player_game(player);
        match code {
            Some(code) => self.get_game(code),
            None => None,
        }
    }

    pub fn find_player_playing(&mut self, player: &UserId) -> Option<&mut GameHandler> {
        for game in self.games.values_mut() {
            if game.has_player(player) {
                return Some(game);
            }
        }
        None
    }
    pub fn find_player_host(&mut self, player: &UserId) -> Option<&mut GameHandler> {
        for game in self.games.values_mut() {
            if game.host == *player {
                return Some(game);
            }
        }
        None
    }

    pub fn can_join(&self, game_code: &GameCode) -> bool {
        let game = self.games.get(game_code).unwrap();
        match &game.game {
            Games::BlackJack(_) | Games::SludgeMonsterBattle(_) | Games::MineBattle(_) => false,
            Games::TexasHoldem(th) => th.can_join(),
        }
    }

    pub fn get_hand(&self, game_code: &GameCode, player: UserId) -> Option<Vec<Card>> {
        let game = self.games.get(game_code).unwrap();
        match &game.game {
            Games::BlackJack(_) | Games::SludgeMonsterBattle(_) | Games::MineBattle(_) => None,
            Games::TexasHoldem(th) => th.get_hand(player),
        }
    }
}