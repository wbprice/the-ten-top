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

    pub fn ingredients(&self, food_node: Food) -> Vec<Food> {
        self.graph
            .neighbors_directed(food_node, Direction::Incoming)
            .collect()
    }

    pub fn makes(&self, food_node: Food) -> Vec<Food> {
        self.graph
            .neighbors_directed(food_node, Direction::Outgoing)
            .collect()
    }
}
