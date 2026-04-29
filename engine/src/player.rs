#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PlayerId {
    P1,
    P2,
}

impl PlayerId {
    pub fn other(self) -> Self {
        match self {
            PlayerId::P1 => PlayerId::P2,
            PlayerId::P2 => PlayerId::P1,
        }
    }
}
