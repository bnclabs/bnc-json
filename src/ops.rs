// Copyright (c) 2018 R Pratap Chakravarthy and AUTHORS

use std::ops::{Add, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use std::ops::{BitAnd, BitOr, BitXor, Index};

use lazy_static::lazy_static;

use crate::json::Json;
use crate::property::{self, Property};

// TODO: use macro to implement Add<Json> and Add<&Json> and similar variant
//       for Sub, Mul, Div, Neg ...

impl Add for Json {
    type Output = Json;

    fn add(self, rhs: Json) -> Json {
        use crate::json::Json::{Array, Float, Integer, Null, Object, String as S};

        match (&self, &rhs) {
            (Null, _) => rhs.clone(),  // Identity operation
            (_, Null) => self.clone(), // Identity operation
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                Json::new(l + r)
            }
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                Json::new(l + r)
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                Json::new((l as f64) + r)
            }
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                Json::new(l + (r as f64))
            }
            (S(l), S(r)) => {
                let mut s = String::new();
                s.push_str(l);
                s.push_str(r);
                S(s)
            }
            (Array(l), Array(r)) => {
                let mut a = vec![];
                a.extend_from_slice(l);
                a.extend_from_slice(r);
                Array(a)
            }
            (Object(l), Object(r)) => {
                use crate::json;

                let mut obj = Json::Object(Vec::new());
                l.to_vec()
                    .into_iter()
                    .for_each(|p| json::insert(&mut obj, p));
                r.to_vec()
                    .into_iter()
                    .for_each(|p| json::insert(&mut obj, p));
                obj
            }
            (_, _) => Json::__Error(format!("invalid {} + {}", self, rhs)),
        }
    }
}

impl Sub for Json {
    type Output = Json;

    fn sub(self, rhs: Json) -> Json {
        use crate::json::Json::{Array, Float, Integer, Null, Object};

        match (&self, &rhs) {
            (Null, _) => rhs.clone(),  // Identity operation
            (_, Null) => self.clone(), // Identity operation
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                Json::new(l - r)
            }
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                Json::new(l - r)
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                Json::new((l as f64) - r)
            }
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                Json::new(l - (r as f64))
            }
            (Array(lhs), Array(rhs)) => {
                let mut res = lhs.clone();
                rhs.iter().for_each(|x| {
                    if let Some(pos) = res.iter().position(|y| *x == *y) {
                        res.remove(pos);
                    }
                });
                Array(res)
            }
            (Object(lhs), Object(rhs)) => {
                let mut res = lhs.clone();
                rhs.iter().for_each(|x| {
                    if let Some(pos) = res.iter().position(|y| *x == *y) {
                        res.remove(pos);
                    }
                });
                Object(res)
            }
            (_, _) => Json::__Error(format!("invalid {} - {}", self, rhs)),
        }
    }
}

impl Mul for Json {
    type Output = Json;

    fn mul(self, rhs: Json) -> Json {
        use crate::json::Json::{Float, Integer, Null, Object, String as S};

        match (&self, &rhs) {
            (Null, _) => Json::Null,
            (_, Null) => Json::Null,
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                Json::new(l * r)
            }
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                Json::new(l * r)
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                Json::new((l as f64) * r)
            }
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                Json::new(l * (r as f64))
            }
            (S(s), Integer(_)) => {
                let n = rhs.integer().unwrap();
                if n == 0 {
                    Null
                } else {
                    S(s.repeat(n as usize))
                }
            }
            (Integer(_), S(s)) => {
                let n = self.integer().unwrap();
                if n == 0 {
                    Null
                } else {
                    S(s.repeat(n as usize))
                }
            }
            (Object(this), Object(other)) => {
                // TODO: this is not well defined.
                let mut obj = Vec::new();
                obj = mixin_object(obj, this.to_vec());
                obj = mixin_object(obj, other.to_vec());
                Json::Object(obj)
            }
            (_, _) => Json::__Error(format!("invalid {} * {}", self, rhs)),
        }
    }
}

impl Div for Json {
    type Output = Json;

    fn div(self, rhs: Json) -> Json {
        use crate::json::Json::{Float, Integer, Null, String as S};

        match (&self, &rhs) {
            (Null, _) => Json::Null,
            (_, Null) => Json::Null,
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                if r == 0 {
                    Null
                } else {
                    Json::new(l / r)
                }
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new((l as f64) / r)
                }
            }
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                if r == 0 {
                    Null
                } else {
                    Json::new(l / (r as f64))
                }
            }
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new(l / r)
                }
            }
            (S(s), S(patt)) => {
                // TODO: not yet defined
                let arr = s.split(patt).map(|s| S(s.to_string())).collect();
                Json::Array(arr)
            }
            (_, _) => Json::__Error(format!("invalid {} / {}", self, rhs)),
        }
    }
}

impl Rem for Json {
    type Output = Json;

    fn rem(self, rhs: Json) -> Json {
        use crate::json::Json::{Float, Integer, Null};

        match (&self, &rhs) {
            (Null, _) => Json::Null,
            (_, Null) => Json::Null,
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                if r == 0 {
                    Null
                } else {
                    Json::new(l % r)
                }
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new((l as f64) % r)
                }
            }
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                if r == 0 {
                    Null
                } else {
                    Json::new(l % (r as f64))
                }
            }
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new(l % r)
                }
            }
            (_, _) => Json::__Error(format!("invalid {} % {}", self, rhs)),
        }
    }
}

