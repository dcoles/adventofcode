use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn main() {
    let mut world = World::from_file("input.txt");

    loop {
        let n_immune = world.everyone_alive().iter().filter(|g| g.team == Team::ImmuneSystem).count();
        let n_infection = world.everyone_alive().iter().filter(|g| g.team == Team::ImmuneSystem).count();
        if n_immune == 0 || n_infection == 0 {
            break;
        }
        world.fight();
    }
    world.print_summary();
}

fn parse_group(team: Team, id: u32, group: &str) -> Group {
    let re = Regex::new(r"^(?P<n>\d+) units each with (?P<hp>\d+) hit points (?:\((?P<w1>\S+) to (?P<t1>[^);]+)(?:; (?P<w2>\S+) to (?P<t2>[^)]+))?\) )?with an attack that does (?P<ap>\d+) (?P<type>\S+) damage at initiative (?P<init>\d+)$").unwrap();
    let caps = re.captures(group).unwrap();

    let n: i32 = caps["n"].parse().unwrap();
    let hp: i32 = caps["hp"].parse().unwrap();
    let ap: i32 = caps["ap"].parse().unwrap();
    let initiative: i32 = caps["init"].parse().unwrap();
    let attack_type = Type::from_str(&caps["type"]).unwrap();

    let mut weakness= HashSet::new();
    let mut immunity = HashSet::new();
    for &(wn, tn) in &[("w1", "t1"), ("w2", "t2")] {
        if let Some(w) = caps.name(wn) {
            let types = caps[tn].split(", ").map(|s| Type::from_str(s).unwrap());
            match w.as_str() {
                "weak" => weakness.extend(types),
                "immune" => immunity.extend(types),
                adj => panic!("Unknown adjective: {}", adj)
            }
        }
    }

    Group { team, id, n, hp, ap, initiative, attack_type, weakness, immunity }
}

#[derive(Debug)]
struct World {
    groups: HashMap<(Team, u32), Group>,
}

impl World {
    fn from_file(filename: &str) -> World {
        let input = fs::read_to_string(filename)
            .expect("Failed to read input");

        let mut groups = HashMap::new();
        let mut id_for_teams = HashMap::new();
        for team in input.split("\n\n") {
            let mut name = "";
            for line in team.lines() {
                if name.is_empty() {
                    name = line.trim_end_matches(":");
                    continue
                }

                let team = Team::from_str(name);
                let id = id_for_teams.entry(team).or_insert(1);

                groups.insert((team, *id), parse_group(team, *id, line));
                *id += 1;
            }

        }

        World { groups }
    }

    fn everyone_alive(&self) -> Vec<&Group> {
        self.groups.values().filter(|g| g.is_alive()).collect()
    }

    fn print_summary(&self) {
        for &team in &[Team::ImmuneSystem, Team::Infection] {
            println!("{:?}:", team);
            let mut total = 0;
            for group in self.everyone_alive().iter().filter(|g| g.team == team) {
                println!("- Group {} contains {} units", group.id, group.n);
                total += group.n;
            }
            println!("(Total: {})", total);
        }
        println!();
    }

    fn fight(&mut self) {
        // Target selection
        let mut everyone = self.everyone_alive();
        let mut all_targets: HashSet<_> = everyone.iter().map(|g| (g.team, g.id)).collect();

        everyone.sort_by_key(|g| (-g.effective_power(), -g.initiative));

        self.print_summary();

        let mut selected_target = HashMap::new();
        for group in &everyone {
            let mut targets: Vec<_> = all_targets.iter().filter(|(t, _)| group.team != *t).collect();
            targets.sort_by_key(|team_id| { let g= self.groups.get(team_id).unwrap(); (-group.damage(&g), -g.effective_power(), -g.initiative)});

            if let Some(&&target_id) = targets.first() {
                let target_group = self.groups.get(&target_id).unwrap();
                let damage = group.damage(&target_group);
                println!("Group {:?} would deal {:?} {} damage", (group.team, group.id), target_id, damage);
                selected_target.insert((group.team, group.id), target_id);
                all_targets.remove(&target_id);
            }
        }

        // Attacking
        everyone.sort_by_key(|g| (-g.initiative));
        let group_team_ids: Vec<_> = everyone.iter().map(|g| (g.team, g.id)).collect();

        for &group_team_id in &group_team_ids {
            let group = self.groups.get(&group_team_id).unwrap();
            if group.n < 1 {
                continue;
            }

            if let Some(&target_id) = selected_target.get(&group_team_id) {
                let target = self.groups.get(&target_id).unwrap();
                let kills = target.n.min(group.damage(target) / target.hp);
                self.groups.entry(target_id).and_modify(|g| {
                    println!("Group {:?} attacks {:?}, killing {} units", group_team_id, target_id, kills);
                    g.n -= kills;
                });
            }
        }
        println!();
    }
}

#[derive(Debug)]
struct Group {
    team: Team,
    id: u32,
    n: i32,
    hp: i32,
    ap: i32,
    initiative: i32,
    attack_type: Type,
    weakness: HashSet<Type>,
    immunity: HashSet<Type>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.n * self.ap
    }

    fn is_alive(&self) -> bool {
        self.n > 0
    }

    fn damage(&self, other: &Group) -> i32 {
        if other.immunity.contains(&self.attack_type) {
            0
        } else if other.weakness.contains(&self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Team {
    ImmuneSystem,
    Infection,
}

impl Team {
    fn from_str(name: &str) -> Team {
        match name {
            "Immune System" => Team::ImmuneSystem,
            "Infection" => Team::Infection,
            _ => panic!("Unknown team: {}", name),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Type {
    Slashing,
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
}

impl Type {
    fn from_str(name: &str) -> Option<Type> {
        match name {
            "slashing" => Some(Type::Slashing),
            "bludgeoning" => Some(Type::Bludgeoning),
            "cold" => Some(Type::Cold),
            "fire" => Some(Type::Fire),
            "radiation" => Some(Type::Radiation),
            _ => panic!("Unknown type: {}", name)
        }
    }
}
