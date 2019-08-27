# Design Document

The Ten Top is a game about owning and operating a small restaurant.

## Project Goals

Goals:

- Make a fun, casual game about running a restaurant
- Learn game design and development concepts

Non-goals:

- Create a hardcore simulation of restaurateur life: Fun should take priority over realism

## Game Objectives

The object of the game is to make enough money to:

- Cover operating costs of the restaurant every month
- Pay the worker(s) every two weeks
- Eventually sell the restaurant and retire

If bi-weekly and monthly objectives aren't met, various bad things happen:

- Workers quit
- The bank seizes the restaurant (this ends the game immediately)

## Game Mechanics

The primary way players achieve the objectives is to sell food at a profit to patrons.

### Units

There are two types of entity in the game:

- Patrons
- Workers

#### Patron Behaviors

Patrons are people who are willing to exchange money for food.

- Patrons walk by the restaurant
- If the restaurant caters to their craving, they will enter the restaurant
- The patron will order food
- The patron will wait for the food to be ready
- Each patron has a set amount of satisfaction
- Patron satisfaction decreases as they wait for food
- If satisfaction is depleted, they will ask for a refund and leave
- If satisfaction goes up, their chances of coming back to the restaurant is increased
- If patron satisfaction is high enough, they'll invite their friends!

#### Worker Behaviors

Workers are people who are willing to exhange time for money.
The restaurant owner employs some number of workers to help them handle Patron requests.

- Workers work at the restaurant every day
- Workers perform tasks associated with serving food
  - Cooking
  - Cleaning
  - Taking orders
- Workers perform one task at a time
  - If they are cooking, they can't clean
  - If they are taking orders, they can't cook
  - and so on
- Each worker begins the day with a set amount of satisfaction
- Satisfaction decreases as time goes on
- Satisfaction is boosted whenever the worker gets tips
- Satisfaction is decreased if a patron complains
- If satisfaction is depleted, they will stop working for the day
- If workers can't be paid, they won't come to work

## Inspirations

- Overcooked
- Oxygen Not Included
- The (actual) Ten Top (Norfolk, VA)
