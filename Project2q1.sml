(* 1. Meaning of Life *)

fun hitchhiker list = List.exists (fn x => x = 42) list;

hitchhiker [1, 55, 66, 765765, 42];


(* 2. Every Other Element *)

fun everyother [] = []
  | everyother [x] = []
  | everyother (head::tail) = (hd tail)::everyother(tl tail);

everyother [1, 2, 3, 4, 5, 6, 7];


(* Goldbach’s Conjecture*)
fun prime_helper (num, i) =
  case num of
    n => if n = i then true
         else if n mod i = 0 then false
         else prime_helper (num, i + 1);

fun is_prime 0 = false
  | is_prime 1 = false
  | is_prime 2 = true
  | is_prime 3 = true
  | is_prime n = if prime_helper (n, 2) then true else false;

fun are_two_addends_prime (x, y) = 
  if is_prime x andalso is_prime y
    then Int.toString x ^ " + " ^ Int.toString y ^ " is " ^ Int.toString (x + y)
  else if x < y
    then are_two_addends_prime (x + 1, y - 1)
  else "Error could not find primes";

fun goldbach n =
  if n mod 2 <> 0
  then "Error, number is not even"
  else are_two_addends_prime (0, n);

goldbach 30;