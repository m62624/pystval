use std::{
    fmt::{Debug, Display},
    ops::RangeInclusive,
    str::FromStr,
};

use super::rules::{next::NextStep, traits::IntoConcreteType};

use super::{convert::FromBytes, *};

fn single_range_bytes_check<
    'a,
    T: FromStr + Copy + Debug + Display + PartialOrd + FromBytes<T>,
    C: IntoConcreteType<'a>,
>(
    numbers: &mut CaptureData<'a, C>,
    range: &RangeInclusive<T>,
    range_mode: RangeMode,
    read_mode: &ReadMode,
) -> bool {
    match range_mode {
        RangeMode::Any => {
            numbers
                .text_for_capture
                .iter()
                .filter(|&num| {
                    let num = match read_mode {
                        ReadMode::FromBeBytes => {
                            T::from_be_bytes_non_const(num.into_bytes().unwrap())
                        }
                        ReadMode::FromLeBytes => {
                            T::from_le_bytes_non_const(num.into_bytes().unwrap())
                        }
                        ReadMode::FromUtf8 => T::from_utf8(num.into_bytes().unwrap()),
                    };
                    num.map(|num| range.contains(&num)).unwrap_or(false)
                })
                .count()
                > 0
        }
        RangeMode::All => {
            numbers
                .text_for_capture
                .iter()
                .filter(|&num| {
                    let num = match read_mode {
                        ReadMode::FromBeBytes => {
                            T::from_be_bytes_non_const(num.into_bytes().unwrap())
                        }
                        ReadMode::FromLeBytes => {
                            T::from_le_bytes_non_const(num.into_bytes().unwrap())
                        }
                        ReadMode::FromUtf8 => T::from_utf8(num.into_bytes().unwrap()),
                    };
                    num.map(|num| range.contains(&num)).unwrap_or(false)
                })
                .count()
                == numbers.text_for_capture.len()
        }
        RangeMode::Exactly(target_count) => {
            let required_count = target_count.min(numbers.text_for_capture.len());
            numbers
                .text_for_capture
                .iter()
                .filter(|&num| {
                    let num = match read_mode {
                        ReadMode::FromBeBytes => {
                            T::from_be_bytes_non_const(num.into_bytes().unwrap())
                        }
                        ReadMode::FromLeBytes => {
                            T::from_le_bytes_non_const(num.into_bytes().unwrap())
                        }
                        ReadMode::FromUtf8 => T::from_utf8(num.into_bytes().unwrap()),
                    };
                    num.map(|num| range.contains(&num)).unwrap_or(false)
                })
                .take(required_count)
                .count()
                == required_count
        }
    }
}

pub fn single_bytes_result<
    'a,
    T: Debug + FromStr + Copy + Display + PartialOrd + FromBytes<T>,
    C: IntoConcreteType<'a>,
>(
    range_bytes: &RangeBytes,
    captures: &mut CaptureData<'a, C>,
    value: &RangeInclusive<T>,
) -> NextStep {
    if single_range_bytes_check(
        captures,
        value,
        range_bytes.range_mode,
        &range_bytes.read_mode,
    ) {
        return NextStep::Finish;
    } else {
        return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
    }
}
