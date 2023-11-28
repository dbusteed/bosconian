
#[derive(Clone)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub vert: bool,
}

#[derive(Clone)]
pub struct Rock {
    pub x: f32,
    pub y: f32,
    pub mine: bool,
}

pub const MAX_LEVEL: usize = 3;

//
// LEVEL 1
//
pub struct Level1 {
    pub stars: [Star; 3],
    pub rocks: [Rock; 64],
    pub start_i: usize,
    pub max_i: usize,
    pub start_p: usize,
    pub max_p: usize,
    pub time_limit: usize,
}

pub const LEVEL_1: Level1 = Level1 {
    stars: [
        Star { x: -750.0, y: 350.0, vert: false },
        Star { x: 300.0, y: -700.0, vert: false },
        Star { x: 500.0, y: 700.0, vert: true },
    ],
    rocks: [
        Rock { x: -2400.0, y: 600.0, mine: false },
        Rock { x: -2300.0, y: 2200.0, mine: true },
        Rock { x: -2100.0, y: -1750.0, mine: false },
        Rock { x: -1950.0, y: 1600.0, mine: false },
        Rock { x: -1950.0, y: -600.0, mine: true },
        Rock { x: -1900.0, y: -1350.0, mine: false },
        Rock { x: -1750.0, y: -2100.0, mine: false },
        Rock { x: -1700.0, y: 850.0, mine: true },
        Rock { x: -1600.0, y: -200.0, mine: false },
        Rock { x: -1400.0, y: -1800.0, mine: true },
        Rock { x: -1350.0, y: 0.0, mine: true },
        Rock { x: -1300.0, y: 1250.0, mine: false },
        Rock { x: -1250.0, y: -1200.0, mine: true },
        Rock { x: -1100.0, y: 1850.0, mine: true },
        Rock { x: -1100.0, y: -600.0, mine: true },
        Rock { x: -1050.0, y: 950.0, mine: false },
        Rock { x: -1050.0, y: 700.0, mine: false },
        Rock { x: -950.0, y: 1100.0, mine: false },
        Rock { x: -950.0, y: -1550.0, mine: false },
        Rock { x: -900.0, y: 850.0, mine: false },
        Rock { x: -850.0, y: 1000.0, mine: false },
        Rock { x: -800.0, y: 1150.0, mine: false },
        Rock { x: -800.0, y: -950.0, mine: true },
        Rock { x: -750.0, y: 900.0, mine: false },
        Rock { x: -650.0, y: 200.0, mine: false },
        Rock { x: -600.0, y: 1100.0, mine: false },
        Rock { x: -600.0, y: -400.0, mine: false },
        Rock { x: -550.0, y: 1550.0, mine: false },
        Rock { x: -550.0, y: 900.0, mine: false },
        Rock { x: -400.0, y: -2100.0, mine: false },
        Rock { x: -350.0, y: 1300.0, mine: false },
        Rock { x: -250.0, y: 2150.0, mine: false },
        Rock { x: -250.0, y: -1450.0, mine: true },
        Rock { x: -200.0, y: 850.0, mine: false },
        Rock { x: 100.0, y: 500.0, mine: false },
        Rock { x: 100.0, y: -900.0, mine: true },
        Rock { x: 300.0, y: -500.0, mine: false },
        Rock { x: 350.0, y: 1900.0, mine: true },
        Rock { x: 400.0, y: -2100.0, mine: true },
        Rock { x: 450.0, y: 1700.0, mine: false },
        Rock { x: 450.0, y: 1100.0, mine: true },
        Rock { x: 450.0, y: -1600.0, mine: false },
        Rock { x: 550.0, y: 2250.0, mine: false },
        Rock { x: 650.0, y: 150.0, mine: false },
        Rock { x: 900.0, y: -1800.0, mine: false },
        Rock { x: 1000.0, y: 1400.0, mine: true },
        Rock { x: 1000.0, y: 750.0, mine: false },
        Rock { x: 1000.0, y: -150.0, mine: true },
        Rock { x: 1050.0, y: -750.0, mine: false },
        Rock { x: 1150.0, y: 2000.0, mine: true },
        Rock { x: 1250.0, y: 400.0, mine: false },
        Rock { x: 1350.0, y: -1500.0, mine: true },
        Rock { x: 1500.0, y: 900.0, mine: true },
        Rock { x: 1500.0, y: -2150.0, mine: false },
        Rock { x: 1550.0, y: 1450.0, mine: false },
        Rock { x: 1600.0, y: 2100.0, mine: false },
        Rock { x: 1650.0, y: 50.0, mine: true },
        Rock { x: 1650.0, y: -1700.0, mine: false },
        Rock { x: 1800.0, y: -1050.0, mine: true },
        Rock { x: 2000.0, y: 550.0, mine: false },
        Rock { x: 2050.0, y: 1100.0, mine: false },
        Rock { x: 2050.0, y: -500.0, mine: false },
        Rock { x: 2100.0, y: 1850.0, mine: true },
        Rock { x: 2200.0, y: -2000.0, mine: true },
    ],
    start_i: 3,
    max_i: 6,
    start_p: 3,
    max_p: 6,
    time_limit: 30,
};


