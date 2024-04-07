use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg(long)]
    rules: String,
    #[arg(long)]
    events: String,

    #[arg(long)]
    object_hits: u32,
    #[arg(long)]
    location_hits: u32,
    #[arg(long)]
    area_hits: u32
}

impl Config {
    pub fn dummy() -> Self {
        Self {
            rules: String::from("C:\\Users\\wikto\\Desktop\\eventsystem\\rules.txt"),
            events: String::from("C:\\Users\\wikto\\Desktop\\eventsystem\\events.txt"),
            object_hits: 3,
            location_hits: 3,
            area_hits: 3
        }
    }
    pub fn parse() -> Self {
        Parser::parse()
    }
    pub fn rules(&self) -> &str {
        &self.rules
    }
    pub fn events(&self) -> &str {
        &self.events
    }
    pub fn object_hits(&self) -> u32 {
        self.object_hits
    }
    pub fn location_hits(&self) -> u32 {
        self.location_hits
    }
    pub fn area_hits(&self) -> u32 {
        self.area_hits
    }
}