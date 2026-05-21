const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "winter";
    let mut points_scored: i32 = 28;
    println!("Score default pts {points_scored}");
    points_scored = 35;

    let event_time: &str = "06:00";
    println!("The event time is {} PM", event_time);
    let event_time: i32 = 6;

    println!("I like {season} season. The team scored {points_scored}.");
    println!(
        "The event time is {0} PM and touchdown pts is worth {1}.",
        event_time, TOUCHDOWN_POINTS
    );

    let _favorite_beverage: &str = "CocaCola";
}
