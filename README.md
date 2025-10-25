# Implementation of the get_score function with explanations

## The best solution

```rust
const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

// ...

fn recursive_get_score_part(game_stamps: &[Stamp], offset: i32) -> &[Stamp] {
    let pivot_rem = game_stamps.len() % 2;
    let pivot_idx = (game_stamps.len() - pivot_rem) / 2;

    if game_stamps.len() == 1 {
        return if game_stamps[0].offset == offset {
            game_stamps
        } else {
            &[]
        };
    }

    if game_stamps[pivot_idx].offset <= offset {
        recursive_get_score_part(&game_stamps[pivot_idx..], offset)
    } else {
        recursive_get_score_part(&game_stamps[..pivot_idx], offset)
    }
}

pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    let found = recursive_get_score_part(game_stamps, offset);

    let stamp = found.get(0).unwrap_or(&INITIAL_STAMP);

    (stamp.score.home, stamp.score.away)
}
```

### Explanation

This solution is the best because it has a good balance between execution time and ease of implementation.

### Ambiguities in the task

1. Due to the lack of clarity in error handling and task completion criteria, several variants of the get_score function
   had to be implemented - simple/efficient, with a panic/default value.
2. Another implementation option is also possible – based on the subject area. Example: if a player's leaderboard
   aggregate doesn't currently include an entry for the current timestamp, we use the aggregate for the previous period.

Declined to implement point 2, as the subject area is rather vague.

## Test

You can test locally with the command: `cargo test` in the root of the project

## Benchmarks

You can perform local benchmarking with the command: `cargo bench` in the project root.

The difference in execution time is an order of magnitude: 20-30 µs versus 7-10 ns