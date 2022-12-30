
* fdncred
I'm wondering if it would be worth it to port this from old-nushell to current-nushell and then update a bunch of code to use it? https://github.com/lily-mara/serde-nu

* Reilly — 12/23/2022
What are you thinking of using that for?
(seems cool, just wondering how we'd use it)

* fdncred — 12/23/2022
I'm just thinking of the dozens of places where we go from primitive types to nushell values and vice-versa. Just wondering if there's a better way than duplicating code all over the place to go to and from nushell values. 


[discord](https://discord.com/channels/601130461678272522/615329862395101194/1056002041794801674)

* fdncred — 12/30/2022 (design-discussion)
maybe to nuon should work like to json and just put null in when it doesn't know what to do in situations like $env.config | to nuon, instead of failing with error messages?

[discord](https://discord.com/channels/601130461678272522/615329862395101194/1058398141952565310)