impl Neg for Json {
    type Output = Json;

    fn neg(self) -> Json {
        match self {
            Json::Null => Json::Null,
            Json::Integer(_) => Json::new(-self.integer().unwrap()),
            Json::Float(_) => Json::new(-self.float().unwrap()),
            _ => Json::__Error(format!("invalid -{}", self)),
        }
    }
}

impl Shl for Json {
    type Output = Json;

    fn shl(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l << r),
            (_, _) => Json::__Error(format!("invalid {} % {}", self, rhs)),
        }
    }
}

impl Shr for Json {
    type Output = Json;

    fn shr(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l >> r),
            (_, _) => Json::__Error(format!("invalid {} % {}", self, rhs)),
        }
    }
}

impl BitAnd for Json {
    type Output = Json;

    fn bitand(self, rhs: Json) -> Json {
        use crate::json::Json::Integer;

        match (self, rhs) {
            (x @ Json::__Error(_), _) => x,
            (_, y @ Json::__Error(_)) => y,
            (x @ Integer(_), y @ Integer(_)) => {
                (x.integer().unwrap() & y.integer().unwrap()).into()
            }
            (x, y) => {
                let (x, y): (bool, bool) = (x.into(), y.into());
                (x & y).into()
            }
        }
    }
}

impl BitOr for Json {
    type Output = Json;

    fn bitor(self, rhs: Json) -> Json {
        use crate::json::Json::Integer;

        match (self, rhs) {
            (x @ Json::__Error(_), _) => x,
            (_, y @ Json::__Error(_)) => y,
            (x @ Integer(_), y @ Integer(_)) => {
                (x.integer().unwrap() | y.integer().unwrap()).into()
            }
            (x, y) => {
                let (x, y): (bool, bool) = (x.into(), y.into());
                (x | y).into()
            }
        }
    }
}

impl BitXor for Json {
    type Output = Json;

    fn bitxor(self, rhs: Json) -> Json {
        use crate::json::Json::Integer;

        match (self, rhs) {
            (x @ Json::__Error(_), _) => x,
            (_, y @ Json::__Error(_)) => y,
            (x @ Integer(_), y @ Integer(_)) => {
                (x.integer().unwrap() ^ y.integer().unwrap()).into()
            }
            (x, y) => {
                let (x, y): (bool, bool) = (x.into(), y.into());
                (x ^ y).into()
            }
        }
    }
}

impl Not for Json {
    type Output = Json;

    fn not(self) -> Json {
        if self.is_error() {
            return self;
        }
        let value: bool = self.into();
        value.not().into()
    }
}

lazy_static! {
    pub static ref INDEX_ARRAY_ERROR: Json = Json::__Error("cannot index non-array".to_string());
    pub static ref INDEX_OUTOFBOUND: Json = Json::__Error("index out of bound".to_string());
    pub static ref INDEX_OBJECT_ERROR: Json = Json::__Error("cannot index non-object".to_string());
    pub static ref INDEX_MISSING_KEY: Json = Json::__Error("key not found".to_string());
}
impl Index<isize> for Json {
    type Output = Json;

    fn index(&self, index: isize) -> &Json {
        match self {
            Json::__Error(_) => self,
            Json::Array(arr) => match normalized_offset(index, arr.len()) {
                Some(off) => &arr[off],
                None => &INDEX_OUTOFBOUND,
            },
            _ => &INDEX_ARRAY_ERROR,
        }
    }
}

impl Index<&str> for Json {
    type Output = Json;

    fn index(&self, index: &str) -> &Json {
        match self {
            Json::__Error(_) => self,
            Json::Object(obj) => match property::search_by_key(obj, index) {
                Ok(off) => obj[off].value_ref(),
                Err(_) => &INDEX_MISSING_KEY,
            },
            _ => &INDEX_OBJECT_ERROR,
        }
    }
}

//// TODO: To handle || and && short-circuiting operations.
////impl And for Json {
////    type Output=Json;
////
////    fn and(self, other: Json) -> Self::Output {
////        let lhs = bool::from(self);
////        let rhs = bool::from(other);
////        Json::Bool(lhs & rhs)
////    }
////}
////
////impl Or for Json {
////    type Output=Json;
////
////    fn or(self, other: Json) -> Self::Output {
////        let lhs = bool::from(self);
////        let rhs = bool::from(other);
////        Json::Bool(lhs | rhs)
////    }
////}

fn mixin_object(mut this: Vec<Property>, other: Vec<Property>) -> Vec<Property> {
    use crate::json::Json::Object;

    for o in other.into_iter() {
        match property::search_by_key(&this, o.key_ref()) {
            Ok(off) => match (this[off].clone().value(), o.clone().value()) {
                (Object(val), Object(val2)) => this[off].set_value(Object(mixin_object(val, val2))),
                _ => this.insert(off, o),
            },
            Err(off) => this.insert(off, o.clone()),
        }
    }
    this
}

pub fn normalized_offset(off: isize, len: usize) -> Option<usize> {
    let len = len as isize;
    let off = if off < 0 { off + len } else { off };
    if off >= 0 && off < len {
        Some(off as usize)
    } else {
        None
    }
}
