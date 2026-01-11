use heroes_of_camelot::data::{stub_party, stub_party_2};
use heroes_of_camelot::domain::*;
use heroes_of_camelot::domain::print_battle::print_battle;

fn main() {
    let attacker = stub_party();
    let defender = stub_party_2();

    let result = BattleResult::new(attacker, defender);

    print_battle(&result);
}