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

### Menu Design

The player character is able to design a menu to attrack Patrons.

- If the restaurnant can make burgers, it will attrack Patrons who want burgers.
- If the restaurant makes tacos, it will attrack Patrons who want tacos.

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

## Inspirations

- SimCity
- Overcooked
- Oxygen Not Included
- The (actual) Ten Top (Norfolk, VA)
