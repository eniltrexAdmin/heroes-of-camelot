use heroes_of_camelot::data::{stub_party, stub_party_2};
use heroes_of_camelot::domain::{print_battle_turn, BattleResult};

fn main() {
    let attacker = stub_party();
    let defender = stub_party_2();

    let result = BattleResult::new(attacker, defender);

    for (i, turn) in result.turn_logs.iter().enumerate() {
        print_battle_turn(turn, i + 1);
    }
}