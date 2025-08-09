use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record
{
    pub name: String,
    pub reaction_time_ms : u64,
}

#[derive(Debug, Default)]
pub struct Leaderboard{
    records : Vec<Record>,
}

impl Leaderboard {
    pub fn new() -> Self
    {
        Self { records: vec![] }
    }


    pub fn add_record(&mut self, record : Record)
    {
        self.records.push(record);
        self.records.sort_by_key(|r| r.reaction_time_ms);
        if self.records.len() > 100
        {
            self.records.truncate(100);
        }
    }

    pub fn get_top(&self, count:usize) -> Vec<Record>
    {
        self.records.iter().take(count).cloned().collect()
    }
}