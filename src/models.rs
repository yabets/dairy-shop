use std::collections::HashSet;

pub struct Yak {
    pub name: String,
    pub initial_age_years: f32,
    pub sex: char,
}

pub struct Herd {
    pub yaks: Vec<Yak>,
}

pub struct Stock {
    pub milk_liters: f64,
    pub wool_skins: u32,
}

impl Yak {
    pub fn is_alive(&self, day: u32) -> bool {
        let current_age_years = self.current_age_years(day);
        current_age_years < 10.0
    }

    pub fn can_shave(&self, day: u32) -> bool {
        if !self.is_alive(day) {
            return false;
        }

        // Yaks must be at least 1 year (100 days) old to be shaved
        let age_days = (self.initial_age_years * 100.0) as u32 + day;
        age_days >= 100
    }


    fn shave_interval(&self, day: u32) -> u32 {
        let age_days = (self.initial_age_years * 100.0) as u32 + day;
        (8.0 + (age_days as f64) * 0.01).ceil() as u32
    }

    pub fn current_age_years(&self, day: u32) -> f64 {
        self.initial_age_years as f64 + (day as f64 / 100.0)
    }

    pub fn milk_production_for_day(&self, day: u32) -> f64 {
        if !self.is_alive(day) {
            return 0.0;
        }

        // Only female yaks produce milk
        if self.sex != 'f' {
            return 0.0;
        }

        let age_days = (self.initial_age_years * 100.0) as u32 + day;
        let milk = 50.0 - (age_days as f64 * 0.03);
        if milk < 0.0 { 0.0 } else { milk }
    }
}

impl Herd {
    pub fn calculate_stock(&self, days_elapsed: u32) -> Stock {
        let mut milk_liters = 0.0;
        let mut wool_skins = 0;
        let mut shaved_days: Vec<HashSet<String>> = vec![HashSet::new(); days_elapsed as usize + 1];

        // Calculate milk production and wool collection for each day
        for day in 0..days_elapsed {
            for yak in &self.yaks {
                // Add milk production for this day
                milk_liters += yak.milk_production_for_day(day);

                // Check if the yak can be shaved today
                if yak.can_shave(day) {
                    let mut can_be_shaved = true;

                    // Check if this yak was already shaved within its interval
                    for prev_day in (0..day).rev() {
                        if shaved_days[prev_day as usize].contains(&yak.name) {
                            let interval = yak.shave_interval(prev_day);
                            if day - prev_day <= interval {
                                can_be_shaved = false;
                                break;
                            }
                        }
                    }

                    if can_be_shaved {
                        wool_skins += 1;
                        shaved_days[day as usize].insert(yak.name.clone());
                    }
                }
            }
        }

        Stock {
            milk_liters,
            wool_skins,
        }
    }
}
