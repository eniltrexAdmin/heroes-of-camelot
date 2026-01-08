Trying to make the battle logic, and trying to keep with event sourcing.


Thrre are several ways of doing it and its very annoying with rust + game.


I will opt for the simplest, which is now have domain events, whcih can be applied to the
whole Game State. 

Whatever is calculated should be present in teh domain event. 

The hesitation comes when applying the domain event, it feels that it shoudl be then where
things are calculated. But that shoudl retrigger new domain events, and being applied etc.

That will bring me to the domain event bus or something. And possibly each team subscribed etc.


(Also another thing that was complicated with Rust was to use the Battle Team itself, I resolved
using references -> the shiaiPosition, as it's very hard to have mut references of the same struct.

But now that I am leaniing more for the functional approach and generating the objects on each
modification, I can try to use the objects again, and it might help with some logic)


Anyway, I will leave the domaine vents subscribe events etc for later when the game evolves and
it becomes complicated. Now I will start with something simple, still Event Sourcing, applying events
but naive where the event has all teh info, and applying the event is deterministic, as it shoudl be
with the exact same result each time.

## For now

Events are applied at state level. (Consider state as the aggregate root and team the entities in it)


## When we need more logic

Events will be applied at team level, probably produce more event, and have buses to keep applying
until everything is resolved.

(when we have skills like re-reflect damage to a third team, and complicated things like that.)

or a loop to relistne to event.

also many domain events will get "richer" if we have reflect damage and so on, all of that is
"atomic" in one operation all of that happened, so all of that info should be in the 
same domain event. (I can't have several as there's only one aggregate root here -> "game/partida")