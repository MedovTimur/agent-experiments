pub struct BrokenVote {
    votes: Vec<u64>,
}

impl BrokenVote {
    pub fn create_poll(&self, title: String) -> PollId {
        self.votes.push(0);
        title.len()
    }

    pub fn vote(&mut self, poll_id: u64, option: u8) -> Result<(), VoteError> {
        let count = self.votes.get_mut(poll_id);
        count.increment_by(option);
        Ok("voted")
    }
}

