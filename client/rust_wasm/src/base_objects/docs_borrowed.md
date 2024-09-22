# What are borrowed base objects

Base objects that are not owned by current resource,
for example remote players, vehicles

## Problem

```rust
let player: Option<altv::Player> = altv::Player::get_by_id(123);
// let's assume player with such id is valid on this tick
let player: altv::Player = player.unwrap();

altv::set_timeout(|| {
  let name: ??? = player.name();
}, Duration::from_secs(10));
```

What should `player.name()` return?

- `String`: Then what happens if player will be disconnected at that time? should it panic?
- `Result<String>`: Return `Result` from ANY method, if player disconnected - return `Err`

`Result<String>` would be similar to what JS module does (both v1 and v2) with "invalid base object" exceptions

```rust
const player = alt.Player.getByID(123);
alt.Utils.assert(player != null);

alt.setTimeout(() => {
  // string or invalid base object exception
  const name = player.name();
}, 10_000);
```

It turns out there is another way

```rust
altv::new_scope(|scope| {
  // `player` is owned by `scope` now
  let player: Option<&altv::Player> = altv::Player::get_by_id(scope, 123);
  // let's assume player with such id is valid on this tick
  let player: altv::Player = player.unwrap();

  altv::set_timeout(|| {
    // compile time error, because `player` is owned by `scope`
    // and scope is only valid in closure to which it is passed
    let name: String = player.name();
  }, Duration::from_secs(10));
});
```

When we need to use borrowed base object we attach it some scope (in other words one tick)

To use `player` again we need to _unscope_ it (at this point we can't call any method on it) and attach to new scope

```rust
new_scope(|scope| {
  // `player` is owned by `scope` now
  let player: Option<&altv::Player> = altv::Player::get_by_id(scope, 123);
  // let's assume player with such id is valid on this tick
  let player: altv::Player = player.unwrap();
  // "unscoping" it
  let unscoped_player = player.unscope();

  altv::set_timeout(|scope| {
    let Some(player) = unscoped_player.scope(scope) else {
      // player already disconnected
      return;
    };

    // all calls to `player` are infallible here ✨
    let name: String = player.name();
  }, Duration::from_secs(10));
});
```

## Scopes

"Scopes" are needed so that borrowed base objects cannot be used
after destroy (invalid base objects)<br>
One scope is equal to one tick (both serverside and clientside)<br>
To use some borrowed base object (read something from it or call something on it)
it needs to be attached to some scope (in other words, scope becomes its temporary owner)

Scopes are passed into event callbacks, timer callbacks, everywhere

```rust
altv::events::on_game_entity_create(|scope| {
// ...
});

altv::events::on_net_owner_change(|scope| {
// ...
});

altv::every_tick(|scope| {
// ...
});
```

But futures (async/await) is special case

```rust
let future = async {
  // from where do we have to get scope?
  let player: Option<altv::Player> = altv::Player::get_by_id(
    /* scope: */ ???,
    /* id: */ 123
  );
  // let's assume player with such id is valid on this tick
  let player = player.unwrap();

  altv::wait(Duration::from_secs(1)).await;
  // ???
  dbg!(player.name());
};
```

In this case it's possible to create new scope anywhere using [`new_scope`](super::scope::new_scope)

```rust
let future = async {
  let player_name = altv::new_scope(|scope| {
    let player = altv::Player::get_by_id(scope, 123);
    // let's assume player with such id is valid on this tick
    let player = player.unwrap();

    // no need to hold reference to player instance if we only need data from it
    player.name()
  });

  altv::wait(Duration::from_secs(1)).await;

  dbg!(player_name);
};
```

If you want to hold reference to base object for longer than one scope
(one tick) there are _unscoped_ base objects

```rust
let future = async {
  let unscoped_player = altv::new_scope(|scope| {
    let player = altv::Player::get_by_id(scope, 123);
    // let's assume player with such id is valid on this tick
    let player = player.unwrap();

    player.unscope()
  });

  altv::wait(Duration::from_secs(1)).await;

  altv::new_scope(|scope| {
    let Some(player) = unscoped_player.scope(scope) else {
      // player was destroyed
      return;
    };
    dbg!(player.name());
  });
};
```

See [`Scope`](super::scope::Scope#example) trait documentation for more examples of usage
