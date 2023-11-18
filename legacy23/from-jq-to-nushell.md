---
type: note
id: from-jq-to-nushell
publication_date: 2023-11-16
author: arnau
tags:
  - nushell
  - jq
  - data
---
# From jq to Nushell

Where I translate [The Ultimate Interactive JQ Guide](https://ishan.page/blog/2023-11-06-jq-by-example/) to [Nushell](https://ww.nushell.sh/).


<!-- body -->

Both `jq` and `nu` have the ability to transform data in a composable way. Nushell however has a wider range data types and inputs to parse from.

All examples will stick to JSON to have a good mirroring experience but keep in mind that any data transformation you can see in here is applicable to JSON, YAML, CSV, SQLite and more.


## Basic Operations

### Selecting values

In `jq`, to get the value from an object we do:

```sh
echo '{"name": "Alice", "age": 30}' | jq -r '.name'
```

In `nu` we do:

```nu
'{"name": "Alice", "age": 30}' | from json | get name
```

### Filtering Arrays

In `jq`, to filter an array we do:

```sh
echo '[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]' |
jq -r '.[] | select(.age > 28)'
```

In `nu` we do:

```nu
'[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]'
| from json
| where age > 28
```

Notice that the output for `jq` is a JSON string whereas in `nu` it's a table value. To get the output of any pipeline in JSON simply apply a `to json` at the end:

```nu
'[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]'
| from json
| where age > 28
| to json
```

### Mapping Arrays

In `jq`, to map an array we do:

```sh
echo '[1, 2, 3, 4, 5]' | 
jq -r 'map(. * 2)'
```

In `nu` we do:

```nu
'[1, 2, 3, 4, 5]' 
| from json
| each { |x| $x * 2 }
```

Note that you can rely on the auto-binding `$in` to type a slightly more compact block:

```nu
'[1, 2, 3, 4, 5]' 
| from json
| each { $in * 2 }
```

### Combining Filters

In `jq`, to combine filters we do:

```sh
echo '[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]' | 
jq -r '.[] | select(.age > 28) | .name'
```

In `nu` we do:

```nu
'[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]' 
| from json
| where age > 28
| get name
```

### Splitting Strings

In `jq`, to split a string we do:

```sh
echo '{"name": "Alice Smith"}' |
jq -r '.name | split(" ") | .[0]'
```

In `nu` we do:

```nu
'{"name": "Alice Smith"}'
| from json
| get name
| split words
| get 0
```

### Conditional logic

In `jq` to work with `if` expressions we do:

```sh
echo '{"name": "Alice", "age": 30}' |
jq -r 'if .age > 18 then "Adult" else "Child" end'
```

In `nu` we do:

```nu
'{"name": "Alice", "age": 30}'
| from json
| if $in.age > 18 { "Adult" } else { "Child" }
```

### Handling Null Values

In `jq`, to filter out `null` values we do:

```sh
echo '[1, null, 3, null, 5]' |
jq -r 'map(select(. != null))'
```

In `nu` we do:

```nu
'[1, null, 3, null, 5]'
| from json
| where { $in != null }
```

### Formating Output

In `jq`, to output a formatted string we do:

```sh
echo '{"name": "Alice", "age": 30}' |
jq -r "Name: \(.name), Age: \(.age)"
```

In `nu` we do:

```nu
'{"name": "Alice", "age": 30}'
| from json
| format "Name: {name}, Age: {age}"
```

### Multiple Outputs

In `jq`, to compose a new object we do:

```sh
echo '{"name": "Alice", "age": 30}' | 
jq -r '{name: .name, age: (.age + 5)}'
```

In `nu` we do:

```nu
'{"name": "Alice", "age": 30}'
| from json
| {name: $in.name, age: ($in.age + 5)}
```

## Dealing with Nested Items

### Recursive Descent

In `jq`, to recursively filter a tree structure we do:

```sh
echo '{"data": {"value": 42, "nested": {"value": 24}}}' | 
jq -r '.. | .value?'
```

In `nu`, there is no built-in function to achieve this, however, you can always define your own reusable commands. [See the Appendix: toolbox.nu](#appendix-toolbox-nu) for an implementation of the command `cherry-pick` shown in the example below.

```nu
'{"data": {"value": 42, "nested": {"value": 24}}}' 
| from json
| cherry-pick {|x| $x.value?}
```


### Filtering Nested Arrays

In `jq`, to filter nested arrays we do:

```sh
echo '{"data": [{"values": [1, 2, 3]}, {"values": [4, 5, 6]}]}' |
jq -r '.data[].values[] | select(. > 3)'
```

In `nu` we do:

```nu
'{"data": [{"values": [1, 2, 3]}, {"values": [4, 5, 6]}]}'
| from json
| get data.values
| flatten
| where {|x| $x > 3}
```

### Flattening Nested JSON Objects

In `jq`, to get all the key-value pairs we do:

```sh
echo '{"person": {"name": {"first": "Alice", "last": "Smith"}, "age": 30}}' | 
jq -r 'paths as $p | select(getpath($p) | type != "object") | ($p | join(".")) + " = " + (getpath($p) | tostring)'
```

In `nu`, there is also no built-in function to achieve this. [See the Appendix: toolbox.nu](#appendix-toolbox-nu) for an implementation of the command `flatten record-paths` shown in the example below.

```nu
'{"person": {"name": {"first": "Alice", "last": "Smith"}, "age": 30}}'
| from json
| flatten record-paths
```

### Recursive Object Manipulation

In `jq`, to traverse a tree we can do:

```sh
echo '{"data": {"value": 42, "nested": {"value": 24}}}' | 
jq -r 'recurse | .value? | select(. != null) | { value: (. * 5) } | add'
```

In `nu`, there is no built-in function equivalent to `recurse`. However, we can reuse the solution from [Recursive Descent](#recursive-descent) to extract the values to manipulate:

```nu
'{"data": {"value": 42, "nested": {"value": 24}}}' 
| from json
| cherry-pick {|x| $x.value?}
| compact # or use the familiar `where {|x| $x != null}`
| each { |x| $x * 5 }
```


### Complex Object Transformation

In `jq`, to map over object values we do:

```sh
echo '{"items": [{"name": "Apple", "price": 1}, {"name": "Banana", "price": 0.5}]}' | 
jq -r '.items | map({(.name): (.price * 2)}) | add'
```

In `nu` we do:

```nu
'{"items": [{"name": "Apple", "price": 1}, {"name": "Banana", "price": 0.5}]}' 
| from json
| get items
| update price {|row| $row.price * 2}
```

Note that in this case nu does not require creating new records because we can leverage the fact that a list of records is a table. However, in other situations it might be required as we have seen in [Multiple Outputs](#multiple-outputs).


### Walk through object and apply a transformation conditionally

In `jq`, to traverse and transform an object we do:

```sh
echo '{"data": {"values": [1, 2, 3], "nested": {"values": [4, 5, 6]}}}' | 
jq -r 'walk(if type == "number" then . * 2 else . end)'
```

In `nu`, there is no built-in function to achieve this. [See the Appendix: toolbox.nu](#appendix-toolbox-nu) for an implementation of the command `walk` shown in the example below.

```nu
'{"data": {"values": [1, 2, 3], "nested": {"values": [4, 5, 6]}}}' 
| from json
| walk {|value| if ($value | describe) == "int" { $value * 2 } else { $value }}
```

## Statistical Operations

### Sorting Arrays

In `jq`, to sort an array we do:

```sh
echo '[3, 1, 4, 2, 5]' | 
jq -r 'sort'
```

In `nu` we do:

```nu
'[3, 1, 4, 2, 5]'
| from json
| sort 
```

### Extracting Unique Values from an Array

In `jq`, to filter an array keeping just unique values we do:

```sh
echo '[1, 2, 2, 3, 4, 4, 5]' | 
jq -r 'unique'
```

In `nu` we do:

```nu
'[1, 2, 2, 3, 4, 4, 5]' 
| from json
| uniq
```

### Calculating Averages

In `jq`, to calculate an average we do:

```sh
echo '[{"score": 90}, {"score": 85}, {"score": 95}]' | 
jq -r 'map(.score) | add / length'
```

In `nu` we do:

```nu
'[{"score": 90}, {"score": 85}, {"score": 95}]' 
| from json
| get score
| math avg
```

### Grouping and Aggregating

In `jq`, to group an array of objects by key we do:

```sh
echo '[{"category": "A", "value": 10}, {"category": "B", "value": 20}, {"category": "A", "value": 5}]' | 
jq -r 'group_by(.category) | map({category: .[0].category, sum: map(.value) | add})'
```

In `nu` we do:

```nu
'[{"category": "A", "value": 10}, {"category": "B", "value": 20}, {"category": "A", "value": 5}]' 
| from json
| group-by --to-table category
| update items { |row| $row.items.value | math sum }
| rename category sum
```

### Filtering after Aggregation

In `jq`, to filter after aggregating we do:

```sh
echo '[{"category": "A", "value": 10}, {"category": "B", "value": 20}, {"category": "A", "value": 5}]' | 
jq -r 'group_by(.category) | map({category: .[0].category, sum: (map(.value) | add)}) | .[] | select(.sum > 17)'
```

In `nu` we do:

```nu
'[{"category": "A", "value": 10}, {"category": "B", "value": 20}, {"category": "A", "value": 5}]' 
| from json
| group-by --to-table category
| update items { |row| $row.items.value | math sum }
| rename category value
| where value > 17
```

### Custom Aggregation with reduce

In `jq`, to use a custom aggregation we do: 

```sh
echo '[{"value": 10}, {"value": 20}, {"value": 30}]' | 
jq -r 'reduce .[] as $item (0; . + $item.value)'
```

In `nu` we do:

```nu
'[{"value": 10}, {"value": 20}, {"value": 30}]' 
| from json
| reduce -f 0 { |item, acc| $acc + $item.value }
```

### Calculating Histogram Bins

In `jq`, to 

```sh
echo '[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]' | 
jq -r 'group_by(. / 5 | floor * 5) | map({ bin: .[0], count: length })'
```

In `nu` we do:

```nu
'[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]'
| from json
| group-by --to-table { $in // 5 * 5 }
| each { |row| {bin: $row.items.0, count: ($row.items | length)} }
```


## Closing thoughts

First, I want to thank the author of [The Ultimate Interactive JQ Guide](https://ishan.page/blog/2023-11-06-jq-by-example/) for putting that guide together. Translating these examples to Nushell has been quite fun. 

Over the years I've used `jq` sporadically, typically to quickly wrangle and inspect some unknown JSON. Due to that on/off interaction I have never managed to retain its syntax in my head for long so when using it for any non trivial operation I've had to invest a decent chunk of time to re-acquaint myself with it.

On the other hand, Nushell fits nicely with my mental model so once I've learnt a command I can reuse it with little effort.

Both jq and Nushell are excellent tools but I found Nushell to be a better investment.



## Appendix: toolbox.nu

The following is a module implementing and testing the custom commands used in previous sections. To use them, load them using:

```nu
use toolbox.nu *
```


```nu
# toolbox.nu
use std assert

# A command for cherry-picking values from a record key recursively
export def cherry-pick [
    test               # The test funciton to run over each element
    list: list = []    # The initial list for collecting cherry-picked values
] {
    let input = $in

    if ($input | describe) =~ "record|table" {
        $input
        | values
        | reduce --fold $list { |value, acc|
            $acc | append [($value | cherry-pick $test)]
          }
        | prepend [(do $test $input)]
        | flatten
    } else {
        $list
    }
}


#[test]
def test_deep_record_with_key [] {
    assert equal ({data: {value: 42, nested: {value: 442}}} | cherry-pick {|x| $x.value?}) [null 42 442]
    assert equal ({value: 42, nested: {value: 442, nested: {other: 4442}}} | cherry-pick {|x| $x.value?}) [42 442 null]
    assert equal ({
        value: 1,
        nested: {value: 2, nested: {terminal: 3}}
        terminal: 4,
        nested2: {value: 5}} | cherry-pick {|x| $x.value?}) [1 2 null 5]
}

#[test]
def test_record_without_key [] {
    assert equal ({data: 1} | cherry-pick {|x| $x.value?}) [null]
}

#[test]
def test_integer [] {
    assert equal (1 | cherry-pick {|x| $x.value?}) []
}

def test_string [] {
    assert equal ("foo" | cherry-pick {|x| $x.value?}) []
}

#[test]
def test_list [] {
    assert equal (["foo"] | cherry-pick {|x| $x.value?}) []
}

#[test]
def test_table [] {
    assert equal ([[a b]; [1.1 1.2] [2.1 2.2]] | cherry-pick {|x| $x.value?}) [null null]
    assert equal ([[a b]; [1.1 1.2] [2.1 2.2]] | cherry-pick {|x| $x.b?}) [1.2 2.2]
}

#[test]
def test_record_with_key [] {
    assert equal ({value: 42} | cherry-pick {|x| $x.value?}) [42]
    assert equal ({value: null} | cherry-pick {|x| $x.value?}) [null]
}

#[test]
def test_deep_record_without_key [] {
    assert equal ({data: {v: 42}} | cherry-pick {|x| $x.value?}) [null null]
}

# Like `describe` but dropping item types for collections.
export def describe-primitive []: any -> string {
  $in | describe | str replace --regex '<.*' ''
}


# A command for cherry-picking values from a record key recursively
export def "flatten record-paths" [
    --separator (-s): string = "."    # The separator to use when chaining paths 
] {
    let input = $in

    if ($input | describe) !~ "record" {
        error make {msg: "The record-paths command expects a record"}
    }

    $input | flatten-record-paths $separator
}
    
def flatten-record-paths [separator: string, ctx?: string] {
    let input = $in

    match ($input | describe-primitive) {
        "record" => {
            $input
            | items { |key, value|
                  let path = if $ctx == null { $key } else { [$ctx $key] | str join $separator } 
                  {path: $path, value: $value}
              }
            | reduce -f [] { |row, acc|
                  $acc
                  | append ($row.value | flatten-record-paths $separator $row.path)
                  | flatten
              }
        },
        "list" => {
            $input
            | enumerate
            | each { |e|
                  {path: ([$ctx $e.index] | str join $separator), value: $e.item}
              }
        },
        "table" | "block" | "closure" => { error make {msg: "Unexpected type"} },
        _ => {
            {path: $ctx, value: $input}
        },
    }
}

#[test]
def test_record_path [] {
    assert equal ({a: 1} | flatten record-paths) [{path: "a", value: 1}]
    assert equal ({a: 1, b: [2 3]} | flatten record-paths) [[path value]; [a 1] ["b.0" 2] ["b.1" 3]]
    assert equal ({a: 1, b: {c: 2}} | flatten record-paths) [[path value]; [a 1] ["b.c" 2]]
    assert equal ({a: {b: {c: null}}} | flatten record-paths -s "->") [[path value]; ["a->b->c" null]]
}



# A command for walking through a complex data structure and tranforming its values recursively
export def walk [mapping_fn: closure] {
    let input = $in

    match ($input | describe-primitive) {
        "record" => {
            $input
            | items { |key, value|
                  {key: $key, value: ($value | walk $mapping_fn)}
              }
            | transpose -rd
        },
        "list" => {
            $input
            | each { |value|
                  $value | walk $mapping_fn
              }
        },
        "table" | "block" | "closure" => { error make {msg: "unimplemented"} },
        _ => {
            do $mapping_fn $input
        },
    }
}

#[test]
def test_walk [] {
    assert equal ({a: 42} | walk {|x| if ($x | describe) == "int" { $x * 2 } else { $x }}) {a: 84}
    assert equal ({a: 1, b: 2, c: {d: 3}} | walk {|x| if ($x | describe) == "int" { $x * 2 } else { $x }}) {a: 2, b: 4, c: {d: 6}}
    assert equal ({a: 1, b: "2", c: {d: 3}} | walk {|x| if ($x | describe) == "int" { $x * 2 } else { $x }}) {a: 2, b: "2", c: {d: 6}}
}
```
