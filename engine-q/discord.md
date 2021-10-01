
##### Interesting comments on discord about how engine-q works

##### JT goes through the whole logic of eq parsing

* [About how the parser works](https://discord.com/channels/601130461678272522/889232844101156914/893316285037936730)

lite_parse takes tokens and groups them for you, but doesn't do a full parse

lite_parse and parse both run at keystroke and submit

To do syntax highlighting we do a full parse that includes classifying what everything is so we know the correct style for each span
