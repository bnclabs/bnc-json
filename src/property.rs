// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use crate::json::Json;

/// Property type captures a single (key,value) pair in a JSON object.
///
/// Where,
/// * **key** is [String] type, defined by JSON specification.
/// * **value** is JSON value.
///
/// Implements [PartialEq] and [PartialOrd] for equality and ordering.
///
/// [string]: std::string::String
/// [PartialEq]: std::cmp::PartialEq
/// [PartialOrd]: std::cmp::PartialOrd
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Property(String, Json);

/// Following inherent methods are self explanatory, typically
/// used to move, or obtain a reference for key or value
/// component of a property.
impl Property {
    #[inline]
    pub fn new<T>(key: T, value: Json) -> Property
    where
        T: ToString,
    {
        Property(key.to_string(), value)
    }

    #[inline]
    pub fn into_key(self) -> String {
        self.0
    }

    #[inline]
    pub fn as_ref_key(&self) -> &String {
        &self.0
    }

    #[inline]
    pub fn into_value(self) -> Json {
        self.1
    }

    #[inline]
    pub fn as_ref_value(&self) -> &Json {
        &self.1
    }

    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Json {
        &mut self.1
    }

    #[inline]
    pub fn set_key(&mut self, key: String) {
        self.0 = key;
    }

    #[inline]
    pub fn set_value(&mut self, value: Json) {
        self.1 = value;
    }
}

pub fn search_by_key(obj: &[Property], key: &str) -> Result<usize, usize> {
    use std::cmp::Ordering::{Equal, Greater, Less};

    let mut size = obj.len();
    if size == 0 {
        return Err(0);
    }

    let mut base = 0_usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // mid is always in [0, size), that means mid is >= 0 and < size.
        // mid >= 0: by definition
        // mid < size: mid = size / 2 + size / 4 + size / 8 ...
        let item: &str = obj[mid].as_ref_key();
        let cmp = item.cmp(key);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // base is always in [0, size) because base <= mid.
    let item: &str = obj[base].as_ref_key();
    let cmp = item.cmp(key);
    if cmp == Equal {
        Ok(base)
    } else {
        Err(base + usize::from(cmp == Less))
    }
}

pub fn upsert_object_key(obj: &mut Vec<Property>, prop: Property) {
    match search_by_key(obj, prop.as_ref_key()) {
        Ok(off) => obj[off] = prop,
        Err(off) => obj.insert(off, prop),
    }
}
