mod user;

pub trait Model<Key: PartialEq + PartialOrd + Copy> {
    type KeyType;

    fn get_id(&self) -> Key;
    fn set_id(&mut self, id: Key);
}
