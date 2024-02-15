<?hh
/* Prototype  : string money_format  ( string $format  , float $number  )
 * Description: Formats a number as a currency string
 * Source code: ext/standard/string.c
*/

// ===========================================================================================
// = We do not test for exact return-values, as those might be different between OS-versions =
// ===========================================================================================

//defining a couple of sample classes
class class_no_tostring  {
}

class class_with_tostring  {
  public function __toString() :mixed{
    return "  sample object  ";
  }
}

<<__EntryPoint>> function main(): void {
echo "*** Testing money_format() function: with unexpected inputs for 'number' argument ***\n";


//getting the resource
$file_handle = fopen(__FILE__, "r");

// array with different values for $number
$numbers =  vec[
          // float values
          10.5,
          -20.5,
          10.1234567e10,
];

// loop through with each element of the $numbers array to test money_format() function
$count = 1;
$format = '%14#8.2i';

foreach($numbers as $number) {
  echo "-- Iteration $count --\n";
  try { echo gettype(money_format($format, $number))."\n"; } catch (Exception $e) { echo "\n".'Warning: '.$e->getMessage().' in '.__FILE__.' on line '.__LINE__."\n"; }
  $count ++;
}

// close the file handle
fclose($file_handle);
echo "===Done===";
}
