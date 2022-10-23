# homework1
_Homework for the 'Rust School 1.0' course._

## Description
The implementation of area and volume calculation for various of figures.

The `main` branch contains basic implementation for area calculation of 4 figures:
* circle
* quadrate
* rectangle
* triangle
The type of their dimensions sizes should be `f32`.

The `generic` branch contains area and volume calculation of 8 figures using generics:
* circle
* quadrate
* rectangle
* triangle
* sphere
* cube
* cylinder
* cuboid

## Usage
### To Run:

`cargo run <FIGURE> <PARAM1> [<PARAM2> ...]`

Where `<FIGURE>` - is a figure name mentioned before (begins with an upper case letter);

`<ARG1> [<ARG2> ...]` - dimensions of a figure (radius, edge, etc).

### List of figure types and corresponding arguments:

|Figure|Arg1|Arg2|Arg3|
|------|----|----|----|
|Circle|Radius|||
|Quadrate|Side|||
|Rectangle|Side1|Side2||
|Triangle|Side1|Side2|Side3|
|Sphere*|Radius|||
|Cude*|Edge|||
|Cylinder*|Radius|Height||
|Cuboid*|Edge1|Edge2|Edge3|

`*` on the `generic` branch only

### For example:

`cargo run Sphere 5`

### Obtained result:
> Figure: Sphere, Area: 314.1592653589793, Volume: 523.5987755982989
