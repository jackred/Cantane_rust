
#[derive(Debug)]
pub struct City {
    location: crate::Coord,
    player: i32, // player to be implemented
    harbor: Option<bool> // harbor type to be implemented
}

impl City {
    pub fn new(location: crate::Coord, player: i32, harbor: Option<bool>) -> City{
        City {
            location: location,
            player: player,
            harbor: harbor
        }
    }
    
    pub fn get_player(&self) -> i32 {
        self.player
    }
}

impl super::Town for City {
    #[inline]
    fn point(&self) -> i32 {
        2
    }
    #[inline]
    fn resource_multiplier(&self) -> i32 {
        2
    }
}

impl super::super::Buyable for City  {
    #[inline]
    fn get_cost(&self) -> crate::resource::ResourceDeck {
        enum_map! {
            crate::resource::Resource::Ore => 3,
            crate::resource::Resource::Grain => 2,
            _ => 0,
        }
    }
}