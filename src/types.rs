use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Santa {
    pub name: String,
    pub clan: String,
    pub email_address: String,
    pub history: Vec<String>,
}

pub type SantaMap = HashMap<String, Santa>;