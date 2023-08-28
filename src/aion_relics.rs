use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Item {
    pub name: &'static str,
    pub value: i32,
}

#[derive(Debug)]
pub struct Owner<'a> {
    pub name: String,
    pub value: i32,
    pub items: Vec<&'a Item>
}


pub fn distribute_values<'a>(items: &'a Vec<Item>, owners: &mut Vec<Owner<'a>>)  {
    // Sort the numbers in descending order
    let items = items.into_iter().sorted_by(|a, b| b.value.cmp(&a.value));

    for item in items {
        // Sort owners by value
        owners.sort_by(|a, b| a.value.cmp(&b.value));
        // Add the largest number to the owner with the smallest value
        owners[0].value += &item.value;
        owners[0].items.push(item);
    }
}

pub fn create_item_mapping() -> HashMap<String, Item> {
    let items = vec![
        Item { name: "Major Crown", value: 9600 },
        Item { name: "Greater Crown", value: 7200 },
        Item { name: "Normal Crown", value: 4800 },
        Item { name: "Lesser Crown", value: 2400 },
        Item { name: "Major Goblet", value: 4800 },
        Item { name: "Greater Goblet", value: 3600 },
        Item { name: "Normal Goblet", value: 2400 },
        Item { name: "Lesser Goblet", value: 1200 },
        Item { name: "Major Seal", value: 2400 },
        Item { name: "Greater Seal", value: 1800 },
        Item { name: "Normal Seal", value: 1200 },
        Item { name: "Lesser Seal", value: 600 },
        Item { name: "Major Icon", value: 1200 },
        Item { name: "Greater Icon", value: 900 },
        Item { name: "Normal Icon", value: 600 },
        Item { name: "Lesser Icon", value: 300 }
    ];

    let mut mapping = HashMap::new();

    for item in items {
        let initials = item.name.split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_lowercase();
        mapping.insert(initials, item);
    }

    mapping
}