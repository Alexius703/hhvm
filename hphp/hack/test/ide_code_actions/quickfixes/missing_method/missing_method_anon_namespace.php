<?hh

namespace {

interface IFoo {
  public function bar(): void;
}

class Foo implements IFoo {}
                  // ^ at-caret

}
