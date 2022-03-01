
[discord on exit codes](https://discord.com/channels/601130461678272522/601130461678272524/948277806733852682)

whats the nu equivalent to a && b? so run command a, and if it exits with code 0, run command b.

jt [they/them] — Today at 9:57 AM
a; b

LordMZTE — Today at 9:58 AM
doesnt that also run b if a fails?

jt [they/them] — Today at 9:58 AM
exit code 0 isn't really success, non-zero isn't really failure, sadly 😦
this is a "how posix works in the real world" kind of issue

LordMZTE — Today at 9:58 AM
i really need that for my use case 😛 guess ill just work with fish shell for now

jt [they/them] — Today at 9:59 AM
for nu a; b means "run a and then if it errors, don't run b. Otherwise, run b"
which isn't exit codes but nushell's internal error system, which actually does mean failure or success

LordMZTE — Today at 10:01 AM
oh ok! thats when i want then
hmm nah it just ran the second command although the first one failed :/

jt [they/them] — Today at 10:02 AM
is a an external or internal command?
LordMZTE — Today at 10:04 AM
external

jt [they/them] — Today at 10:04 AM
then you'll need a way to convert the exit code into an error
(or just check the exit code)
the reason we don't turn exit non-zero into errors is because it breaks too many external commands, as they use non-zero exit codes as information to the user, not as a sign of failure

in 0.44, we had an optional flag you could turn on if you wanted that behaviour, though I don't think it's been ported to 0.59
