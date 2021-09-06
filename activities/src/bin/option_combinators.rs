// NOTES:
// - Option.is_some(&self) -> bool
// - The Debug Macro: dbg!(variable); is handy!

fn main() {
    let a: Option<i32> = None;  // Or, Some(1);
    dbg!(a);


    let a_is_some = a.is_some();
    dbg!(a_is_some);

    let a_is_none = a.is_none();
    dbg!(a_is_none);

    // .map() ONLY applies if there is SOME data, if not then nothing happens (NONE)
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);


    // .filter() body of closure must return true or false. If return value
    // is TRUE, then we KEEP the data (e.g., return 'a'). Otherwise, we throwaway the optional data
    // When we throw away, the return value is NONE
    // NOTE .filter() borrows the inner value (&i32) so you need to borrow the
    // the comparison value as well inside closure body
    // NOTE You can borrow (&1) or you can access the raw data by DE-REFERENCING
    // the borrowed 'num' using *num == 1;
    // let a_filtered = a.filter(|num: &i32| num == &1);
    let a_filtered = a.filter(|num| *num == 1);
    dbg!(a_filtered);



    // .or_else() returns the original data if present, otherwise we use a
    // closure to specify what to return if 'a' originally has no data.
    // NOTE Since we're dealing with Optional data (Some or None), we still
    // need to 'match' on it later.
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);


    // .unwrap_or_else()
    // NOTE The difference between or_else() and unwrap_or_else() is that
    // or_else() -> Optional data (i.e., Some or None). unwrap_or_else() actually
    // takes out the inner data and stores in the variable (so you don't have to
    // use 'match' to access inner data). If the original 'a' has no data, then
    // the default value (i.e., 0 in this case) will be stored in variable.
    // This is useful to set a default value if no data is originally provided
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}
