#![deny(warnings)]
use sha2;
use sha2::Digest as Sha2Digest;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Digest(pub digest::Output<sha2::Sha256>);

impl core::fmt::Debug for Digest {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str("sha256:")?;
        for b in self.0.iter() {
            fmt.write_fmt(format_args!("{:02x}", b))?;
        }
        Ok(())
    }
}

pub fn zero_digest() -> Digest {
    Digest(*digest::Output::<sha2::Sha256>::from_slice(
        &vec![0u8; sha2::Sha256::output_size()],
    ))
}

impl From<digest::Output<sha2::Sha256>> for Digest {
    fn from(d: digest::Output<sha2::Sha256>) -> Self {
        Self(d)
    }
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub fn hash_one_thing<T1>(label1: &str, v1: T1) -> Digest
where
    T1: AsRef<[u8]>,
{
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hash_one_thing");
    {
        hasher.update(label1.as_bytes());
        let r1: &[u8] = v1.as_ref();
        hasher.update(r1.len().to_le_bytes());
        hasher.update(r1);
    }

    hasher.finalize().into()
}

pub fn hash_two_things<T1, T2>(
    label1: &str,
    label2: &str,
    v1: T1,
    v2: T2,
) -> Digest
where
    T1: AsRef<[u8]>,
    T2: AsRef<[u8]>,
{
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hash_two_things");
    {
        hasher.update(label1.as_bytes());
        let r1: &[u8] = v1.as_ref();
        hasher.update(r1.len().to_le_bytes());
        hasher.update(r1);
    }

    {
        hasher.update(label2.as_bytes());
        let r2: &[u8] = v2.as_ref();
        hasher.update(r2.len().to_le_bytes());
        hasher.update(r2);
    }

    hasher.finalize().into()
}
