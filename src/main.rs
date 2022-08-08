use activity::Activity;

use crate::activity_util::list_by_tag;

pub mod activity;
pub mod activity_util;

fn main() {
    println!("First Struct");

    let test_activity = Activity {
        name: String::from("test_activity_name"),
        tags: vec![String::from("tag1"), String::from("tag2")],
    };

    let test_activity_2 = Activity {
        name: String::from("test_activity_name"),
        tags: vec![
            String::from("tag1"),
            String::from("tag2"),
            String::from("tag3"),
        ],
    };

    println!(
        "test_activity: Name {} - tags {:?}",
        test_activity.name, test_activity.tags
    );

    let activities = vec![test_activity, test_activity_2];

    let found_activites = list_by_tag(String::from("tag3"), activities);

    println!("found_activites: {:#?}", found_activites);
}
