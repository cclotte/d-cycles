use activity::Activity;

pub mod activity;

fn main() {
    println!("First Struct");

    let test_activity = Activity {
        name: String::from("test_activity_name"),
        tags: vec![String::from("tag1"), String::from("tag2")],
    };

    println!(
        "test_activity: Name {} - tags {:?}",
        test_activity.name, test_activity.tags
    );
}
