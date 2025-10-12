use rand::Rng;
use crate::domain::*;

pub fn choose_skill(team: Team) -> CardSkill {
    team.captain().active_skill().clone()
}

fn skill_procs(card: &Card) -> bool {
    let effective_chance = trigger_effective_chance(card);
    // there could be extra modifiers here on "effective_chance".
    let mut rng = rand::rng();
    does_it_proc(effective_chance, &mut rng)
}

fn does_it_proc(proc_rate: ProcRate, rng: &mut impl Rng) -> bool {
    rng.random_range(0..100) < proc_rate
}



#[cfg(test)]
mod tests {
    use crate::domain::stubs::stub_build_skill;
    use super::*;

    #[test]
    fn test_skill_procs() {

            // this should test the "other modifiers".
    }
}