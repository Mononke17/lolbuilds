

struct Item<'a> {
    id: u64,
    tier: u8,
    name: &'a str,
    gold_cost: u32,
    passive_exists: bool,
    passive: Vec<&'a str>,
    unique_passive_exists: bool,
    unique_passive: Vec<&'a str>,
    limitation: &'a str,
    limitation_exists: bool,
    active_exists: bool,
    active: &'a str,

    //
    //stats
    //

    //Offensive:
    ability_power: u32,
    attack_damage: u32,
    attack_speed: u32,
    critical_strike_chance: u32,
    percentage_armor_penetration: u32,
    lethality: u32,
    flat_magic_penetration: u32,
    percentage_magic_penetration: u32,
    life_steal: u32,
    omnivamp: u32,
    physical_vamp: u32,

    //Defensive:
    health: u32,
    health_regeneration: u32,
    heal_shield_power: u32,
    armor: u32,
    magic_resist: u32,
    tenacity: [u32; 2], //[value, type]

    //Utility:
    ability_haste: u32,
    summoner_spell_haste: u32,
    item_haste: u32,
    mana: u32,
    mana_regeneration: u32,
    movement_speed: u32,
}



pub fn loaditems() {

    static LONGSWORD: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "Longsword",
        gold_cost: 350,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 10,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };




    static AMPLIFYING_TOME: Item =  Item {

        id:  2 ,
        tier: 1,
        name: "Amplifying Tome",
        gold_cost: 435,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 20,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static BF_SWORD: Item =  Item {

        id: 3,
        tier: 1,
        name: "B.F. Sword",
        gold_cost: 1300,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 40,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static BLASTING_WAND: Item =  Item {

        id:  4 ,
        tier: 1,
        name: "Blasting Wand",
        gold_cost: 850,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 40,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static BROKEN_STOPWATCH: Item =  Item {

        id:  5,
        tier: 1,
        name: "Broken Stopwatch",
        gold_cost: 750,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: true,
        active: "Shattered Time: Is broken, but can still be upgraded.",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static CLOAK_OF_AGILITY: Item =  Item {

        id:  6 ,
        tier: 1,
        name: "Cloak of Agility",
        gold_cost: 600,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 15,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static CLOTH_ARMOR: Item =  Item {

        id:  7 ,
        tier: 1,
        name: "Cloth Armor",
        gold_cost: 300,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 15,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static DAGGER: Item =  Item {

        id:  8 ,
        tier: 1,
        name: "Dagger",
        gold_cost: 300,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 12,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static FAERIE_CHARM: Item =  Item {

        id:  9 ,
        tier: 1,
        name: "Faerie Charm",
        gold_cost: 250,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 50,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static NEEDLESSLY_LARGE_ROD: Item =  Item {

        id:  10,
        tier: 1,
        name: "Needlessly Large Rod",
        gold_cost: 1200,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 1200,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static NULL_MAGIC_MANTLE: Item =  Item {

        id:  11 ,
        tier: 1,
        name: "Null Magic Mantle",
        gold_cost: 450,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 25,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static PICKAXE: Item =  Item {

        id:  12 ,
        tier: 1,
        name: "Pickaxe",
        gold_cost: 875,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 25,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static REJUVINATION_BEAD: Item =  Item {

        id:  13,
        tier: 1,
        name: "Rejuvination Bead",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static RUBY_CRYSTAL: Item =  Item {

        id:  14 ,
        tier: 1,
        name: "Ruby Crystal",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 150,
        health_regeneration: 100,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static SAPHIRE_CRYSTAL: Item =  Item {

        id:  14,
        tier: 1,
        name: "Saphire Crystal",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 250,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static SHEEN: Item =  Item {

        id:  16 ,
        tier: 1,
        name: "Sheen",
        gold_cost: 700,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: true,
        unique_passive: vec!["Spellblade: After using an ability, your next basic attack within 10 seconds deals 100% base AD bonus physical damage on-hit (1.5 (begins after using the empowered attack) second cooldown)."],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static STOPWATCH: Item =  Item {

        id:  17,
        tier: 1,
        name: "",
        gold_cost: 750,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: true,
        unique_passive: vec!["Stasis: Put yourself in stasis for 2.5 seconds, rendering you untargetable and invulnerable for the duration but also unable to move, declare basic attacks, cast abilities, use summoner spells, or activate items."],
        limitation: "Cannot be purchased while owning Guardian Angel or Zhonya's Hourglass. Transforms into Broken Stopwatch when activated, which cannot be activated again. Once broken, all future purchases are also broken. Limited to 1 Stopwatch.",
        limitation_exists: true,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static CULL: Item =  Item {

        id:  18,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };



    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        gold_cost: 0,
        passive_exists: false,
        passive: vec![],
        unique_passive_exists: false,
        unique_passive: vec![],
        limitation: "",
        limitation_exists: false,

        active_exists: false,
        active: "",

        //
        //stats
        //

        //Offensive:
        ability_power: 0,
        attack_damage: 0,
        attack_speed: 0,
        critical_strike_chance: 0,
        percentage_armor_penetration: 0,
        lethality: 0,
        flat_magic_penetration: 0,
        percentage_magic_penetration: 0,
        life_steal: 0,
        omnivamp: 0,
        physical_vamp: 0,

        //Defensive:
        health: 0,
        health_regeneration: 0,
        heal_shield_power: 0,
        armor: 0,
        magic_resist: 0,
        tenacity: [0; 2], //[value, type]

        //Utility:
        ability_haste: 0,
        mana: 0,
        mana_regeneration: 0,
        movement_speed: 0,

        summoner_spell_haste: 0,
        item_haste: 0,
    };
}
