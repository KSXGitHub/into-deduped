/// Removes consecutive repeated elements in the vector according to the
/// [`PartialEq`] trait implementation.
///
/// If the vector is sorted, this removes all duplicates.
///
/// This function calls [`Vec::dedup`] under the hood.
pub fn into_deduped<Item>(mut vec: Vec<Item>) -> Vec<Item>
where
    Item: PartialEq,
{
    vec.dedup();
    vec
}

/// Removes all but the first of consecutive elements in the vector that
/// resolve to the same value according to the given equality function.
///
/// If the vector is sorted, this removes all duplicates.
///
/// This function calls [`Vec::dedup_by`] under the hood.
pub fn into_deduped_by<Item, SameBucket>(mut vec: Vec<Item>, same_bucket: SameBucket) -> Vec<Item>
where
    SameBucket: FnMut(&mut Item, &mut Item) -> bool,
{
    vec.dedup_by(same_bucket);
    vec
}

/// Removes all but the first of consecutive elements in the vector that
/// resolve to the same key.
///
/// If the vector is sorted, this removes all duplicates.
///
/// This function calls [`Vec::dedup_by_key`] under the hood.
pub fn into_deduped_by_key<Item, Key, GetKey>(mut vec: Vec<Item>, key: GetKey) -> Vec<Item>
where
    GetKey: FnMut(&mut Item) -> Key,
    Key: PartialEq,
{
    vec.dedup_by_key(key);
    vec
}
