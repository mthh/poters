## poters
[![Build Status](https://travis-ci.org/mthh/magrit.svg?branch=master)](https://travis-ci.org/mthh/magrit)
  
Rust library and CLI utility to compute **neighbourhood population potential** with scale control.
Expect an input of observation points caracterised by a latitude, a longitude and a stock value :

#### CLI usage reference:
```
USAGE:
    poters --function <FUNCTION> --input <FILE> --output <FILE> --range <RANGE> --scale <SCALE> --window=<SCALE>

OPTIONS:
    -f, --function <FUNCTION>
            Name of the smoothing function, chosen among:
            disk, amortized_disk, gaussian, pareto, exponential.
    -i, --input <FILE>
            Input file to use (.csv or .json).
    -o, --output <FILE>
            Path for output file (json format).
    -r, --range <RANGE>
            Smoothing range in kilometers, used as parameter of the interaction
            function.
    -s, --scale <SCALE>
            Resolution of the output in kilometers as ResoLat-ResoLon.
    -w, --window=<SCALE>
            Coordinate of the visualisation window, given in degrees as:
            minimum latitude,minimum longitude,maximum latitude,maximum longitude
```

#### Example usage:
```
poters --input tests/data_1.in \
        --output output.json \
        --range 10 \
        --scale 80-80 \
        --window=-1,6,-1,6 \
        --function amortized_disk
```

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
