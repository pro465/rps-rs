# rps-rs
[![Rust](https://github.com/pro465/rps-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/pro465/rps-rs/actions/workflows/rust.yml)

example that uses [nn-rs](https://github.com/pro465/nn-rs) to try to beat the player

# How does this work?
this basically contains two parts:
   * an nn-based behaviour-learning system
   * an system that uses the first system to choose its moves

the first system learns to behave like the player.
and the second system will choose a winning move agaiinst the first system's next move.
since the first system models the player's behaviour, the second system will be more likely to also win against the player than to lose or draw.
