pub struct GameState {
    tree: TreeState,
    ai: AI,
    messenger: Messenger,
}

impl GameState {
    pub fn new(root_statement: String, messenger: Messenger) -> Self {
        Self {
            tree: TreeState::new(root_statement),
            ai: AI {
                cooldown_until: Instant::now(),
                max_ai_cooldown_seconds: env::var("MAX_AI_COOLDOWN_SECONDS")
                    .expect("MAX_AI_COOLDOWN_SECONDS not in env")
                    .parse::<u64>()
                    .expect("MAX_AI_COOLDOWN_SECONDS must be a number."),
            },
            messenger,
        }
    }

    /// handle incoming messages from client(s). Returns a message to be sent only to the sender.
    pub async fn on_incoming_message(&mut self, incoming_message: ClientMessage) {
        //remember if we want to push the tree (as long as no error happens)
        let state_change = &mut match incoming_message {
            Add { .. } | Delete { .. } | Link { .. } | Unlink { .. } | Edit { .. } => true,
            _ => false,
        };

        //handle incoming messages from client(s)
        let result: Result<(), ProofError> = match incoming_message {
            Add { statement } => {
                let id = self.tree.add_node(statement);
                self.messenger.reply(ServerMessage::NewNodeId(id)).await;
                Ok(())
            }
            GetGameState => {
                self.messenger.reply_tree(&self.tree).await;
                Ok(())
            }
            Link { premise, conclusion } => self.tree.link(conclusion, premise),
            Unlink { premise, conclusion } => self.tree.unlink(conclusion, premise),
            Delete { id } => self.tree.remove_node(id),
            Edit { id, statement } => self.tree.change_node_statement(id, statement),
            ProveDirect { id } => self.prove_direct(id, state_change).await,
            ProveImplication { id } => self.prove_implication(id, state_change).await,
        };
        if let Err(e) = result {
            self.messenger.reply(ServerMessage::Error(e)).await;
        } else {
            if *state_change {
                self.messenger.send_tree(&self.tree).await;
                if self.tree.proof_complete() {
                    self.messenger.msg_win().await;
                }
            }
        }
    }

    pub async fn prove_direct(&mut self, id: Index, tree_changed: &mut bool) -> Result<(), ProofError> {
        self.messenger.send_cooldown(self.ai.max_ai_cooldown_seconds).await;
        match self.ai.check_statement(self.tree.get_statement(id)?).await {
            Ok(explanation) => {
                self.tree.set_directly_proven(id);
                *tree_changed = true;
                self.messenger.msg(id, explanation, true).await;
            }
            Err(explanation) => {
                self.messenger.msg(id, explanation, false).await;
            }
        }
        Ok(())
    }
    pub async fn prove_implication(&mut self, id: Index, tree_changed: &mut bool) -> Result<(), ProofError> {
        self.messenger.send_cooldown(self.ai.max_ai_cooldown_seconds).await;
        let conclusion = self.tree.get_statement(id)?;
        let premises = self.tree.get_premises(id)?;
        if premises.len() == 0 {
            self.messenger
                .msg(
                    id,
                    "You need to add at least one premise to prove an implication.".to_string(),
                    false,
                )
                .await;
            return Ok(());
        }
        match self.ai.check_implication(&premises, conclusion).await {
            Ok(explanation) => {
                self.tree.set_implied(id);
                *tree_changed = true;
                self.messenger.msg(id, explanation, true).await;
            }
            Err(explanation) => {
                self.messenger.msg(id, explanation, false).await;
            }
        }
        Ok(())
    }
}
