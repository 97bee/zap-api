use std::{fmt};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
  Minted,
  Burnt,
  Swapped,
  Claimed,
}

impl fmt::Display for Event {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Event::Minted => write!(f, "Mint"),
      Event::Burnt => write!(f, "Burnt"),
      Event::Swapped => write!(f, "Swapped"),
      Event::Claimed => write!(f, "Claimed"),
    }
  }
}

#[derive(Clone)]
pub enum Network {
  MainNet,
  TestNet,
}

impl fmt::Display for Network {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Network::MainNet => write!(f, "mainnet"),
      Network::TestNet => write!(f, "testnet"),
    }
  }
}
