Using this tool requires an account on WeatherAPI.com (basic usage should be free).
Specifically you need:
- a key for either real-time weather or forecast data (I forget which, and I can't access my account right now because life happened),
- a file containing said key named 'WeatherAPI.key' in same directory as the built binary when running it directly,
- and a file named 'zipcode.txt' containing your zip code in the same directory

Additionally, the key and zip code can be overwritten with the -k and -z options, respectively

The easiest way to install it is to:
run `git clone https://www.github.com/ionarevamp/clitemp && cd clitemp` (Requires git)
run `cargo install --path .` (Requires a rust install) , and
copy/move 'WeatherAPI.key' and 'zipcode.txt' to $CARGO_HOME/bin (~/.cargo/bin by default).

The name is a work-in-progress, and this is just a tool I made to quickly check the weather via command line.
