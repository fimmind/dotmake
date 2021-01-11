use crate::config::Identifier;
use std::collections::HashSet;
use std::error::Error;
use structopt::StructOpt;

/// Perform installation of given rules
#[derive(Debug, StructOpt)]
pub struct Install {
    /// Rules to be installed
    #[structopt(required = true)]
    rules: Vec<Identifier>,
}

#[derive(Debug)]
pub struct InstallationState {
    rules_stack: Vec<Identifier>,
    performed_rules: HashSet<Identifier>,
}

impl InstallationState {
    fn init(rules: &[Identifier]) -> Self {
        InstallationState {
            rules_stack: Vec::from(rules),
            performed_rules: HashSet::new(),
        }
    }
}

pub trait Rule {
    type Deps: IntoIterator<Item = Identifier>;
    type PostDeps: IntoIterator<Item = Identifier>;

    /// Perform all the actions for the given rule
    fn perform(&self, state: &InstallationState) -> Result<(), Box<dyn Error>>;

    /// List of dependencies of the given rule
    fn get_deps(&self, state: &InstallationState) -> Self::Deps;

    /// List of rules, that are specified to be performed after the given rule
    fn get_post_deps(&self, state: &InstallationState) -> Self::PostDeps;
}

pub trait Config {
    type Rule: Rule;
    fn get_rule(&self, ident: &Identifier) -> Option<&Self::Rule>;
}

struct MockConfig;
struct MockRule;

impl Rule for MockRule {
    type Deps = Vec<Identifier>;
    type PostDeps = Vec<Identifier>;

    fn perform(&self, state: &InstallationState) -> Result<(), Box<dyn Error>> {
        print_info!("Performing");
        Ok(())
    }

    fn get_deps(&self, state: &InstallationState) -> Self::Deps {
        print_info!("Getting deps");
        vec![]
    }

    fn get_post_deps(&self, state: &InstallationState) -> Self::PostDeps {
        print_info!("Getting post deps");
        vec![]
    }
}

impl Config for MockConfig {
    type Rule = MockRule;

    fn get_rule(&self, ident: &Identifier) -> Option<&Self::Rule> {
        Some(&MockRule)
    }
}

fn get_rule(ident: &Identifier) -> Result<&<MockConfig as Config>::Rule, Box<dyn Error>> {
    Config::get_rule(&MockConfig, ident).ok_or_else(|| format!("Undefined rule: {}", ident).into())
}

impl Install {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Figure out what to do with loops in dependencies
        let mut state = InstallationState::init(&self.rules);

        while let Some(rule_identifier) = state.rules_stack.pop() {
            if !state.performed_rules.contains(&rule_identifier) {
                let rule = get_rule(&rule_identifier)?;

                let mut deps_left = false;
                for dep in Rule::get_deps(rule, &state) {
                    if !state.performed_rules.contains(&dep) {
                        if deps_left == false {
                            state.rules_stack.push(rule_identifier.clone());
                            deps_left = true;
                        }
                        state.rules_stack.push(dep);
                    }
                }
                if deps_left {
                    continue;
                }

                print_info!("Installing '{}'", rule_identifier);
                Rule::perform(rule, &state)?;
                state.performed_rules.insert(rule_identifier.clone());
                state.rules_stack.extend(Rule::get_post_deps(rule, &state));
            }
        }

        Ok(())
    }
}
