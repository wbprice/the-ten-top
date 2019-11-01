use petgraph::{graphmap::GraphMap, Directed, Direction};

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Food {
    Actions(Actions),
    Dishes(Dishes),
    Ingredients(Ingredients),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Actions {
    Cook,
    Chop,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Dishes {
    Hamburger,
    HotDog,
    Taco,
    Elote,
    Takoyaki,
    Fishballs,
    BanhMi,
    Pho,
    Dumplings,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Ingredients {
    HamburgerBun,
    HamburgerPatty,
    HamburgerPattyCooked,
    Lettuce,
    Tomato,
    HotDogBun,
    HotDogWeiner,
    HotDogWeinerCooked,
}

#[derive(Debug, Default)]
pub struct Cookbook {
    graph: GraphMap<Food, f64, Directed>,
}

impl Cookbook {
    pub fn new() -> Cookbook {
        let mut graph = GraphMap::new();

        // Add nodes for actions
        graph.add_node(Food::Actions(Actions::Cook));

        // Add nodes for ingredients and dishes
        graph.add_node(Food::Ingredients(Ingredients::HotDogBun));
        graph.add_node(Food::Ingredients(Ingredients::HotDogWeiner));
        graph.add_node(Food::Ingredients(Ingredients::HotDogWeinerCooked));
        graph.add_node(Food::Dishes(Dishes::HotDog));

        // Add edges
        graph.add_edge(
            Food::Actions(Actions::Cook),
            Food::Ingredients(Ingredients::HotDogWeinerCooked),
            1.,
        );
        graph.add_edge(
            Food::Ingredients(Ingredients::HotDogWeiner),
            Food::Ingredients(Ingredients::HotDogWeinerCooked),
            1.,
        );
        graph.add_edge(
            Food::Ingredients(Ingredients::HotDogWeinerCooked),
            Food::Dishes(Dishes::HotDog),
            1.,
        );
        graph.add_edge(
            Food::Ingredients(Ingredients::HotDogBun),
            Food::Dishes(Dishes::HotDog),
            1.,
        );

        Cookbook { graph }
    }

    pub fn actions(&self, food_node: Food) -> Vec<Actions> {
        let mut actions: Vec<Actions> = vec![];

        for node in self
            .graph
            .neighbors_directed(food_node, Direction::Incoming)
        {
            if let Food::Actions(action) = node {
                actions.push(action);
            }
        }

        actions
    }

    pub fn ingredients(&self, food_node: Food) -> Vec<Ingredients> {
        let mut ingredients: Vec<Ingredients> = vec![];

        for node in self
            .graph
            .neighbors_directed(food_node, Direction::Incoming)
        {
            if let Food::Ingredients(ingredient) = node {
                ingredients.push(ingredient);
            }
        }

        ingredients
    }

    pub fn makes(&self, food_node: Food) -> Vec<Dishes> {
        let mut dishes: Vec<Dishes> = vec![];

        for node in self
            .graph
            .neighbors_directed(food_node, Direction::Outgoing)
        {
            if let Food::Dishes(dish) = node {
                dishes.push(dish);
            }
        }

        dishes
    }
}
