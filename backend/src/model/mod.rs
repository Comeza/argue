use generational_arena::Index;
use serde::{Deserialize, Serialize};

use self::proof::ProofState;
mod proof;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatementTree {
    statements: Vec<Statement>,
    root: Index,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statement {
    statement: String,
    state: ProofState,
    parents: Vec<Index>,
    children: Vec<Index>,
}
