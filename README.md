# sprouts-stranded
A simulation of agents doing what they do best in the environment they found themselves in.


# Premise
A group of plant folks are deposited in a procedurally generated world.

The environment lives on the server, and clients request the area they
want to survey.

Ideally, nothing would be rushed, a folk could live for weeks, and clients
could check in every so often on their favorites.

# Folks (Pending name)
Folks' main goal is to survive on a personal scale. This would include a
simple loop of gathering food whenever they are low on energy.

I've planned for more, but for the motivation of the project I've decided
to keep the scope rather small.

# Motivation
I want to learn Rust, with features such as a server-clients model,
concurrency and such. So I want to keep a small scope of achievable 
goals. Once this goal is done, I would search best practices and ways
to refactor the codebase in a Rusty way to further improve my
understanding of the project.

Ideally only the server would be in Rust, the client would be done in
C++ for instance, with the premise of not getting any update while the
server would.

# Environment
The environment will be generated using simplex noise, meaning, it will
be a 2D plane with elevation informations.

A ressouce manager will be in charge of the plants and different food
sources in the world, it'll keep track of the whole potential food stock
and the growth + spread of each element.

# Energy managment and sensors
Sensors are tools that can be used by different agents
For instance in the folks, we would find two type of sensors

## Point sensor
It has a position, a direction, a distance, and what to test for.
It returns a value in the range 0.0 - 1.0

The folks will use it to locate where the elevation is ideal to traverse.

## Cone sensor
It has a position, a direction, a distance, an angle range (start - end), 
and what to look for.
It returns a list of points (vectors).

The folks will use it to locate different food sources (it'll be used to
change the state of the state machine for instance).

# Folk state machine

## Wander
Life is good, energy levels too, they wander the land, ideally alongside
friends.

## Gather
There is a need in energy, they deploy a strategy to search for food.

## Fetch
During the gathering stage, a food source has been found, the agent
goes toward it to cousume the food using the presarvation policy.

# What do folks implement ?
An agent (position, velocity, acc, friction, etc)
A target (Updated by the state machine)
An identity (name, family name, group/faction)
A physical state (age, energy, (base speed: from age & energy))
A personality (Prefered elevation, tendency to gather & wander)
Sensors (How efficient, different configurations)

