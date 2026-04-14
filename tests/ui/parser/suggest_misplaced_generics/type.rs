// Issue: 103366 , Suggest fix for misplaced generic params
//@ run-rustfix

#[allow(unused, trivial_type_alias)]
type<T> Foo = T;
//~^ ERROR expected identifier, found `<`
//~| HELP place the generic parameter name after the type name

fn main() {}
