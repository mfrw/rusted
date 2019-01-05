use std::collections::btree_map::BTreeMap;

struct Person {
    blood_alchol: f32,
}

pub fn main() {
    let orders = vec![1, 2, 1, 2, 3, 2, 2, 2, 2, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    let mut blood_alchol = BTreeMap::new();

    for id in orders {
        let person = blood_alchol
            .entry(id)
            .or_insert(Person { blood_alchol: 0.0 });
        person.blood_alchol *= 0.9;

        if person.blood_alchol > 0.3 {
            println!("Sorry {} youre too drunk", id);
        } else {
            person.blood_alchol += 0.1;
        }
    }
}
