/// handles communication between client and server.
pub struct Messenger {
    pub sender: SplitSink<WebSocket, Message>,
}

#[derive(Serialize)]
pub enum ServerMessage {
    NewNodeId(Index),
    GameState(TreeStateDTO),
    Comment { id: Index, comment: String, success: bool },
    Win,
    AICooldown { seconds: u64 },
    Error(ProofError),
}
#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    GetGameState,
    Add { statement: String },
    Delete { id: Index },
    Edit { id: Index, statement: String },
    Link { premise: Index, conclusion: Index },
    Unlink { premise: Index, conclusion: Index },
    ProveDirect { id: Index },
    ProveImplication { id: Index },
}

impl Messenger {
    async fn send(&mut self, msg: ServerMessage) {
        let _ = self
            .sender
            .send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await;
    }
    async fn send_cooldown(&mut self, seconds: u64) {
        let _ = self.send(ServerMessage::AICooldown { seconds }).await;
    }
    async fn send_tree(&mut self, tree: &TreeState) {
        //push game state to client(s)
        let tree_dto: TreeStateDTO = tree.as_dto();
        let _ = self.send(ServerMessage::GameState(tree_dto)).await;
    }
    async fn msg(&mut self, id: Index, comment: String, success: bool) {
        //append message to node
        let _ = self.send(ServerMessage::Comment { id, comment, success }).await;
    }
    async fn msg_win(&mut self) {
        let _ = self.send(ServerMessage::Win).await;
    }

    /* Methods to (in future) only reply to the client that triggered some command */
    async fn reply(&mut self, msg: ServerMessage) {
        self.send(msg).await;
    }

    async fn reply_tree(&mut self, tree: &TreeState) {
        let tree_dto: TreeStateDTO = tree.as_dto();
        let _ = self.reply(ServerMessage::GameState(tree_dto)).await;
    }
}
