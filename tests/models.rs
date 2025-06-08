use dairyshop::models::{Yak, Herd};

#[test]
fn test_milk_production_for_day() {
    let yak = Yak { name: "Yaka".to_string(), initial_age_years: 5.0, sex: 'f' };
    // On day 0, milk = 50 - (age_days * 0.03)
    let milk = yak.milk_production_for_day(0);
    assert!((milk - 35.0).abs() < f64::EPSILON);
}

#[test]
fn test_is_alive() {
    let yak = Yak { name: "Oldy".to_string(), initial_age_years: 9.99, sex: 'f' };
    assert!(yak.is_alive(1));
    // After more than 10 years total age the yak dies
    assert!(!yak.is_alive(2));
}

#[test]
fn test_herd_calculate_stock_single_day() {
    let yak = Yak { name: "Betsy".to_string(), initial_age_years: 5.0, sex: 'f' };
    let herd = Herd { yaks: vec![yak] };
    let stock = herd.calculate_stock(1); // compute day 0 only
    assert!((stock.milk_liters - 35.0).abs() < f64::EPSILON);
    assert_eq!(stock.wool_skins, 1);
}

#[test]
fn test_all_yaks_shaved_day_zero() {
    let herd = Herd {
        yaks: vec![
            Yak { name: "Yak1".to_string(), initial_age_years: 4.0, sex: 'f' },
            Yak { name: "Yak2".to_string(), initial_age_years: 8.0, sex: 'f' },
            Yak { name: "Yak3".to_string(), initial_age_years: 9.5, sex: 'f' },
        ],
    };

    // Day 0 is the shop opening day so all mature yaks can be shaved
    let stock = herd.calculate_stock(1);

    assert_eq!(stock.wool_skins, 3);
    assert!((stock.milk_liters - 85.5).abs() < 1e-6);
}

#[test]
fn test_shave_interval_four_year_old() {
    let herd = Herd {
        yaks: vec![Yak { name: "Yakky".to_string(), initial_age_years: 4.0, sex: 'f' }],
    };

    // Up to day 12 elapsed (T=13) -> the yak should have been shaved only once
    let stock_before = herd.calculate_stock(13);
    assert_eq!(stock_before.wool_skins, 1);
    assert!((stock_before.milk_liters - 491.66).abs() < 1e-2);

    // Including day 13 (T=14) -> the yak becomes eligible again
    let stock_after = herd.calculate_stock(14);
    assert_eq!(stock_after.wool_skins, 2);
    assert!((stock_after.milk_liters - 529.27).abs() < 1e-2);
}
