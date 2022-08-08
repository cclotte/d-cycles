use crate::activity::Activity;

pub fn list_by_tag(tag_name: String, activites: Vec<Activity>) -> Vec<Activity> {
    let mut matched_activites: Vec<Activity> = Vec::new();

    for act in activites {
        if act.tags.contains(&tag_name) {
            matched_activites.push(act);
        }
    }
    return matched_activites;
}

#[test]
fn tag_search_test() {
    unimplemented!();
}
