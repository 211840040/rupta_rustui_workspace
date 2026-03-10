// Animal Hierarchy Test Module
//
// 本模块演示使用 classes 宏实现的动物层次结构系统
// 包括抽象类、继承、多态、mixin 以及类型转换

// 声明所有子模块
mod animal;
mod bird;
mod cat;
mod dog;
mod fish;
mod mixins;

// 重新导出主要类型供测试使用
pub use animal::Animal;
pub use bird::{Bird, Duck, Eagle, Ostrich, Penguin};
pub use cat::Cat;
pub use dog::Dog;
pub use fish::{Fish, FlyingFish, Salmon, Shark};

pub fn main() {
    println!("\n=== Property Test: Mixin methods callable ===");

    // 测试 Feathered mixin 方法（通过 Bird 类）
    println!("\n--- Testing Feathered mixin methods ---");
    let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);
    let eagle_name = eagle.get_name().as_ref().unwrap().clone();
    let eagle_feather_color = eagle.get_feather_color().as_ref().unwrap().clone();

    // 调用 Feathered mixin 方法
    let preen_result = eagle.preen_feathers();
    println!("Eagle preen_feathers(): {}", preen_result);

    // 验证返回字符串非空且包含实例信息
    assert!(
        !preen_result.is_empty(),
        "preen_feathers() should return non-empty string"
    );
    assert!(
        preen_result.contains(&eagle_name),
        "preen_feathers() should contain animal name"
    );
    assert!(
        preen_result.contains(&eagle_feather_color),
        "preen_feathers() should contain feather color"
    );

    // 测试 Penguin 的 Feathered mixin
    let penguin = Penguin::new(
        "Pingu".to_string(),
        4,
        0.8,
        "Black and White".to_string(),
        5.0,
        1000,
    );
    let penguin_name = penguin.get_name().as_ref().unwrap().clone();
    let penguin_feather_color = penguin.get_feather_color().as_ref().unwrap().clone();

    let preen_result = penguin.preen_feathers();
    println!("Penguin preen_feathers(): {}", preen_result);

    assert!(
        !preen_result.is_empty(),
        "Penguin preen should be non-empty"
    );
    assert!(
        preen_result.contains(&penguin_name),
        "Penguin preen should contain name"
    );
    assert!(
        preen_result.contains(&penguin_feather_color),
        "Penguin preen should contain feather color"
    );

    println!("✓ Feathered mixin methods are callable and return valid results");

    // 测试 Scaled mixin 方法（通过 Fish 类）
    println!("\n--- Testing Scaled mixin methods ---");
    let shark = Shark::new(
        "Jaws".to_string(),
        10,
        "Saltwater".to_string(),
        "Gray".to_string(),
        300,
        15.0,
    );
    let shark_name = shark.get_name().as_ref().unwrap().clone();
    let shark_scale_pattern = shark.get_scale_pattern().as_ref().unwrap().clone();

    // 调用 Scaled mixin 方法
    let shed_result = shark.shed_scales();
    println!("Shark shed_scales(): {}", shed_result);

    // 验证返回字符串非空且包含实例信息
    assert!(
        !shed_result.is_empty(),
        "shed_scales() should return non-empty string"
    );
    assert!(
        shed_result.contains(&shark_name),
        "shed_scales() should contain animal name"
    );
    assert!(
        shed_result.contains(&shark_scale_pattern),
        "shed_scales() should contain scale pattern"
    );

    // 测试 Salmon 的 Scaled mixin
    let salmon = Salmon::new(
        "Sockeye".to_string(),
        4,
        "Freshwater".to_string(),
        "Silver".to_string(),
        "Alaska River".to_string(),
        8.0,
    );
    let salmon_name = salmon.get_name().as_ref().unwrap().clone();
    let salmon_scale_pattern = salmon.get_scale_pattern().as_ref().unwrap().clone();

    let shed_result = salmon.shed_scales();
    println!("Salmon shed_scales(): {}", shed_result);

    assert!(!shed_result.is_empty(), "Salmon shed should be non-empty");
    assert!(
        shed_result.contains(&salmon_name),
        "Salmon shed should contain name"
    );
    assert!(
        shed_result.contains(&salmon_scale_pattern),
        "Salmon shed should contain scale pattern"
    );

    println!("✓ Scaled mixin methods are callable and return valid results");

    // 测试 Flyable mixin 方法
    println!("\n--- Testing Flyable mixin methods ---");
    let eagle2 = Eagle::new(
        "Thunder".to_string(),
        8,
        2.8,
        "Dark Brown".to_string(),
        4000.0,
        60.0,
    );
    let eagle2_name = eagle2.get_name().as_ref().unwrap().clone();
    let eagle2_max_altitude = eagle2.get_max_altitude();

    // 调用 Flyable mixin 方法
    let fly_result = eagle2.fly();
    println!("Eagle fly(): {}", fly_result);

    // 验证返回字符串非空且包含实例信息
    assert!(
        !fly_result.is_empty(),
        "fly() should return non-empty string"
    );
    assert!(
        fly_result.contains(&eagle2_name),
        "fly() should contain animal name"
    );
    assert!(
        fly_result.contains(&eagle2_max_altitude.to_string()),
        "fly() should contain max altitude"
    );

    // 测试 Duck 的 Flyable mixin
    let duck = Duck::new(
        "Daffy".to_string(),
        2,
        1.0,
        "Brown".to_string(),
        800.0,
        8.0,
        3000.0,
    );
    let duck_name = duck.get_name().as_ref().unwrap().clone();
    let duck_max_altitude = duck.get_max_altitude();

    let fly_result = duck.fly();
    println!("Duck fly(): {}", fly_result);

    assert!(!fly_result.is_empty(), "Duck fly should be non-empty");
    assert!(
        fly_result.contains(&duck_name),
        "Duck fly should contain name"
    );
    assert!(
        fly_result.contains(&duck_max_altitude.to_string()),
        "Duck fly should contain max altitude"
    );

    // 测试 FlyingFish 的 Flyable mixin
    let flying_fish = FlyingFish::new(
        "Skipper".to_string(),
        1,
        "Saltwater".to_string(),
        "Silver".to_string(),
        400.0,
        10.0,
        40.0,
    );
    let fish_name = flying_fish.get_name().as_ref().unwrap().clone();
    let fish_max_altitude = flying_fish.get_max_altitude();

    let fly_result = flying_fish.fly();
    println!("FlyingFish fly(): {}", fly_result);

    assert!(!fly_result.is_empty(), "FlyingFish fly should be non-empty");
    assert!(
        fly_result.contains(&fish_name),
        "FlyingFish fly should contain name"
    );
    assert!(
        fly_result.contains(&fish_max_altitude.to_string()),
        "FlyingFish fly should contain max altitude"
    );

    println!("✓ Flyable mixin methods are callable and return valid results");

    // 测试 Swimmable mixin 方法
    println!("\n--- Testing Swimmable mixin methods ---");
    let penguin2 = Penguin::new(
        "Skipper".to_string(),
        5,
        0.9,
        "Black and White".to_string(),
        6.0,
        500,
    );
    let penguin2_name = penguin2.get_name().as_ref().unwrap().clone();
    let penguin2_swim_speed = penguin2.get_swim_speed();

    // 调用 Swimmable mixin 方法
    let swim_result = penguin2.swim();
    println!("Penguin swim(): {}", swim_result);

    // 验证返回字符串非空且包含实例信息
    assert!(
        !swim_result.is_empty(),
        "swim() should return non-empty string"
    );
    assert!(
        swim_result.contains(&penguin2_name),
        "swim() should contain animal name"
    );
    assert!(
        swim_result.contains(&penguin2_swim_speed.to_string()),
        "swim() should contain swim speed"
    );

    // 测试 Duck 的 Swimmable mixin
    let duck2 = Duck::new(
        "Quackers".to_string(),
        3,
        1.1,
        "White".to_string(),
        900.0,
        9.0,
        4000.0,
    );
    let duck2_name = duck2.get_name().as_ref().unwrap().clone();
    let duck2_swim_speed = duck2.get_swim_speed();

    let swim_result = duck2.swim();
    println!("Duck swim(): {}", swim_result);

    assert!(!swim_result.is_empty(), "Duck swim should be non-empty");
    assert!(
        swim_result.contains(&duck2_name),
        "Duck swim should contain name"
    );
    assert!(
        swim_result.contains(&duck2_swim_speed.to_string()),
        "Duck swim should contain swim speed"
    );

    // 测试 Shark 的 Swimmable mixin
    let shark2 = Shark::new(
        "Bruce".to_string(),
        12,
        "Saltwater".to_string(),
        "Blue-Gray".to_string(),
        350,
        18.0,
    );
    let shark2_name = shark2.get_name().as_ref().unwrap().clone();
    let shark2_swim_speed = shark2.get_swim_speed();

    let swim_result = shark2.swim();
    println!("Shark swim(): {}", swim_result);

    assert!(!swim_result.is_empty(), "Shark swim should be non-empty");
    assert!(
        swim_result.contains(&shark2_name),
        "Shark swim should contain name"
    );
    assert!(
        swim_result.contains(&shark2_swim_speed.to_string()),
        "Shark swim should contain swim speed"
    );

    // 测试 Salmon 的 Swimmable mixin
    let salmon2 = Salmon::new(
        "Chinook".to_string(),
        3,
        "Freshwater".to_string(),
        "Red".to_string(),
        "Columbia River".to_string(),
        7.0,
    );
    let salmon2_name = salmon2.get_name().as_ref().unwrap().clone();
    let salmon2_swim_speed = salmon2.get_swim_speed();

    let swim_result = salmon2.swim();
    println!("Salmon swim(): {}", swim_result);

    assert!(!swim_result.is_empty(), "Salmon swim should be non-empty");
    assert!(
        swim_result.contains(&salmon2_name),
        "Salmon swim should contain name"
    );
    assert!(
        swim_result.contains(&salmon2_swim_speed.to_string()),
        "Salmon swim should contain swim speed"
    );

    // 测试 FlyingFish 的 Swimmable mixin
    let flying_fish2 = FlyingFish::new(
        "Jumper".to_string(),
        2,
        "Saltwater".to_string(),
        "Blue".to_string(),
        450.0,
        11.0,
        45.0,
    );
    let fish2_name = flying_fish2.get_name().as_ref().unwrap().clone();
    let fish2_swim_speed = flying_fish2.get_swim_speed();

    let swim_result = flying_fish2.swim();
    println!("FlyingFish swim(): {}", swim_result);

    assert!(
        !swim_result.is_empty(),
        "FlyingFish swim should be non-empty"
    );
    assert!(
        swim_result.contains(&fish2_name),
        "FlyingFish swim should contain name"
    );
    assert!(
        swim_result.contains(&fish2_swim_speed.to_string()),
        "FlyingFish swim should contain swim speed"
    );

    println!("✓ Swimmable mixin methods are callable and return valid results");

    println!("\n✓ Property verified: All mixin methods are callable and return valid results");
    println!("  Tested mixins: Feathered, Scaled, Flyable, Swimmable");
    println!("  All methods returned non-empty strings containing instance information");
}
