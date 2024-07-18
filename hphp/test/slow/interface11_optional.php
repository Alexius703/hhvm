<?hh

// derived from interface11.php, with defaults in interface replaced by optional

class A {}
interface I {
  public function a($x, optional AnyArray $a):mixed;
  public function b(optional AnyArray $a, $x):mixed;
  public function c($x, A $a1, optional A $a2, A $a3, $y):mixed;
  public function d(optional AnyArray $a, optional $x, $y):mixed;
}
class B implements I {
  public function a($x, AnyArray $a = vec[]) :mixed{}
  public function b(AnyArray $a = null, $x) :mixed{}
  public function c($x, A $a1, A $a2=null, A $a3, $y) :mixed{}
  public function d(AnyArray $a = null, $x, $y=0) :mixed{}
}
class C implements I {
  public function a($x=0, AnyArray $a = null) :mixed{}
  public function b(AnyArray $a = vec[], $x=0) :mixed{}
  public function c($x, A $a1=null, A $a2, A $a3=null, $y, $z=0) :mixed{}
  public function d(AnyArray $a = null, $x, $y) :mixed{}
}
<<__EntryPoint>> function main(): void {
print "Test begin\n";
print "Test end\n";
}
