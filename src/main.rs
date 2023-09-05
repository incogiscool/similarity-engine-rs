use std::time::Instant;

#[derive(Debug)]
struct Item {
    rating: Vec<i32>,
    title: String,
    description: String, // Corrected the field name to "description"
    id: i32,
}

#[derive(Debug)]
struct Key {
    title: String,
    weight: i32,
}

#[derive(Debug)]
struct Similarity {
    keys: Vec<Key>,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
struct SimilarityArrayObject<'a> {
    id: i32,
    against: &'a Item,
    similarity: f64,
    title: &'a String,
}

impl Similarity {
    pub fn create_key(&mut self, key: Key) {
        self.keys.push(key);
    }

    pub fn create_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn dot_product(&self, item_1: &Item, item_2: &Item) -> i32 {
        if item_1.rating.len() != item_2.rating.len() {
            panic!("All Items ratings length must be equal.")
        };

        let dot_product = item_1
            .rating
            .iter()
            .zip(item_2.rating.iter())
            .map(|(&x, &y)| x * y)
            .sum();

        return dot_product;
    }

    pub fn magnitude(&self, item: &Item) -> f64 {
        let values_squared_added: i32 = item.rating.iter().map(|rating| rating * rating).sum();

        return f64::from(values_squared_added).sqrt();
    }

    pub fn cosine_similarity(&self, item_1: &Item, item_2: &Item) -> f64 {
        let dot_product = self.dot_product(item_1, item_2);
        let magnitude_1 = self.magnitude(item_1);
        let magnitude_2 = self.magnitude(item_2);

        if magnitude_1 == 0.0 || magnitude_2 == 0.0 {
            return 0.0;
        };

        return f64::from(dot_product) / (magnitude_1 * magnitude_2);
    }

    pub fn get_similar(&self, item_id: i32) -> Vec<SimilarityArrayObject> {
        let item = self
            .items
            .iter()
            .find(|item| item.id == item_id)
            .expect("Couldn't find item");

        let mut all_items_calculated: Vec<SimilarityArrayObject> = vec![];

        for current_item in self.items.iter() {
            if current_item.id == item.id {
                continue;
            }

            let value = self.cosine_similarity(item, &current_item);

            all_items_calculated.push(SimilarityArrayObject {
                id: current_item.id,
                similarity: value,
                against: current_item,
                title: &current_item.title,
            });
        }

        // Sort the similarities in descending order
        all_items_calculated.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take the top 5 items
        let top_5_similar = all_items_calculated
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>();

        return top_5_similar;
    }
}

fn init_test_similarity_engine() -> Similarity {
    let mut engine = Similarity {
        keys: vec![],
        items: vec![],
    };

    engine.create_key(Key {
        title: String::from("Comedy"),
        weight: 3,
    });

    engine.create_key(Key {
        title: String::from("Action"),
        weight: 2,
    });

    engine.create_item(Item {
        title: String::from("Brooklyn 99"),
        rating: vec![10, 4],
        description: String::from("Its a show..."),
        id: 1,
    });

    engine.create_item(Item {
        title: String::from("Rush Hour 2"),
        rating: vec![10, 7],
        description: String::from("Its a movie..."),
        id: 2,
    });

    engine.create_item(Item {
        title: String::from("Rush Hour 3"),
        rating: vec![8, 7],
        description: String::from("Its a  nother movie..."),
        id: 3,
    });

    engine.create_item(Item {
        title: String::from("my custom action movie"),
        rating: vec![4, 10],
        description: String::from("Its a action movie..."),
        id: 4,
    });

    engine.create_item(Item {
        title: String::from("John Wick"),
        rating: vec![2, 9],
        description: String::from("Its another movie..."),
        id: 5,
    });

    engine.create_item(Item {
        rating: vec![8, 7],
        title: String::from("Central Intelligence"),
        description: String::from("movi"),
        id: 6,
    });

    engine.create_item(Item {
        title: String::from("The Hangover"),
        rating: vec![9, 2],
        description: String::from("It's a comedy..."),
        id: 7,
    });

    engine.create_item(Item {
        title: String::from("Die Hard"),
        rating: vec![3, 10],
        description: String::from("It's an action movie..."),
        id: 8,
    });

    engine.create_item(Item {
        title: String::from("Forrest Gump"),
        rating: vec![9, 2],
        description: String::from("A classic..."),
        id: 9,
    });

    engine.create_item(Item {
        title: String::from("The Matrix"),
        rating: vec![4, 10],
        description: String::from("Sci-fi action..."),
        id: 10,
    });

    return engine;
}

fn main() {
    let engine = init_test_similarity_engine();

    let id = 5;

    let id_item = engine
        .items
        .iter()
        .find(|item| item.id == id)
        .expect("Invalid id");

    println!("Finding top 5 for {}...\n", id_item.title);

    let start_time = Instant::now();

    let similar = engine.get_similar(id);

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    similar.iter().enumerate().for_each(|(index, item)| {
        println!(
            "Item: {}\nPlacement: {}\nScore: {}%\n",
            item.title,
            index + 1,
            (item.similarity * 100.00).round()
        )
    });

    println!("Execution time: {:?}", elapsed_time);
}
