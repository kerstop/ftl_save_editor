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
string  : ship name
string  : ship ID
int     : sector
4 bytes : unkown data block 2
```
## Score categories
There will be an arbetrary number of score categories based on the value in `number of categories`
```
int     : number of categories (n)

--- This section will repeat n number of times
string  : category name
int     : category value
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
string  : race
string  : name
```

## Opponent info
Unsure of whether this information refers to the player ship or the opposing ship.
```
bool    : hostile
int     : jump Charge
bool    : is jumping
int     : jump animation ticks
```

## Ship Resources
```
int : Hull
int : Fuel
int : Drones
int : Missiles
int : Scrap
```
