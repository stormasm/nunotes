
```rust

```
let v = [[[a]; [0], [1]], [[b]; [0] [1]]]
$v.0 | merge {$v.1}
```

[discord](https://discord.com/channels/601130461678272522/614593951969574961/973776154974629918)

def pwd-short [] {
  $env.PWD | str replace $nu.home-path '~' -s
}
```

```rust
date to-record
```

```rust
fetch https://api.github.com/repos/nushell/nushell | get stargazers_count
```

### Helper function to identify SQLite databases:

```rust
def is-sqlite-db [$path: path] {(open --raw $path | first 16) == ($"SQLite format 3(char -i 0)" | into binary)}
```

Example:

```rust
ls **/* | where (is-sqlite-db $it.name)
```

Hack to convert a table to a record

```rust
❯  let record = ([[key value]; [foo 10] [bar 20] [baz 30]] | each {|row| echo $"($row.key): ($row.value)"} | str collect , | $"{($in)}" | from nuon)
```                                                                                      
❯ $record.foo  
10               
❯ $record.bar  
20            
❯ $record.baz  
30  

[discord](https://discord.com/channels/601130461678272522/614593951969574961/964613306155413535)

```rust
git branch | lines | find new | str trim | each { |it| git branch -D $it }
```

How do you add a list to a table ?

```rust
ls | upsert md5 {|n| $n.name | hash md5}
```

[discord](https://discord.com/channels/601130461678272522/614593951969574961/961946292198715442)

```rust
help commands | select name usage | find --predicate { |it| ($it.usage | split chars | first) == ($it.usage | split chars | first | str downcase) }
```

[discord](https://discord.com/channels/601130461678272522/614593951969574961/960773100411179018)

#### Is there an easy way to get a column from a table by column index ?

```rust
def column [n] { transpose | select $n | transpose | select column1 | headers }
```

```rust
ls | column 1
```

[discord](https://discord.com/channels/601130461678272522/615253963645911060/959152636945387632)

### Understanding reduce

```rust
let rec = {foo: "bar"};
let cols = ("one two three" | split row " ");
$cols | reduce -f $rec {|it, acc| $acc | insert $it $it}
```

[discord](https://discord.com/channels/601130461678272522/614593951969574961/956339229946953738)

started working on a str repeat command until I realized you can just use lpad/rpad

```rust
'' | str lpad -l 3 -c 'ab'                             
ababab
```

[discord](https://discord.com/channels/601130461678272522/615962413203718156/954486380099154051)

```rust
def ! [b: expr] { if ($b) { $false } else { $true } }
```
> ! $true
false
> ! $false
true

[discord](https://discord.com/channels/601130461678272522/614593951969574961/948583172914962432)

```rust
seq date -b '2020-01-01' -e '2020-12-31'
```

cd to a path (file or directory) in the pipeline

```rust
def-env goto [] {
    let input = $in
    cd (
        if ($input | path type) == file {
            ($input | path dirname)
        } else {
            $input
        }
    )
}
```
example usage:   

```rust
$nu.config-path | goto
```

```bash
cd ~/j/tmp17/nuscripts/testdata;
open payload.json | get my_payload | to json | post https://jsonplaceholder.typicode.com/posts $in
```

where payload.json is...   
```rust
{
  "my_payload": {
    "title": "foo",
    "body": "bar",
    "userId": 1
  }
}
```

[discord](https://discord.com/channels/601130461678272522/614593951969574961/945600413028212756)

```rust
##You need to accumulate both the number and the array.
##So you need to modify #acc to store both values:

0..4 | reduce -f {n: 0, a: []} {|it acc| let res = $acc.n + $it; { n: $res ,  a: ($acc.a | append $res) } } | get a


#The acc in my example is a record { n: int, a: list }

###version 0.44

#!/usr/bin/nu

echo 0..4 |
reduce -f [[n a]; [0 []]] {|it acc|
                    let res = (($acc | get n) + $it)
                    [[n a]; [$res ($acc.a | append $res)]]
             } |
get a

───┬────────────────
 0 │ [table 0 rows]
 1 │              0
 2 │              1
 3 │              3
 4 │              6
 5 │             10
───┴────────────────
```

[discord core](https://discord.com/channels/601130461678272522/683070703716925568/945689592554733568)

```rust
##it's a little odd, you have to do it in two steps:

let block = {|x| $x + 10}
do $block 15
25
##we should probably add support for the literal form also
```

[discord](https://discord.com/channels/601130461678272522/683070703716925568/940721505250209842)

```rust
for x in 0..3 { ls } | flatten | table
```

```rust
which rg | get path | str collect
```

[discord](https://discord.com/channels/601130461678272522/683070703716925568/941826298802749480)

```rust
^ls -al | detect columns -s 2 | rename perms id user group size month day time file
```

```rust
alias lsg = (ls | grid -c)
alias nuvermd = (version | transpose key value | to md)
ls | drop column | to nuon | from nuon
```

[discord](https://discord.com/channels/601130461678272522/889232844101156914/939357502414409758)

```rust
$nu.cwd
```

[discord](https://discord.com/channels/601130461678272522/889232844101156914/938711460505325649)

is it possible to skip some kinds of comments line when open a csv file ?   
i got some bank's transaction file which include some lines begin with #

I'd do something like

```rust
open foo.csv --raw | lines | where ($it | str starts-with '#') == $false | from csv
```

### String stuff

This is a bit of a dumb question, but I bizarrely can't find anything about it in the docs... I'm trying to append a string to a variable, in bash I would do:

```rust
echo $variable/dir
```

But it doesn't seem to work. I also tried echo

```rust
$variable + /dir
```

but that also didn't work. How do I do this?

```rust
$"($variable)/dir"
```

or in this case

```rust
$variable | path join dir

or

[$variable, dir] | path join
```
