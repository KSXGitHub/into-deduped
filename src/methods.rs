use crate::functions::*;

/// Extension trait to convert a vector into a deduplicated vector.
pub trait IntoDeduped<Item>: crate::sealed::IsVec {
    /// Removes consecutive repeated elements in the vector according to the
    /// [`PartialEq`] trait implementation.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// This function calls [`Vec::dedup`] under the hook.
    fn into_deduped(self) -> Self
    where
        Item: PartialEq;

    /// Removes all but the first of consecutive elements in the vector that
    /// resolve to the same value according to the given equality function.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// This function calls [`Vec::dedup_by`] under the hook.
    fn into_deduped_by<SameBucket>(self, same_bucket: SameBucket) -> Self
    where
        SameBucket: FnMut(&mut Item, &mut Item) -> bool;

    /// Removes all but the first of consecutive elements in the vector that
    /// resolve to the same key.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// This function calls [`Vec::dedup_by_key`] under the hook.
    fn into_deduped_by_key<Key, GetKey>(self, key: GetKey) -> Self
    where
        GetKey: FnMut(&mut Item) -> Key,
        Key: PartialEq;
}

impl<Item> IntoDeduped<Item> for Vec<Item> {
    fn into_deduped(self) -> Self
    where
        Item: PartialEq,
    {
        into_deduped(self)
    }

    fn into_deduped_by<SameBucket>(self, same_bucket: SameBucket) -> Self
    where
        SameBucket: FnMut(&mut Item, &mut Item) -> bool,
    {
        into_deduped_by(self, same_bucket)
    }

    fn into_deduped_by_key<Key, GetKey>(self, key: GetKey) -> Self
    where
        GetKey: FnMut(&mut Item) -> Key,
        Key: PartialEq,
    {
        into_deduped_by_key(self, key)
    }
}
