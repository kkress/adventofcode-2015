use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn damge_per_gold(&self) -> f32 {
        self.damage as f32 / self.cost as f32
    }

    fn armor_per_gold(&self) -> f32 {
        self.armor  as f32/ self.cost as f32
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}/{}) - {}g", self.name, self.damage, self.armor, self.cost)
    }
}

impl cmp::PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
struct Slot {
    item: Option<Item>,
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.item {
            Some(ref x) => x.fmt(f),
            _ => write!(f, "None"),
        }
    }
}

impl Slot {
    fn new(item: Option<Item>) -> Slot { 
        Slot{ item: item }
    }

    fn damage(&self) -> i32 {
        match self.item {
            Some(ref i) => i.damage,
            None => 0,
        }
    }

    fn armor(&self) -> i32 {
        match self.item {
            Some(ref i) => i.armor,
            None => 0,
        }
    }

    fn cost(&self) -> i32 {
        match self.item {
            Some(ref i) => i.cost,
            None => 0,
        }
    }
}

#[derive(Debug)]
struct ItemSet {
    weapon: Item,
    armor: Slot,
    ring_l: Slot,
    ring_r: Slot,
}

impl fmt::Display for ItemSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Total {}g for [{}, {}, {}, {}]", self.cost(), self.weapon, self.armor, self.ring_l, self.ring_r)
    }
}

impl ItemSet {
    fn new(weapon: Item, armor: Slot, ring_l: Slot, ring_r: Slot) -> ItemSet {
        ItemSet{weapon: weapon, armor: armor, ring_l: ring_l, ring_r: ring_r}
    }

    fn cost(&self) -> i32 {
        self.weapon.cost + self.armor.cost() + self.ring_l.cost() + self.ring_r.cost()
    }

    fn attack_power(&self) -> i32 {
        self.weapon.damage + self.armor.damage() + self.ring_l.damage() + self.ring_r.damage()
        
    }
    fn defense_power(&self) -> i32 {
        self.weapon.armor + self.armor.armor() + self.ring_l.armor() + self.ring_r.armor()
    }
}



trait Combatant {
    fn hp(&self) -> i32;
    fn attack_power(&self) -> i32;
    fn defense_power(&self) -> i32;
    fn take_damage(&mut self, amount: i32);

    fn sim_attack(&self, other: &Combatant) -> i32 {
        cmp::max(1, self.attack_power() - other.defense_power())
    }

    fn attack(&self, other: &mut Combatant) {
        let amount = cmp::min(1, other.defense_power() - self.attack_power());
        other.take_damage(amount);
    }

    fn can_beat(&self, opponent: &Combatant) -> bool where Self: std::marker::Sized {
        let mut opponent_hp = opponent.hp();
        let mut player_hp = self.hp();
        let player_dmg = self.sim_attack(opponent);
        let opponent_dmg = opponent.sim_attack(self);

        let mut i = 1;
        loop {
            opponent_hp = opponent_hp - player_dmg;
            if opponent_hp <= 0 {
                return true;
            }
            player_hp = player_hp - opponent_dmg;
            if player_hp <= 0 {
                return false;
            }
            i += 1;
        }
    }
}

#[derive(Debug)]
struct Boss {
    hp: i32,
    attack: i32,
    defense: i32,
}

impl Combatant for Boss {
    fn hp(&self) -> i32 {
        self.hp
    }
    fn attack_power(&self) -> i32 {
        self.attack
    }
    fn defense_power(&self) -> i32 {
        self.defense
    }

    fn take_damage(&mut self, amount: i32) {
        self.hp = cmp::max(0, self.hp - amount);
    }
}

#[derive(Debug)]
struct Player<'a> {
    hp: i32,
    items: &'a ItemSet,
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}HP {}", self.hp, self.items)
    }
}

impl<'a> Combatant for Player<'a> {
    fn hp(&self) -> i32 {
        self.hp
    }
    fn take_damage(&mut self, amount: i32) {
        self.hp = cmp::max(0, self.hp - amount);
    }

    fn attack_power(&self) -> i32 {
        self.items.attack_power()
    }
    fn defense_power(&self) -> i32 {
        self.items.defense_power()
    }
}

fn all_item_sets() -> Vec<ItemSet> {
    let weapons = vec!(
        Item{
            name: "Dagger".to_string(),
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            name: "Shortsword".to_string(),
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            name: "Warhammer".to_string(),
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            name: "Longsword".to_string(),
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            name: "Greataxe".to_string(),
            cost: 74,
            damage: 8,
            armor: 0,
        },
    );
    let armor = vec!(
        None,
        Some(Item{
            name: "Leather".to_string(),
            cost: 13,
            damage: 0,
            armor: 1,
        }),
        Some(Item {
            name: "Chainmail".to_string(),
            cost: 31,
            damage: 0,
            armor: 2,
        }),
        Some(Item {
            name: "Splintmail".to_string(),
            cost: 53,
            damage: 0,
            armor: 3,
        }),
        Some(Item {
            name: "Bandedmail".to_string(),
            cost: 75,
            damage: 0,
            armor: 4,
        }),
        Some(Item {
            name: "Platemail".to_string(),
            cost: 102,
            damage: 0,
            armor: 5,
        }),
    );
    let rings = vec!(
        None,
        Some(Item {
            name: "Dmg +1".to_string(),
            cost: 25,
            damage: 1,
            armor: 0,
        }),
        Some(Item {
            name: "Dmg +2".to_string(),
            cost: 50,
            damage: 2,
            armor: 0,
        }),
        Some(Item {
            name: "Dmg +3".to_string(),
            cost: 100,
            damage: 3,
            armor: 0,
        }),
        Some(Item {
            name: "Def +1".to_string(),
            cost: 20,
            damage: 0,
            armor: 1,
        }),
        Some(Item {
            name: "Def +2".to_string(),
            cost: 40,
            damage: 0,
            armor: 2,
        }),
        Some(Item {
            name: "Def +3".to_string(),
            cost: 80,
            damage: 0,
            armor: 3,
        }),
    );

    let mut all = Vec::new();

    for w in &weapons {
        for a in &armor {
            for r in &rings {
                let others: Vec<Option<Item>> = rings.clone()
                    .into_iter()
                    .filter(|ref x| x.is_none() || **x != *r)
                    .collect::<Vec<_>>();
                for o in &others {
                    all.push(ItemSet::new(w.clone(),
                            Slot::new(a.clone()),
                            Slot::new(r.clone()),
                            Slot::new(o.clone())))
                }
            }
        }
    }
    all
}

fn main() {


    let boss = Boss {
        hp: 103,
        attack: 9,
        defense: 2,
    };

    let mut viable = all_item_sets().into_iter().filter(|i| {
        let player = Player{ hp: 100, items: i};
        player.can_beat(&boss)
    }).collect::<Vec<_>>();

    viable.sort_by_key(|i| i.cost());
    println!("Cheapest winning is {}", viable.first().unwrap());

    let mut not_viable = all_item_sets().into_iter().filter(|i| {
        let player = Player{ hp: 100, items: i};
        !player.can_beat(&boss)
    }).collect::<Vec<_>>();
    not_viable.sort_by_key(|i| i.cost());
    println!("Least Expensive losing is {}", not_viable.first().unwrap());
    println!("Most Expensive losing is {}", not_viable.last().unwrap());

}
