pub trait AuthenticatedKV {
    type K: Clone;
    type V: Clone;
    type LookupProof: Clone;
    type Commitment: Clone + Copy;

    fn new() -> Self;
    fn commit(&self) -> Self::Commitment;
    fn check_proof(
        key: Self::K,
        res: Option<Self::V>,
        pf: &Self::LookupProof,
        comm: &Self::Commitment,
    ) -> Option<()>;

    fn insert(self, key: Self::K, value: Self::V) -> Self;
    fn get(&self, key: Self::K) -> (Option<Self::V>, Self::LookupProof);
    fn remove(self, key: Self::K) -> Self;
}
