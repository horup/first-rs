
Maybe i need an intro and a menu?

I need a way to despawn all entities based on some premise.

Had strange issues with vscode. Used alot of memory after building.
Seems to be related to extensions being installed.
Removed all extensions and reinstalled rust analyzer, issue seems to be gone.

William said that mouse input is too fast and needs to be slowed down.
there should be a way to set this using a UI or atleast a json file or similar.

Using instant::now to keep a start variable works in the engine, but if the PC sleeps and is resumed, the tick will jump very much forward in time. Maybe this is OK behavior or maybe the engine should use dt to inck?

Loading needs to have some kind of progress bar.
loading in rust should be easy, but what about when loading the wasm module? how to handle that?

map should have an entity as a keyvalue pair allowing much more flexibility.
editor knows some keys, e.g. facing and texture.

I would love to have the ability to add singletons without having to enforce Serialize, since some singletons should not be 
serialized, e.g. campaign.
maybe simply implement serialize and deserialize and make them empty / default?

The final screen needs to be updated with a score showing how long time it took to complete the game
and how many pokemon cards were collected vs avaliable.
maybe not super imoportant.

Viktor says there should be a timer which determins when piggy arrives.

Playing a different tune when Piggy sees the player would make the experience better.
When piggy loses sight of the player, or is not following the player, the music resets to a calm tune.