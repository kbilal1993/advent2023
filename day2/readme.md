# Day 2:

# Comments
First time using unit tests while doing Advent of Code work. Should have done this earlier. I like how easy Rust makes this.

Rust is a bit weird with iterations. I understand why I need such big chains of .collect::<Vec<_>>() and .parse().unwrap() but it seems a bit overkill. This specific line gave me issues:

`temp_handful_strings = temp_split[1].split(";").collect::<Vec<_>>().iter().map(|x| x.trim()).collect();`

It's supposed to take the list of handfuls pulled as part of the game, and turn them into individual strings. Maybe there's an easier way of doing that.

My hands are freezing. Hard to type with cold hands.
