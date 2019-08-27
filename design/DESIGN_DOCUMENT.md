# Design Document

"The Ten Top" is a game about owning and operating a small restaurant.

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

### Example Game Activities

#### Situation A

1. A Patron is attracted by the cuisine offered by the restaurant
2. The Patron enters the store and gets in line
3. The Patron waits in line until they get to talk to the Worker at the register
4. The Worker receives an order for a hamburger and money
5. The Worker leaves the cash register to make food for the Patron
6. The Worker completes making food and brings it back to the Patron
7. The Patron accepts the food. Their satisfaction is increased!
8. The Patron sits down and eats their food
9. The Patron is satisfied because the food was tasty.  They make an effort to clean up after themselves
10. The Patron leaves
11. A Worker cleans up after the Patron

#### Situation B

1. A patron is attracted by the cuisine offered by the restaurant
2. The patron enters the store and gets in line
3. The patron waits in line until their satisfaction is depleted
4. They leave in a huff!

#### Situation C

1. A patron is attracted by the cuisine offered by the restaurant
2. The patron enters the store and gets in line
3. The patron waits in line until they get to talk to the Worker at the register
4. The Worker receives an order for a hamburger and money.
5. The Worker has a great many other hamburger orders to fill.
6. The Patron's satisfaction decreases until they demand a refund and storm out.
7. The Worker's satisfaction decreases

#### Situation D

1. A Patron stops by the restaurant, but they don't see anything on the menu they want to eat.
2. The Patron keeps walking.

### Menu Design

The player character is able to design a menu to attrack Patrons.

- If the restaurnant can make burgers, it will attrack Patrons who want burgers.
- If the restaurant makes tacos, it will attrack Patrons who want tacos.

However, they must buy the equipment needed to make some kinds of items.

- To make stirfry, a wok is required.

### Entities

There are two types of entities in the game:

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
The player character employs some number of workers to help them handle Patron requests.

- The first worker (the restaurant owner) works for free
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
- Satisfaction is boosted whenever the worker gets a tips
- Satisfaction is decreased if a patron complains
- If satisfaction is depleted, they will stop working for the day
- If workers can't be paid, they won't come to work

##### Taking Orders

- A Worker talks to a Patron and takes their order at a register
- A Worker talks to a Patron and takes their order over the phone

##### Cooking

- A Worker gathers ingredients to prepare a meal at a workstation

##### Cleaning

- A Worker cleans the service area (floors, tables) to prepare it for the next Patron
- A Worker cleans dishes, cooking stations, appliances

### Capital Equipment Purchases

The player character is able to make capital equipment purchases to improve the efficency of workers. e.g:

- Installing a bigger stove allows Workers to cook more food at once
- Installing a dishwasher allows Workers to automate dishwashing
- Installing a self-order machine allows Patrons to place orders without talking to a Worker

### Progression

## Level 1

The player character starts with a beat-up food truck and a dream.

### Rules

- The food truck is paid off, so the player character doesn't have to pay rent every month
- The following actions are available to Workers:
  - Cook food
  - Take orders at register

## Level 2

The player character purchases a small lunch-counter with 5 seats.

### Rules

- The player character has to pay rent to the landlord every month.
- The following actions are available to Workers:
  - Cook food
  - Take orders at register
  - Clean
- Patrons are able to eat food at the restaurant, taking up a seat and creating a mess
- Patrons are able to order food "to-go"

## Level 3

The player character operates a small sit-down restasurant with 10 seats.

### Rules

- The player character has to pay rent to the landlord every month.
- The following actions are available to Workers:
  - Cook food
  - Take orders through a server
  - Clean
- Patrons are able to eat food at the restaurant, taking up the seat, using the bathroom, creating messes
- Patrons are able to order food "to-go"

### Events

Sometimes, good and bad things happen randomly.  This affects operations of the restaurant.

#### Good Events

- A Food Critic wrote nice things about the restaurant.  Surely more people will visit this week!

#### Bad Events

- A Food Critic wrote bad things about the restaurant! Fewer people will visit this week.
- A worker called in sick.  Fewer workers are available today

## Inspirations

- SimCity
- Overcooked
- Oxygen Not Included
- The (actual) Ten Top (Norfolk, VA)
