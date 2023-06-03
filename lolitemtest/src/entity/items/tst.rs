

    static UNTITLED: Item =  Item {

        id:  1 ,
        tier: 1,
        name: "",
        price: 0,
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

