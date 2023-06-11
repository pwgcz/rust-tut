// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
// fn give_adult(drink: Option<&str>) {
//     // Specify a course of action for each case.
//     match drink {
//         Some("lemonade") => println!("Yuck! Too sugary."),
//         Some(inner)   => println!("{}? How nice.", inner),
//         None          => println!("No drink? Oh well."),
//     }
// }

// // Others will `panic` before drinking sugary drinks.
// // All drinks are handled implicitly using `unwrap`.
// fn drink(drink: Option<&str>) {
//     // `unwrap` returns a `panic` when it receives a `None`.
//     let inside = drink.unwrap();
//     if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

//     println!("I love {}s!!!!!", inside);
// }

// fn next_birthday(current_age: Option<u8>) -> Option<String> {
// 	// If `current_age` is `None`, this returns `None`.
// 	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
//     let next_age: u8 = current_age? + 1;
//     Some(format!("Next year I will be {}", next_age))
// }

// #[derive(Debug)]
// struct Person {
//     job: Option<Job>,
// }

// #[derive(Clone, Copy, Debug)]
// struct Job {
//     phone_number: Option<PhoneNumber>,
// }

// #[derive(Clone, Copy, Debug)]
// struct PhoneNumber {
//     area_code: Option<u8>,
//     number: u8,
// }

// impl Person {

//     // Gets the area code of the phone number of the person's job, if it exists.
//     fn work_phone_area_code(&self) -> Option<u8> {
//         // This would need many nested `match` statements without the `?` operator.
//         // It would take a lot more code - try writing it yourself and see which
//         // is easier.
//         self.job?.phone_number?.area_code
//     }
// }
//3.
// #![allow(dead_code)]

// #[derive(Debug)] enum Food { Apple, Carrot, Potato }

// #[derive(Debug)] struct Peeled(Food);
// #[derive(Debug)] struct Chopped(Food);
// #[derive(Debug)] struct Cooked(Food);

// // Peeling food. If there isn't any, then return `None`.
// // Otherwise, return the peeled food.
// fn peel(food: Option<Food>) -> Option<Peeled> {
//     match food {
//         Some(food) => Some(Peeled(food)),
//         None       => None,
//     }
// }

// // Chopping food. If there isn't any, then return `None`.
// // Otherwise, return the chopped food.
// fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
//     match peeled {
//         Some(Peeled(food)) => Some(Chopped(food)),
//         None               => None,
//     }
// }

// // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
// fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
//     chopped.map(|Chopped(food)| Cooked(food))
// }

// // A function to peel, chop, and cook food all in sequence.
// // We chain multiple uses of `map()` to simplify the code.
// fn process(food: Option<Food>) -> Option<Cooked> {
//     food.map(|f| Peeled(f))
//         .map(|Peeled(f)| Chopped(f))
//         .map(|Chopped(f)| Cooked(f))
// }

// // Check whether there's food or not before trying to eat it!
// fn eat(food: Option<Cooked>) {
//     match food {
//         Some(food) => println!("Mmm. I love {:?}", food),
//         None       => println!("Oh no! It wasn't edible."),
//     }
// }

//4.
#![allow(dead_code)]
#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    // 1.
    // let water  = Some("water");
    // let lemonade = Some("lemonade");
    // let void  = None;

    // give_adult(water);
    // give_adult(lemonade);
    // give_adult(void);

    // let coffee = Some("coffee");
    // let nothing = None;

    // drink(coffee);
    // drink(nothing);

    // 2.
    // let age  = None;
    // let age = next_birthday(age);
    // println!("AGE: {:?}", age);
    // let p = Person {
    //     job: Some(Job {
    //         phone_number: Some(PhoneNumber {
    //             area_code: Some(61),
    //             number: 0,
    //         }),
    //     }),
    // };
    // println!("RESULT: {:?}", p);
    // assert_eq!(p.work_phone_area_code(), Some(61));

    // 3.
    // let apple = Some(Food::Apple);
    // let carrot = Some(Food::Carrot);
    // let potato = None;

    // let cooked_apple = cook(chop(peel(apple)));
    // let cooked_carrot = cook(chop(peel(carrot)));
    // // Let's try the simpler looking `process()` now.
    // let cooked_potato = process(potato);

    // eat(cooked_apple);
    // eat(cooked_carrot);
    // eat(cooked_potato);

    //4.
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}



// #[derive(Debug)] 
// enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

// fn main() {
//     let apple = Some(Fruit::Apple);
//     let orange = Some(Fruit::Orange);
//     let no_fruit: Option<Fruit> = None;

//     let first_available_fruit = no_fruit.or(orange).or(apple);
//     println!("first_available_fruit: {:?}", first_available_fruit);
//     // first_available_fruit: Some(Orange)

//     // `or` moves its argument.
//     // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
//     // But the variable named `apple` has been moved regardless, and cannot be used anymore.
//     // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
//     // TODO: uncomment the line above to see the compiler error
//  }

// #[derive(Debug)] 
// enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

// fn main() {
//     let apple = Some(Fruit::Apple);
//     let no_fruit: Option<Fruit> = None;
//     let get_kiwi_as_fallback = || {
//         println!("Providing kiwi as fallback");
//         Some(Fruit::Kiwi)
//     };
//     let get_lemon_as_fallback = || {
//         println!("Providing lemon as fallback");
//         Some(Fruit::Lemon)
//     };

//     let first_available_fruit = no_fruit
//         .or_else(get_kiwi_as_fallback)
//         .or_else(get_lemon_as_fallback);
//     println!("first_available_fruit: {:?}", first_available_fruit);
//     // Providing kiwi as fallback
//     // first_available_fruit: Some(Kiwi)

// #[derive(Debug)]
// enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

// fn main() {
//     let mut my_fruit: Option<Fruit> = None;
//     let apple = Fruit::Apple;
//     let first_available_fruit = my_fruit.get_or_insert(apple);
//     println!("first_available_fruit is: {:?}", first_available_fruit);
//     println!("my_fruit is: {:?}", my_fruit);
//     // first_available_fruit is: Apple
//     // my_fruit is: Some(Apple)
//     //println!("Variable named `apple` is moved: {:?}", apple);
//     // TODO: uncomment the line above to see the compiler error
// }

// #[derive(Debug)] 
// enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

// fn main() {
//     let mut my_fruit: Option<Fruit> = None;
//     let get_lemon_as_fallback = || {
//         println!("Providing lemon as fallback");
//         Fruit::Lemon
//     };
//     let first_available_fruit = my_fruit
//         .get_or_insert_with(get_lemon_as_fallback);
//     println!("first_available_fruit is: {:?}", first_available_fruit);
//     println!("my_fruit is: {:?}", my_fruit);
//     // Providing lemon as fallback
//     // first_available_fruit is: Lemon
//     // my_fruit is: Some(Lemon)

//     // If the Option has a value, it is left unchanged, and the closure is not invoked
//     let mut my_apple = Some(Fruit::Apple);
//     let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
//     println!("should_be_apple is: {:?}", should_be_apple);
//     println!("my_apple is unchanged: {:?}", my_apple);
//     // The output is a follows. Note that the closure `get_lemon_as_fallback` is not invoked
//     // should_be_apple is: Apple
//     // my_apple is unchanged: Some(Apple)
// }