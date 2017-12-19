use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub name: String,
    pub weight: u32,
    pub disc: HashSet<String>,
}

impl Hash for Program {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Program {
    pub fn new(name: String, weight: u32) -> Self {
        Program {
            name: name,
            weight: weight,
            disc: HashSet::new(),
        }
    }

    pub fn insert(&mut self, child: String) {
        self.disc.insert(child);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tower {
    programs: HashMap<String, Program>,
    weights: HashMap<String, u32>,
}

impl Tower {
    pub fn new() -> Self {
        Tower {
            programs: HashMap::new(),
            weights: HashMap::new(),
        }
    }

    pub fn add(&mut self, program: Program) {
        self.programs.insert(program.name.clone(), program);
    }

    pub fn link(&mut self, parent_name: &str, child_name: &str) {
        let parent = self.programs.get_mut(parent_name).unwrap();
        parent.insert(child_name.to_owned());
    }

    pub fn root(&self) -> Option<&Program> {
        let mut children: HashSet<&str> = HashSet::with_capacity(self.programs.len());
        let mut parents: HashSet<&str> = HashSet::with_capacity(self.programs.len());

        for (parent, p) in self.programs.iter() {
            parents.insert(parent);
            for child in &p.disc {
                children.insert(&child);
            }
        }

        parents
            .difference(&children)
            .next()
            .and_then(|&n| self.programs.get(n))
    }

    fn weigh(&mut self, name: &str) {
        let mut sorted = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(name);

        while let Some(n) = q.pop_front() {
            if self.weights.contains_key(n) {
                continue
            }

            sorted.push(n);
            let p = self.programs.get(n).unwrap();
            for c in &p.disc {
                q.push_back(c);
            }
        }

        while let Some(n) = sorted.pop() {
            let p = self.programs.get(n).unwrap();
            let mut w = p.weight;
            for c in &p.disc {
                w += self.weights.get(c).unwrap();
            }
            self.weights.insert(n.to_owned(), w);
        }
    }

    pub fn get_total_weight(&mut self, name: &str) -> Option<u32> {
        if !self.weights.contains_key(name) {
            self.weigh(name);
        }

        self.weights.get(name).map(|x| *x)
    }

    pub fn find_imbalance(&self) -> Option<(&str, u32)> {
        Some(("root", 0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_root() {
        let mut tower = Tower::new();

        tower.add(Program::new("foo".into(), 10));
        tower.add(Program::new("bar".into(), 10));
        tower.add(Program::new("baz".into(), 10));
        tower.add(Program::new("eggs".into(), 10));
        tower.add(Program::new("spam".into(), 10));

        tower.link("foo", "bar");
        tower.link("bar", "baz");
        tower.link("eggs", "spam");
        tower.link("foo", "eggs");

        let mut root = Program::new("foo".into(), 10);
        root.insert("bar".into());
        root.insert("eggs".into());
        assert_eq!(tower.root(), Some(&root));
    }

    #[test]
    fn test_get_total_weight() {
        let mut tower = Tower::new();

        tower.add(Program::new("foo".into(), 1));
        tower.add(Program::new("bar".into(), 3));
        tower.add(Program::new("baz".into(), 5));
        tower.add(Program::new("eggs".into(), 7));
        tower.add(Program::new("spam".into(), 9));

        tower.link("foo", "bar");
        tower.link("bar", "baz");
        tower.link("bar", "eggs");
        tower.link("eggs", "spam");

        assert_eq!(tower.get_total_weight("bar"), Some(3 + 5 + 7 + 9));
    }
}
