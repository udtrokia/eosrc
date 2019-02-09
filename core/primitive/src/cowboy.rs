use crate::{bytes, deref};
use std::ops::{Deref, DerefMut};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair};
use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize};

/// Users Flow Chart
/// Load Cowboy
/// -> Make Transaction
/// -> Send Transaction to Transaction Pool
/// -> Boarcast
/// -> Transaction Pool Call Back?
/// -> <Minner>
#[derive(Serialize, Deserialize, Debug)]
pub struct Cowboy(Keypair);
impl Cowboy {
    pub fn born() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Cowboy(keypair)
    }
}

bytes!(Cowboy);
deref!(Cowboy, Keypair);
