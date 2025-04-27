use heroes_of_camelot::data::*;
use heroes_of_camelot::domain::shiai::shiai;

fn main() {

    let attacker = stub_party();
    let defender = stub_party_2();

    shiai::battle(attacker, defender);

}
