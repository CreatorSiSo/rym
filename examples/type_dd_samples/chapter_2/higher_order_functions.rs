fn twice<A>(f: &dyn Fn(A) -> A) -> impl Fn(A) -> A + '_ {
	|x| f(f(x))
}
