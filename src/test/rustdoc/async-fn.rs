// ignore-tidy-linelength
// edition:2018
// @has async_fn/fn.foo.html '//pre[@class="rust fn"]' 'pub async fn foo() -> Option<Foo>'
pub async fn foo() -> Option<Foo> {
    None
}

// @has async_fn/fn.bar.html '//pre[@class="rust fn"]' 'pub async fn bar(a: i32, b: i32) -> i32'
pub async fn bar(a: i32, b: i32) -> i32 {
    0
}

// @has async_fn/fn.baz.html '//pre[@class="rust fn"]' 'pub async fn baz<T>(a: T) -> T'
pub async fn baz<T>(a: T) -> T {
    a
}

// @has async_fn/fn.qux.html '//pre[@class="rust fn"]' 'pub async unsafe fn qux() -> char'
pub async unsafe fn qux() -> char {
    '⚠'
}

// @has async_fn/fn.mut_args.html '//pre[@class="rust fn"]' 'pub async fn mut_args(a: usize)'
pub async fn mut_args(mut a: usize) {}

// @has async_fn/fn.mut_ref.html '//pre[@class="rust fn"]' 'pub async fn mut_ref(x: i32)'
pub async fn mut_ref(ref mut x: i32) {}

trait Bar {}

impl Bar for () {}

// @has async_fn/fn.quux.html '//pre[@class="rust fn"]' 'pub async fn quux() -> impl Bar'
pub async fn quux() -> impl Bar {
    ()
}

// @has async_fn/struct.Foo.html
// @matches - '//code' 'pub async fn f\(\)$'
// @matches - '//code' 'pub async unsafe fn g\(\)$'
// @matches - '//code' 'pub async fn mut_self\(self, first: usize\)$'
pub struct Foo;

impl Foo {
    pub async fn f() {}
    pub async unsafe fn g() {}
    pub async fn mut_self(mut self, mut first: usize) {}
}

pub trait Pattern<'a> {}

pub trait Trait<const N: usize> {}
// @has async_fn/fn.const_generics.html
// @has - '//pre[@class="rust fn"]' 'pub async fn const_generics<const N: usize>(_: impl Trait<N>)'
pub async fn const_generics<const N: usize>(_: impl Trait<N>) {}

// test that elided lifetimes are properly elided and not displayed as `'_`
// regression test for #63037
// @has async_fn/fn.elided.html
// @has - '//pre[@class="rust fn"]' 'pub async fn elided(foo: &str) -> &str'
pub async fn elided(foo: &str) -> &str {}
// This should really be shown as written, but for implementation reasons it's difficult.
// See `impl Clean for TyKind::Rptr`.
// @has async_fn/fn.user_elided.html
// @has - '//pre[@class="rust fn"]' 'pub async fn user_elided(foo: &str) -> &str'
pub async fn user_elided(foo: &'_ str) -> &str {}
// @has async_fn/fn.static_trait.html
// @has - '//pre[@class="rust fn"]' 'pub async fn static_trait(foo: &str) -> Box<dyn Bar>'
pub async fn static_trait(foo: &str) -> Box<dyn Bar> {}
// @has async_fn/fn.lifetime_for_trait.html
// @has - '//pre[@class="rust fn"]' "pub async fn lifetime_for_trait(foo: &str) -> Box<dyn Bar + '_>"
pub async fn lifetime_for_trait(foo: &str) -> Box<dyn Bar + '_> {}
// @has async_fn/fn.elided_in_input_trait.html
// @has - '//pre[@class="rust fn"]' "pub async fn elided_in_input_trait(t: impl Pattern<'_>)"
pub async fn elided_in_input_trait(t: impl Pattern<'_>) {}

struct AsyncFdReadyGuard<'a, T> { x: &'a T }

impl Foo {
    // @has async_fn/struct.Foo.html
    // @has - '//h4[@class="method"]' 'pub async fn complicated_lifetimes( &self, context: &impl Bar) -> impl Iterator<Item = &usize>'
    pub async fn complicated_lifetimes(&self, context: &impl Bar) -> impl Iterator<Item = &usize> {}
    // taken from `tokio` as an example of a method that was particularly bad before
    // @has - '//h4[@class="method"]' "pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()>"
    pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()> {}
    // @has - '//h4[@class="method"]' "pub async fn mut_self(&mut self)"
    pub async fn mut_self(&mut self) {}
}

// test named lifetimes, just in case
// @has async_fn/fn.named.html
// @has - '//pre[@class="rust fn"]' "pub async fn named<'a, 'b>(foo: &'a str) -> &'b str"
pub async fn named<'a, 'b>(foo: &'a str) -> &'b str {}
// @has async_fn/fn.named_trait.html
// @has - '//pre[@class="rust fn"]' "pub async fn named_trait<'a, 'b>(foo: impl Pattern<'a>) -> impl Pattern<'b>"
pub async fn named_trait<'a, 'b>(foo: impl Pattern<'a>) -> impl Pattern<'b> {}
