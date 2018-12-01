use self::Token::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Rule<T: Debug + Clone + PartialEq + Hash + Eq> {
    pub name: String,
    pub production: Vec<Token<T>>,
}

#[derive(Clone, Debug)]
pub struct Grammar<T: Debug + Clone + PartialEq + Hash + Eq> {
    pub start: String,
    pub rules: Vec<Rule<T>>,
    pub first_sets: Option<HashMap<String, HashSet<(Token<T>, Rule<T>)>>>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Token<T: Debug + Clone + PartialEq + Hash + Eq> {
    Terminal(T),
    Epsilon,
    NonTerminal(String),
}

macro_rules! sentence {
    ($($i: ident),*) => {
        {
            let mut v = vec![];
            $(
                v.push(Terminal(stringify!($i)));
            )*
            v
        }
    };
}


impl<T: Debug + Clone + PartialEq + Hash + Eq> Grammar<T> {

    pub fn new(start: String, rules: Vec<Rule<T>>) -> Self {
        let first_sets = None;
        Self {
            start,
            rules,
            first_sets
        }
    }

    pub fn build(&mut self) -> Result<(), &'static str> {
        let mut temp = HashMap::new();
        for rule in &self.rules {
            let name = rule.name.clone();
            let first_set_for_rule = self.first_set(&name);
            // check first set clashes
            // println!("{}--------------", name);
            // println!("{:?}", first_set_for_rule);
            let mut temp_dedup = HashSet::new();
            for (tok,_) in &first_set_for_rule {
                // println!("tok: {:?}", tok);
                if temp_dedup.contains(tok) {
                    return Err("First/First clash");
                }
                temp_dedup.insert(tok.clone());
            }
            temp.insert(name, first_set_for_rule);
        }

        self.first_sets = Some(temp);

        Ok(())
    }

    fn parse(&self, sent: Vec<Token<T>>) -> Result<(), &'static str> {
        let mut sent = sent;
        self.parse_aux(&self.start, &mut sent)?;
        if sent.is_empty() {
            return Ok(())
        } else {
            return Err("Incomplete parse")
        }
    }

    fn parse_aux(&self, name: &str, sent: &mut Vec<Token<T>>) -> Result<(), &'static str> {
        let mut sent = sent;
        let firsts = self.first_sets.as_ref().map(|i|i.get(name).unwrap()).unwrap();
        println!("Parsing rule {} with {:?}", name, sent);

        let mut prod = if sent.is_empty() {
            match firsts.iter().find(|prod| prod.0 == Epsilon) {
                None => { return Err("Does not contain epsilon. 1"); }
                Some((_,i)) => i.production.clone(),
            }
        } else {
            let rule = firsts.iter().find(|prod|prod.0 == sent[0]);
            match rule {
                Some(i) => i.1.production.clone(),
                None => // match epsilon
                    match firsts.iter().find(|prod| prod.0 == Epsilon) {
                        None => { return Err("Does not contain epsilon. 2"); }
                        Some((_,i)) => i.production.clone(),
                    }
            }
        };
        println!("Found: {:?}", prod);
        self.match_rule(&mut sent, &mut prod)
    }

    fn match_rule(&self, sent: &mut Vec<Token<T>>, rule: &mut Vec<Token<T>>) -> Result<(), &'static str> {
        while let Some(t) = rule.get(0) {
            let t = t.clone();
            rule.remove(0);
            match t {
                Epsilon => {
                    println!("Matching Epsilon");
                }
                Terminal(ref term) => {
                    println!("Matching {:?}", term);
                    if let Some(sent_tok) = sent.get(0) {
                        if sent_tok.clone() == t {
                            sent.remove(0);
                        }
                    } else {
                        return Err("2")
                    }
                }
                NonTerminal(s) => self.parse_aux(&s, sent)?
            }
        }

        Ok(())
    }

    fn first_set(&self, name: &str) -> HashSet<(Token<T>, Rule<T>)> {
        let mut ret = HashSet::new();
        for rule in &self.rules {
            if rule.name != name {
                continue;
            }
            // this is incorrect...
            let first_tok = rule.production[0].clone();
            match first_tok {
                Terminal(_) | Epsilon => {ret.insert((first_tok, rule.clone()));}
                NonTerminal(s) => ret.extend(self.first_set(&s)),
            };
        }
        ret
    }
}
