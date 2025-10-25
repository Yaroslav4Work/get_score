use rand::Rng;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub home: i32,
    pub away: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Stamp {
    pub offset: i32,
    pub score: Score,
}

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::rng().random_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::rng().random_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::rng().random_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

pub fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

// Easy way
pub mod lite {
    pub mod with_default {
        use crate::{INITIAL_STAMP, Stamp};

        pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
            let stamp = game_stamps
                .iter()
                .find(|stamp| stamp.offset == offset)
                // If expected value was not found by offset - using default (initial)
                .unwrap_or(&INITIAL_STAMP);

            (stamp.score.home, stamp.score.away)
        }
    }

    pub mod with_panic {
        use crate::Stamp;

        pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
            let panic_msg = &format!("Score was not found by offset {}", offset);

            let stamp = game_stamps
                .iter()
                .find(|stamp| stamp.offset == offset)
                // If expected value was not found by offset - using default (initial)
                .expect(panic_msg);

            (stamp.score.home, stamp.score.away)
        }
    }
}

pub mod best {
    use crate::Stamp;

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
    pub mod with_default {
        use crate::best::recursive_get_score_part;
        use crate::{INITIAL_STAMP, Stamp};

        pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
            let found = recursive_get_score_part(game_stamps, offset);

            let stamp = found.get(0).unwrap_or(&INITIAL_STAMP);

            (stamp.score.home, stamp.score.away)
        }
    }

    pub mod with_panic {
        use crate::Stamp;
        use crate::best::recursive_get_score_part;

        pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
            if game_stamps.is_empty() {
                panic!("Game stamps is empty");
            }

            let found = recursive_get_score_part(game_stamps, offset);

            let stamp = found.get(0).expect("Stamp was not found");

            (stamp.score.home, stamp.score.away)
        }
    }
}

#[cfg(test)]
mod tests {
    mod lite {
        mod with_default {
            use crate::generate_game;
            use rand::Rng;

            #[test]
            fn can_find_score_by_offset() {
                let game = generate_game();

                // We select, except for the first one, since it is the initiator and has zero values.
                let score_idx = rand::rng().random_range(1..game.len());
                let expected_stamp = game[score_idx];

                let found_score =
                    crate::lite::with_default::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn find_first_score_by_offset() {
                let game = generate_game();

                let score_idx = 0;
                let expected_stamp = game[score_idx];

                let found_score =
                    crate::lite::with_default::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn default_score_by_offset() {
                let game = generate_game();

                let latest_stamp = game.last().unwrap();

                let found_score =
                    crate::lite::with_default::get_score(&game, latest_stamp.offset + 1);
                let expected_score = (0, 0);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }
        }

        mod with_panic {
            use crate::{Stamp, generate_game};
            use rand::Rng;
            use std::panic::catch_unwind;

            #[test]
            fn can_find_score_by_offset() {
                let game = generate_game();

                // We select, except for the first one, since it is the initiator and has zero values.
                let score_idx = rand::rng().random_range(1..game.len());
                let expected_stamp = game[score_idx];

                let found_score = crate::lite::with_panic::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn find_first_score_by_offset() {
                let game = generate_game();

                let score_idx = 0;
                let expected_stamp = game[score_idx];

                let found_score = crate::lite::with_panic::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn panic_by_empty() {
                let game = Vec::<Stamp>::new();

                let found_score_res = catch_unwind(|| crate::lite::with_panic::get_score(&game, 1));

                assert!(
                    found_score_res.is_err(),
                    "Should panic because was not found"
                );
            }

            #[test]
            fn panic_by_not_found() {
                let game = generate_game();

                let latest_stamp = game.last().unwrap();

                let found_score_res = catch_unwind(|| {
                    crate::lite::with_panic::get_score(&game, latest_stamp.offset + 1)
                });

                assert!(
                    found_score_res.is_err(),
                    "Should panic because was not found"
                );
            }
        }
    }

    mod best {
        mod with_default {
            use crate::generate_game;
            use rand::Rng;

            #[test]
            fn can_find_score_by_offset() {
                let game = generate_game();

                // We select, except for the first one, since it is the initiator and has zero values.
                let score_idx = rand::rng().random_range(1..game.len());
                let expected_stamp = game[score_idx];

                let found_score =
                    crate::best::with_default::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn find_first_score_by_offset() {
                let game = generate_game();

                let score_idx = 0;
                let expected_stamp = game[score_idx];

                let found_score =
                    crate::best::with_default::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn default_score_by_offset() {
                let game = generate_game();

                let latest_stamp = game.last().unwrap();

                let found_score =
                    crate::best::with_default::get_score(&game, latest_stamp.offset + 1);
                let expected_score = (0, 0);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }
        }

        mod with_panic {
            use crate::{Stamp, generate_game};
            use rand::Rng;
            use std::panic::catch_unwind;

            #[test]
            fn can_find_score_by_offset() {
                println!("Testing the search for a random element (Best with panic)");

                let game = generate_game();

                // We select, except for the first one, since it is the initiator and has zero values.
                let score_idx = rand::rng().random_range(1..game.len());
                let expected_stamp = game[score_idx];

                let found_score = crate::best::with_panic::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn find_first_score_by_offset() {
                let game = generate_game();

                // We select, except for the first one, since it is the initiator and has zero values.
                let score_idx = 0;
                let expected_stamp = game[score_idx];

                let found_score = crate::best::with_panic::get_score(&game, expected_stamp.offset);
                let expected_score = (expected_stamp.score.home, expected_stamp.score.away);

                assert_eq!(
                    found_score, expected_score,
                    "Score found: {:?}, should be equal to expected: {:?}",
                    found_score, expected_score
                );
            }

            #[test]
            fn panic_by_empty() {
                let game = Vec::<Stamp>::new();

                let found_score_res = catch_unwind(|| crate::best::with_panic::get_score(&game, 1));

                assert!(
                    found_score_res.is_err(),
                    "Should panic because was not found"
                );
            }

            #[test]
            fn panic_by_not_found() {
                let game = generate_game();

                // We select, except for the first one, since it is the initiator and has zero values.
                let latest_stamp = game.last().unwrap();

                let found_score_res = catch_unwind(|| {
                    crate::best::with_panic::get_score(&game, latest_stamp.offset + 1)
                });

                assert!(
                    found_score_res.is_err(),
                    "Should panic because was not found"
                );
            }
        }
    }
}
