## poters
[![Build Status](https://travis-ci.org/mthh/magrit.svg?branch=master)](https://travis-ci.org/mthh/magrit)

Rust library and CLI utility to compute **neighbourhood population potential** with scale control.  
Expect an input of observation points caracterised by a latitude, a longitude and a stock value (GeoJSON of points, csv, ...).

#### CLI usage reference:
```
USAGE:
    poters --function <FUNCTION> --input <FILE> --output <FILE> --range <RANGE> --scale <SCALE> --window=<SCALE> --field_name=<FIELD>

OPTIONS:
    -f, --function <FUNCTION>
            Name of the smoothing function, chosen among:
            disk, amortized_disk, gaussian, pareto, exponential.
    -i, --input <FILE>
            Input file to use (.csv, .json or .geojson).
    -o, --output <FILE>
            Path for output file (json or geojson format).
    -r, --range <RANGE>
            Smoothing range in kilometers, used as parameter of the interaction
            function.
    -s, --scale <SCALE>
            Resolution of the output in kilometers as ResoLat-ResoLon.
    -w, --window=<SCALE>
            Coordinate of the visualisation window, given in degrees as:
            minimum latitude, minimum longitude, maximum latitude, maximum longitude.
    -c, --field_name <FIELD>
            (Required for GeoJSON input) Field name containing the stock values to use.

```

#### Example usage (from a point GeoJSON layer):
```
poters --input tests/input_ra.geojson \
        --field_name="value"
        --output output_points.geojson \
        --range 10 \
        --scale 160-80 \
        --window=1,4,32,35
        --function gaussian
```

Depending on the extension provided, the result will be written as *json* or *geojson* format.  


#### Expected input for CLI tool:
- using JSON format:  
*input.json*
```
{"values": [
  {"lat":0, "lon": 0, "value": 7},
  {"lat":0, "lon": 5, "value": 8},
  {"lat":5, "lon": 5, "value": 9},
  {"lat":5, "lon": 0, "value": 10}
]}
```

- using CSV format (as latitude, longitude, stock; without header):  
*input.csv*
```
0.0, 0.0, 7.0
0.0, 5.0, 8.0
5.0, 5.0, 9.0
5.0, 0.0, 10.0
```

- using GeoJSON format (see [examples](https://github.com/mthh/poters/tree/master/examples) and [tests](https://github.com/mthh/poters/tree/master/tests) folders).  


#### Installation:
With a recent version of cargo :
```
cargo install --git https://github.com/mthh/poters
```

#### Using as a library:
Inlude the following line in your `Cargo.toml` file :
```
poters =  { git = "https://github.com/mthh/poters" }
```


Translation of [hyantes](http://hyantes.gforge.inria.fr/)/hyantesite library, originally authored by *Sebastien Martinez* and *Serge Guelton*, under CeCILL-C License.