//
// LEVEL 2
//
pub struct Level2 {
    pub stars: [Star; 4],
    pub rocks: [Rock; 83],
    pub start_i: usize,
    pub max_i: usize,
    pub start_p: usize,
    pub max_p: usize,
    pub time_limit: usize,
}

pub const LEVEL_2: Level2 = Level2 {
    stars: [
        Star { x: -1400.0, y: 200.0, vert: false },
        Star { x: -500.0, y: 200.0, vert: false },
        Star { x: 850.0, y: 400.0, vert: true },
        Star { x: 850.0, y: -500.0, vert: true },
    ],
    rocks: [
        Rock { x: -2450.0, y: -2050.0, mine: true },
        Rock { x: -2300.0, y: -50.0, mine: true },
        Rock { x: -2250.0, y: 1450.0, mine: true },
        Rock { x: -2100.0, y: 2200.0, mine: false },
        Rock { x: -2100.0, y: -1150.0, mine: false },
        Rock { x: -2050.0, y: -100.0, mine: false },
        Rock { x: -2050.0, y: -1000.0, mine: true },
        Rock { x: -2050.0, y: -1700.0, mine: true },
        Rock { x: -1950.0, y: 450.0, mine: true },
        Rock { x: -1850.0, y: 1550.0, mine: false },
        Rock { x: -1750.0, y: 850.0, mine: false },
        Rock { x: -1600.0, y: -400.0, mine: false },
        Rock { x: -1550.0, y: 0.0, mine: false },
        Rock { x: -1550.0, y: -1050.0, mine: false },
        Rock { x: -1550.0, y: -2300.0, mine: true },
        Rock { x: -1450.0, y: -1450.0, mine: true },
        Rock { x: -1400.0, y: 800.0, mine: false },
        Rock { x: -1350.0, y: 2100.0, mine: false },
        Rock { x: -1250.0, y: 1600.0, mine: true },
        Rock { x: -1150.0, y: -600.0, mine: true },
        Rock { x: -1000.0, y: 1000.0, mine: true },
        Rock { x: -950.0, y: 2200.0, mine: true },
        Rock { x: -950.0, y: -2050.0, mine: true },
        Rock { x: -900.0, y: -1150.0, mine: true },
        Rock { x: -850.0, y: 1350.0, mine: true },
        Rock { x: -800.0, y: 1950.0, mine: false },
        Rock { x: -800.0, y: 400.0, mine: false },
        Rock { x: -800.0, y: -250.0, mine: false },
        Rock { x: -750.0, y: -1400.0, mine: false },
        Rock { x: -750.0, y: -1700.0, mine: false },
        Rock { x: -700.0, y: 1100.0, mine: true },
        Rock { x: -650.0, y: 1500.0, mine: true },
        Rock { x: -450.0, y: -350.0, mine: true },
        Rock { x: -450.0, y: -900.0, mine: false },
        Rock { x: -400.0, y: 2350.0, mine: false },
        Rock { x: -250.0, y: 1900.0, mine: true },
        Rock { x: -250.0, y: -1550.0, mine: true },
        Rock { x: -200.0, y: -2200.0, mine: true },
        Rock { x: -150.0, y: 1650.0, mine: true },
        Rock { x: -150.0, y: -1400.0, mine: false },
        Rock { x: -50.0, y: 900.0, mine: true },
        Rock { x: 0.0, y: 1400.0, mine: false },
        Rock { x: 0.0, y: -700.0, mine: true },
        Rock { x: 100.0, y: -150.0, mine: false },
        Rock { x: 150.0, y: 2150.0, mine: false },
        Rock { x: 150.0, y: -1250.0, mine: true },
        Rock { x: 200.0, y: 1850.0, mine: true },
        Rock { x: 250.0, y: 150.0, mine: true },
        Rock { x: 400.0, y: 1150.0, mine: true },
        Rock { x: 400.0, y: 450.0, mine: false },
        Rock { x: 400.0, y: -2100.0, mine: false },
        Rock { x: 500.0, y: -1400.0, mine: false },
        Rock { x: 550.0, y: 1650.0, mine: true },
        Rock { x: 650.0, y: -500.0, mine: false },
        Rock { x: 800.0, y: -1500.0, mine: false },
        Rock { x: 850.0, y: 2100.0, mine: false },
        Rock { x: 850.0, y: 1500.0, mine: false },
        Rock { x: 850.0, y: 1200.0, mine: false },
        Rock { x: 950.0, y: 800.0, mine: true },
        Rock { x: 1050.0, y: -1100.0, mine: false },
        Rock { x: 1150.0, y: 450.0, mine: true },
        Rock { x: 1250.0, y: 1050.0, mine: false },
        Rock { x: 1300.0, y: -100.0, mine: false },
        Rock { x: 1350.0, y: -1900.0, mine: false },
        Rock { x: 1400.0, y: 2250.0, mine: true },
        Rock { x: 1450.0, y: 2000.0, mine: true },
        Rock { x: 1550.0, y: -100.0, mine: false },
        Rock { x: 1550.0, y: -2150.0, mine: false },
        Rock { x: 1600.0, y: 1550.0, mine: false },
        Rock { x: 1600.0, y: 450.0, mine: false },
        Rock { x: 1700.0, y: -1700.0, mine: false },
        Rock { x: 1850.0, y: 2150.0, mine: false },
        Rock { x: 1850.0, y: 1000.0, mine: false },
        Rock { x: 1900.0, y: 1900.0, mine: true },
        Rock { x: 2000.0, y: 550.0, mine: true },
        Rock { x: 2050.0, y: -500.0, mine: false },
        Rock { x: 2050.0, y: -1050.0, mine: false },
        Rock { x: 2050.0, y: -2100.0, mine: false },
        Rock { x: 2100.0, y: 2350.0, mine: false },
        Rock { x: 2100.0, y: 1100.0, mine: true },
        Rock { x: 2200.0, y: 400.0, mine: false },
        Rock { x: 2200.0, y: -1600.0, mine: false },
        Rock { x: 2400.0, y: 1550.0, mine: true },
    ],
    start_i: 3,
    max_i: 6,
    start_p: 3,
    max_p: 6,
    time_limit: 30,
};


