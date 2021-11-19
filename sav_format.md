# File format for version 11 of the continue.sav file

## Header - (32 bytes)
```
int     : version number
4 bytes : unkown data block 1
int     : AE content enabled
int     : difficulty
int     : ships defeated
int     : jumps in sector
int     : scrap collected
int     : crew recruited
```
## Ship Designations
```
string : ship name
string : ship ID
int     : sector
4 bytes : unkown data block 2
```
## Score categories
There will be an arbetrary number of score categories based on the value in `number of categories`
```
int : number of categories (n)

--- This section will repeat n number of times
string : category name
int    : category value
```
## Ship Details
```
string  : ship name
string  : ship ID
string  : ship graphics base name
```

## Starting Crew Overview
```
int : number of crew(n)
--- This section will repeat n number of times
string : race
string : name
```

## Opponent info
Unsure of whether this information refers to the player ship or the opposing ship.
```
bool : hostile
int  : jump Charge
bool : is jumping
int  : jump animation ticks
```

## Ship Resources
```
int : Hull
int : Fuel
int : Drones
int : Missiles
int : Scrap
```

## Crew details
```
int : Crew count (n)
--- this section will repeat n number of times 
string : Name
string : Race
bool   : Drone
int    : HP
int    : x cord
int    : y cord
int    : Room number
int    : Room tile
bool   : Player Controlled
int    : Clone Ready
int    : Death order
int    : Tint count
[int]  : Sprite tint indeces
bool   : Mind Controlled
int    : Saved Room Square
int    : Saved Room ID
int    : Pilot skill
int    : Engine skill
int    : Shield skill
int    : Weapon skill
int    : Repair skill
int    : Combat skill
bool   : Male
int    : Repairs
int    : Kills
int    : Evasions
int    : Jumps survived
int    : Skill masteries earned
int    : Stun ticks
int    : Health boost
int    : Clonebay Priority
int    : Damage boost
4 bytes: Unkown data
int    : Universal death count
bool   : Pilot mastery one
bool   : Pilot mastery two
bool   : Engine mastery one
bool   : Engine mastery two
bool   : Shield mastery one
bool   : Shield mastery two
bool   : Weapon mastery one
bool   : Weapon mastery two
bool   : Repair mastery one
bool   : Repair mastery two
bool   : Combat mastery one
bool   : Combat mastery two
4 bytes: Unkown data
--- start animation state
bool   : Playing
bool   : Looping
int    : Current frame
int    : Progress ticks
int    : Scale
int    : X
int    : Y
--- end animation state
4 bytes: Unkown data
--- the next values will only be present if the crew members race string is "crystal"
int    : Lockdown recharge ticks
int    : Lockdown recharge goal
4 bytes: Unkown data

```