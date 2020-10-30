pub struct EntryID {
    file_id: u128,
    entry_seq: u128,
}

pub struct EntryOffset {
    entry_id: EntryID,
    frame_first: usize,
}
