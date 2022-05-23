use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub data: String,
}
// impl Entry {
//     pub fn new(&self, text: String) -> Entry{
//         Entry {
//             data: text,
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct EntriesList {
    pub data: Vec<Entry>,
}
