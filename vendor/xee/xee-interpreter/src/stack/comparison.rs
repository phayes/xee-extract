use crate::atomic;
use crate::atomic::AtomicCompare;
use crate::context;
use crate::error;

// https://www.w3.org/TR/xpath-31/#id-general-comparisons
// step 1, atomization, has already taken place
pub(crate) fn general_comparison<O>(
    a_atoms: impl Iterator<Item = error::Result<atomic::Atomic>>,
    b_atoms: impl Iterator<Item = error::Result<atomic::Atomic>> + std::clone::Clone,
    context: &context::DynamicContext,
    _op: O,
) -> error::Result<bool>
where
    O: AtomicCompare,
{
    let b_atoms = b_atoms.collect::<Vec<_>>();
    let collation = context.static_context().default_collation()?;
    let implicit_timezone = context.implicit_timezone();
    for a in a_atoms {
        let a = a?;
        for b in b_atoms.iter() {
            let (a, b) = cast(a.clone(), b.clone()?, context.static_context())?;
            // 2c do value comparison
            if O::atomic_compare(
                a,
                b,
                |a: &str, b: &str| collation.compare(a, b),
                implicit_timezone,
            )? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// step 2: cast
fn cast(
    a: atomic::Atomic,
    b: atomic::Atomic,
    context: &context::StaticContext,
) -> error::Result<(atomic::Atomic, atomic::Atomic)> {
    Ok(match (&a, &b) {
        // step 2a: if both are untyped atomic, cast them both to string
        (atomic::Atomic::Untyped(_), atomic::Atomic::Untyped(_)) => {
            let a = a.cast_to_string();
            let b = b.cast_to_string();
            (a, b)
        }
        // step 2bi: if untyped is combined with numeric, cast to double
        (atomic::Atomic::Untyped(_), _) => {
            let a = if b.is_numeric() {
                a.cast_to_double()?
            } else {
                // step 2biv: in all other cases, cast untyped to primitive base type of other
                a.cast_to_schema_type_of(&b, context)?
            };
            (a, b)
        }
        (_, atomic::Atomic::Untyped(_)) => {
            let b = if a.is_numeric() {
                b.cast_to_double()?
            } else {
                // step 2biv: in all other cases, cast untyped to primitive base type of other
                b.cast_to_schema_type_of(&a, context)?
            };
            (a, b)
        } // step 2bii & 2biii skipped until we have datetime stuff
        _ => (a, b),
    })
}
