use core::cmp::max;
use core::ops::{Add,Mul};

use ff::Field;

use std::{
    convert::TryFrom,
    ops::{Neg,Sub}
};

use super::{lookup,permutation,Assigned,Error};
use crate::{
    circuit::{Layouter, Region, Value},
    poly::Rotataion,
};

mod compress_selectors;




