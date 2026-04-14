//@build-pass

type Direct<T> = T; //~ WARN trivial type alias. this alias does nothing

type WithLifetime<'a, T> = T; //~ WARN trivial type alias. this alias does nothing

type NonTrivial<T> = (T, T);

fn main() {}
