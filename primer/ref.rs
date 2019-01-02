use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
fn sort_works(table: &mut Table) {
    for (_artist, works)  in table {
        works.sort();
    }
}

pub fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The musicans".to_string(),
            "The calling of st.matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of medusa".to_string(),
            "A salt cellar".to_string(),
        ],
    );

    show(&table);
    sort_works(&mut table);
    println!("----After sorting----");
    show(&table);
}
