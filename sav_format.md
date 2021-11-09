# File format for version 11 of the continue.sav file

## Header - (32 bytes)
```
4 bytes : version number
4 bytes : unkown data block 1
4 bytes : AE content enabled
4 bytes : difficulty
4 bytes : ships defeated
4 bytes : jumps in sector
4 bytes : scrap collected
4 bytes : crew recruited
```
## Ship Designations
```
string  : ship name
string  : ship ID
4 bytes : sector
4 bytes : unkown data block 2
```
## Score categories
There will be an arbetrary number of score categories based on the value in `number of categories`
```
4 bytes : number of categories (n)

--- This section will repeat n number of times
string  : category name
4 bytes : category value
```
## Ship Details
```
string  : ship name
string  : ship ID
string  : ship graphics base name
```

## Crew Overview
```
4 bytes : number of crew(n)
--- This section will repeat n number of times
string  : race
string  : name
```

## Ship Resources
```
4 bytes : Hull
4 bytes : Fuel
4 bytes : Drones
4 bytes : Missiles
4 bytes : Scrap
```
