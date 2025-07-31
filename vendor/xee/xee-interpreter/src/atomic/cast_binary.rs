use crate::atomic;
use crate::error;

pub(crate) fn cast_binary_arithmetic(
    a: atomic::Atomic,
    b: atomic::Atomic,
) -> error::Result<(atomic::Atomic, atomic::Atomic)> {
    let a = cast_untyped_arithmetic(a)?;
    let b = cast_untyped_arithmetic(b)?;

    cast_binary(a, b)
}

pub(crate) fn cast_binary_compare(
    a: atomic::Atomic,
    b: atomic::Atomic,
) -> error::Result<(atomic::Atomic, atomic::Atomic)> {
    let a = cast_untyped_compare(a);
    let b = cast_untyped_compare(b);

    cast_binary(a, b)
}

fn cast_untyped_arithmetic(value: atomic::Atomic) -> error::Result<atomic::Atomic> {
    // https://www.w3.org/TR/xpath-31/#id-arithmetic
    // 4: If an atomized operand of of type xs:untypedAtomic, it is cast
    // to xs:double
    if let atomic::Atomic::Untyped(s) = value {
        atomic::Atomic::parse_atomic::<f64>(&s)
    } else {
        Ok(value)
    }
}

fn cast_untyped_compare(value: atomic::Atomic) -> atomic::Atomic {
    // 3.7.1 Value Comparisons
    // 4: If an atomized operand of of type xs:untypedAtomic, it is cast
    // to xs:string
    if let atomic::Atomic::Untyped(s) = value {
        atomic::Atomic::String(atomic::StringType::String, s)
    } else {
        value
    }
}

fn cast_binary(
    a: atomic::Atomic,
    b: atomic::Atomic,
) -> error::Result<(atomic::Atomic, atomic::Atomic)> {
    use atomic::Atomic::*;

    match (&a, &b) {
        // Cast a to a float
        (Decimal(_), Float(_)) | (Integer(_, _), Float(_)) => Ok((a.cast_to_float()?, b)),
        // Cast b to a float
        (Float(_), Decimal(_)) | (Float(_), Integer(_, _)) => Ok((a, b.cast_to_float()?)),
        // Cast a to a double
        (Decimal(_), Double(_)) | (Integer(_, _), Double(_)) | (Float(_), Double(_)) => {
            Ok((a.cast_to_double()?, b))
        }
        // Cast b to a double
        (Double(_), Decimal(_)) | (Double(_), Integer(_, _)) | (Double(_), Float(_)) => {
            Ok((a, b.cast_to_double()?))
        }
        // Cast integer to decimal
        (Decimal(_), Integer(_, _)) => Ok((a, b.cast_to_decimal()?)),
        (Integer(_, _), Decimal(_)) => Ok((a.cast_to_decimal()?, b)),

        // cast any DateTimeStamp to a DateTime
        (DateTimeStamp(_), DateTime(_)) => Ok((a.cast_to_date_time()?, b)),
        (DateTime(_), DateTimeStamp(_)) => Ok((a, b.cast_to_date_time()?)),
        // otherwise, we don't cast
        _ => Ok((a, b)),
    }
}
