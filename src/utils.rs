use traitdef::Delegate;

#[derive(Debug, PartialEq)]
pub enum ChangeType<'a, V: 'a> {
    Unchanged(&'a V),
    Modified(&'a V, &'a V),
}
#[derive(Default, Debug, PartialEq)]
pub struct Recorder<'a, V: 'a> {
    pub calls: Vec<ChangeType<'a, V>>,
}

impl<'a, V> Delegate<'a, V> for Recorder<'a, V> {
    fn unchanged(&mut self, v: &'a V) {
        self.calls.push(ChangeType::Unchanged(v));
    }
    fn modified(&mut self, v1: &'a V, v2: &'a V) {
        self.calls.push(ChangeType::Modified(v1, v2));
    }
}