//
// LEVEL 3
//
pub struct Level3 {
    pub stars: [Star; 6],
    pub rocks: [Rock; 100],
    pub start_i: usize,
    pub max_i: usize,
    pub start_p: usize,
    pub max_p: usize,
    pub time_limit: usize,
}

pub const LEVEL_3: Level3 = Level3 {
    stars: [
        Star { x: -1300.0, y: 0.0, vert: false },
        Star { x: -700.0, y: 1000.0, vert: true },
        Star { x: -700.0, y: -1000.0, vert: true },
        Star { x: 700.0, y: 1000.0, vert: false },
        Star { x: 700.0, y: -1000.0, vert: false },
        Star { x: 1300.0, y: 0.0, vert: true },
    ],
    rocks: [
        Rock { x: -2400.0, y: -700.0, mine: false },
        Rock { x: -2400.0, y: -1700.0, mine: false },
        Rock { x: -2400.0, y: -2350.0, mine: false },
        Rock { x: -2350.0, y: 1300.0, mine: false },
        Rock { x: -2350.0, y: 450.0, mine: false },
        Rock { x: -2300.0, y: 2100.0, mine: true },
        Rock { x: -2250.0, y: -400.0, mine: false },
        Rock { x: -2200.0, y: 950.0, mine: true },
        Rock { x: -2150.0, y: -1900.0, mine: false },
        Rock { x: -2100.0, y: 0.0, mine: false },
        Rock { x: -2050.0, y: 2400.0, mine: false },
        Rock { x: -2050.0, y: 1950.0, mine: false },
        Rock { x: -2050.0, y: -750.0, mine: true },
        Rock { x: -1900.0, y: 1500.0, mine: true },
        Rock { x: -1900.0, y: 400.0, mine: true },
        Rock { x: -1850.0, y: 1150.0, mine: false },
        Rock { x: -1800.0, y: -1500.0, mine: true },
        Rock { x: -1700.0, y: -1200.0, mine: false },
        Rock { x: -1650.0, y: 2300.0, mine: true },
        Rock { x: -1600.0, y: 1850.0, mine: false },
        Rock { x: -1600.0, y: -50.0, mine: false },
        Rock { x: -1600.0, y: -100.0, mine: false },
        Rock { x: -1500.0, y: 800.0, mine: true },
        Rock { x: -1500.0, y: -1550.0, mine: true },
        Rock { x: -1500.0, y: -2200.0, mine: false },
        Rock { x: -1350.0, y: -600.0, mine: false },
        Rock { x: -1300.0, y: -950.0, mine: true },
        Rock { x: -1250.0, y: 1300.0, mine: false },
        Rock { x: -1100.0, y: 500.0, mine: false },
        Rock { x: -1050.0, y: 1800.0, mine: true },
        Rock { x: -1050.0, y: -550.0, mine: true },
        Rock { x: -1050.0, y: -2000.0, mine: false },
        Rock { x: -1000.0, y: 800.0, mine: true },
        Rock { x: -950.0, y: -1400.0, mine: false },
        Rock { x: -850.0, y: 2400.0, mine: false },
        Rock { x: -850.0, y: 1800.0, mine: false },
        Rock { x: -800.0, y: -1450.0, mine: false },
        Rock { x: -700.0, y: 0.0, mine: false },
        Rock { x: -700.0, y: -150.0, mine: false },
        Rock { x: -650.0, y: 200.0, mine: true },
        Rock { x: -600.0, y: -400.0, mine: true },
        Rock { x: -550.0, y: 2400.0, mine: true },
        Rock { x: -450.0, y: -850.0, mine: true },
        Rock { x: -400.0, y: -2200.0, mine: false },
        Rock { x: -350.0, y: 1250.0, mine: false },
        Rock { x: -350.0, y: -2200.0, mine: false },
        Rock { x: -250.0, y: 2050.0, mine: true },
        Rock { x: -250.0, y: -1300.0, mine: true },
        Rock { x: -200.0, y: 600.0, mine: true },
        Rock { x: -150.0, y: 1450.0, mine: true },
        Rock { x: -150.0, y: -650.0, mine: true },
        Rock { x: -100.0, y: -1600.0, mine: true },
        Rock { x: 0.0, y: -2050.0, mine: false },
        Rock { x: 50.0, y: 350.0, mine: false },
        Rock { x: 100.0, y: 2000.0, mine: false },
        Rock { x: 150.0, y: 1100.0, mine: true },
        Rock { x: 200.0, y: -250.0, mine: true },
        Rock { x: 200.0, y: -2250.0, mine: false },
        Rock { x: 350.0, y: 2450.0, mine: true },
        Rock { x: 350.0, y: -1550.0, mine: false },
        Rock { x: 450.0, y: 2150.0, mine: true },
        Rock { x: 450.0, y: 1700.0, mine: false },
        Rock { x: 600.0, y: 1050.0, mine: true },
        Rock { x: 600.0, y: -2150.0, mine: true },
        Rock { x: 650.0, y: -400.0, mine: false },
        Rock { x: 700.0, y: 500.0, mine: true },
        Rock { x: 850.0, y: 2200.0, mine: false },
        Rock { x: 850.0, y: -750.0, mine: true },
        Rock { x: 900.0, y: -1150.0, mine: false },
        Rock { x: 950.0, y: 1750.0, mine: true },
        Rock { x: 1000.0, y: -1700.0, mine: true },
        Rock { x: 1050.0, y: 1050.0, mine: false },
        Rock { x: 1050.0, y: -2100.0, mine: false },
        Rock { x: 1250.0, y: 600.0, mine: false },
        Rock { x: 1300.0, y: 2350.0, mine: true },
        Rock { x: 1300.0, y: -1600.0, mine: false },
        Rock { x: 1350.0, y: 1300.0, mine: false },
        Rock { x: 1450.0, y: 1700.0, mine: false },
        Rock { x: 1500.0, y: 1100.0, mine: true },
        Rock { x: 1500.0, y: -1050.0, mine: true },
        Rock { x: 1500.0, y: -2100.0, mine: false },
        Rock { x: 1550.0, y: 0.0, mine: true },
        Rock { x: 1550.0, y: -750.0, mine: false },
        Rock { x: 1600.0, y: 2150.0, mine: false },
        Rock { x: 1650.0, y: 600.0, mine: true },
        Rock { x: 1650.0, y: -1650.0, mine: false },
        Rock { x: 1800.0, y: 950.0, mine: false },
        Rock { x: 1850.0, y: 1600.0, mine: true },
        Rock { x: 1900.0, y: -450.0, mine: false },
        Rock { x: 1900.0, y: -1250.0, mine: false },
        Rock { x: 2000.0, y: 200.0, mine: false },
        Rock { x: 2050.0, y: -700.0, mine: true },
        Rock { x: 2050.0, y: -1900.0, mine: true },
        Rock { x: 2100.0, y: 2350.0, mine: false },
        Rock { x: 2200.0, y: 1200.0, mine: false },
        Rock { x: 2250.0, y: 2150.0, mine: false },
        Rock { x: 2250.0, y: -1050.0, mine: true },
        Rock { x: 2300.0, y: 500.0, mine: true },
        Rock { x: 2350.0, y: -2350.0, mine: false },
        Rock { x: 2400.0, y: -150.0, mine: false },
    ],
    start_i: 3,
    max_i: 6,
    start_p: 3,
    max_p: 6,
    time_limit: 30,
};


