# Implementation Notes

## Tasks

Tasks (and completing them) is a big part of The Ten Top. Most tasks have some kind of
prerequisite tasks that must be completed first.  Completing some tasks leads to the
creation of other tasks. Scheduling these tasks is the key to victory.

### Task Scheduling

Completing a `TakeOrder` task...

```
TakeOrder {
    patron: Entity
}
```

... leads to the creation of a `DeliverOrder` task.

```
DeliverOrder {
    patron: Entity,
    dish: Dish
}
```

In order to complete `DeliverOrder`, various prerequisite tasks must be completed in a specific
order.

```
DeliverOrder {
    patron: Entity
    dish: Dish::Hamburger
}
```

Ingredients shouldn't have to be fetched every time.
In most cases, ingredients can be fetched in batches of a given number
e.g. `FetchIngredient` could deliver a bag of 6 hamburger buns,
so `FetchIngredient` shouldn't have to be scheduled again until the buns are gone.
Completing these tasks leads to entities representing unprepped food being available.

```
// Grab a bag of hamburger buns from storage
FetchIngredient {
    ingredient: Ingredient::HamburgerBun
}

// Grab a head of lettuce from the cooler
FetchIngredient {
    ingredient: Ingredient::Lettuce
}

// Grab a tray of patties from the cooler
FetchIngredient {
    ingredient: Ingredient::HamburgerPatty
}

// Grab a tomato from the cooler
FetchIngredient {
    ingredient: Ingredient::Tomato
}
```

Ingredients shouldn't have to be prepped each time.
After being fetched and prepped, some number of prepped ingredients should be
available for use by the cook.
e.g. After scheduling `FetchIngedient` to get a head of lettuce, `PrepIngredient`
would slice the lettuce into chunks.
Completing these tasks leads to entities representing prepped food being available.

```
// Slice the lettuce
PrepIngredient {
    ingredient: Ingredient::Lettuce
}

// Grill the patty
PrepIngredient {
    ingredient: Ingredient::HamburgerPatty
}

// Slice the tomatoes
PrepIngredient {
    ingredient: Ingredient::Tomato
}
```

After all the above tasks have been completed (and the prepped ingredient entities have been added)
a task for plating the food can be completed.
Completing this task leads to a hamburger entity being added to the world.

```
PlateOrder {
    dish: Dish::Hamburger
}
```

After a hamburger exists, the `DeliverFood` task can be completed by getting the hamburger entity to the patron entity.

#### Subtasks

Tasks can be further broken down into subtasks. For example:

```
TakeOrder {
    patron
}

// can be broken into

move_to { entity }
await_order { entity }
```

```
DeliverOrder {
    patron,
    dish
}

// can be broken into

move_to { food_entity }
pick_up { food_entity }
move_to { person_entity }
give { thing_entity, person_entity }

Each action should have a way of indicating completion to the task scheduler?

