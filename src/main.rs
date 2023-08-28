mod aion_relics;

use clap::Parser;
pub use crate::aion_relics::{Owner, create_item_mapping, distribute_values, Item};

/// Simple relic calculator for Aion
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of people to split for
    #[arg(required(true), short, long, default_value_t = 1)]
    count: u8,

    /// No initial values for players
    #[arg(short, long)]
    no_init: bool
}

fn read_relics() -> Vec<Item> {
    let mut relics: Vec<String> = Vec::new();

    loop {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            println!("Error reading line.");
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        let mut done_found = false;
        for word in line.split_whitespace() {
            if word == "done" {
                done_found = true;
                break;
            }
            relics.push(word.to_string());
        }

        if done_found {
            break;
        }
    }

    let mapping = create_item_mapping();

    let mut relic_items: Vec<Item> = Vec::new();

    for relic in relics {
        if let Some(item) = mapping.get(&relic) {
            relic_items.push(Item { name: item.name, value: item.value });
        }
    }

    relic_items
}

fn read_owners<'a>(count: i32) -> Vec<Owner<'a>> {
    let mut owners: Vec<Owner> = Vec::new();
    for i in 0..count {
        // ask for init value for each owner from command line
        println!("Enter initial value for owner {}:", i + 1);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let init_value = input.trim().parse().expect("Please type a number!");

        owners.push(Owner {
            name: format!("Owner {}", i + 1),
            value: init_value,
            items: Vec::new()
        });

    }

    owners
}

fn default_owners<'a>(count: i32) -> Vec<Owner<'a>> {
    let mut owners: Vec<Owner> = Vec::new();
    for i in 0..count {
        owners.push(Owner {
            name: format!("Owner {}", i + 1),
            value: 0,
            items: Vec::new()
        });
    }

    owners
}

fn main() {
    let args = Args::parse();

    println!("Splitting for {} people, please input relics", args.count);

    let mut owners = if args.no_init { default_owners(args.count as i32) }
        else { read_owners(args.count as i32) };

    let relics = read_relics();
    distribute_values(&relics, &mut owners);

    for owner in owners {
        println!("{}: {}", owner.name, owner.value);
        for item in owner.items {
            println!("  {}: {}", item.name, item.value);
        }
    }

}
