use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub name: String,
    pub weight: i32,
    pub disc: HashSet<String>,
}

impl Hash for Program {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Program {
    pub fn new(name: String, weight: i32) -> Self {
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
}

impl Tower {
    pub fn new() -> Self {
        Tower {
            programs: HashMap::new(),
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
}
