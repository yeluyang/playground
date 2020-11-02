#[derive(Debug)]
pub struct EntryID {
    pub file_id: u128,
    pub entry_seq: u128,
}

#[derive(Debug)]
pub struct EntryOffset {
    pub entry_id: EntryID,
    pub first_frame: usize,
}

impl EntryOffset {
    pub fn new(file_id: u128, entry_seq: u128, first_frame: usize) -> EntryOffset {
        return EntryOffset {
            entry_id: EntryID { file_id, entry_seq },
            first_frame,
        };
    }
}
