<?hh

module MLT_B;

<<__EntryPoint>>
function zot(): void {
  include 'module_level_traits_module_a.inc';
  include 'module_level_traits_module_b.inc';
  include 'module_level_traits_module_c.inc';
  include 'module_level_traits_9.inc0';
  include 'module_level_traits_9.inc1';

  $c = new C();
  $c->bar();
}
