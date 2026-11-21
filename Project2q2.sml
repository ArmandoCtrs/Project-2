fun intToBoolHelper 0 = false
  | intToBoolHelper 1 = true;

fun intToBool [] = []
  | intToBool (head::tail) = (intToBoolHelper head)::(intToBool tail);

fun boolToIntHelper false = 0
  | boolToIntHelper true = 1;

fun boolToInt []= []
  | boolToInt (head::tail) = (boolToIntHelper head)::(boolToInt tail);


fun doBinaryAddition ([], [], carry) = finishBinaryAdd ([], carry)
  | doBinaryAddition ([], bits2, carry) = finishBinaryAdd (bits2, carry)
  | doBinaryAddition (bits1, [], carry) = finishBinaryAdd (bits1, carry)
  | doBinaryAddition (x::xs, y::ys, carry) =
      let
        val sum = (x <> y) <> carry;
        val newCarry = (x andalso y) orelse (carry andalso (x <> y));
      in
        sum :: doBinaryAddition (xs, ys, newCarry)
      end

and finishBinaryAdd ([], carry) = if carry then [true] else []
  | finishBinaryAdd (x::xs, carry) =
      (x <> carry) :: finishBinaryAdd (xs, x andalso carry);


fun binaryAddition (bits1, bits2) =
  let
    val bits1 = rev bits1;
    val bits2 = rev bits2;
  
    val bits1_bool = intToBool bits1;
    val bits2_bool = intToBool bits2;
  
    val bool_added = doBinaryAddition (bits1_bool, bits2_bool, false);
    val int_added = boolToInt bool_added;
    val int_added_complete = rev int_added;
  in
    int_added_complete
  end;


fun invertBitHelper false = true
  | invertBitHelper true = false;

fun invertBits [] = []
  | invertBits (head::tail) = (invertBitHelper head)::(invertBits tail);

fun padRight (xs, n, x) =
    xs @ List.tabulate (Int.max (0, n - length xs), fn _ => x);

fun binarySubtraction (bits1, bits2) =
  let
    val bits1 = rev bits1;
    val bits2 = rev bits2;
  
    val bits1_bool = intToBool bits1;
    val bits2_bool = intToBool bits2;
  
    val bits1_bool_padded =
      if length bits1_bool < length bits2_bool
      then padRight (bits1_bool, length bits2_bool, false)
      else bits1_bool;
  
    val bits2_bool_padded =
      if length bits2_bool < length bits1_bool
      then padRight (bits2_bool, length bits1_bool, false)
      else bits2_bool;
  
  val bits2_bool_padded_and_inverted = invertBits bits2_bool_padded;
  
    val bool_added = doBinaryAddition (bits1_bool_padded, bits2_bool_padded_and_inverted, true);
    val bool_added_truncated = List.take (bool_added, length bool_added - 1);
  
    val int_added = boolToInt bool_added_truncated;
    val int_added_complete = rev int_added;
  in
    int_added_complete
  end;

val result = binaryAddition ([1, 1, 1, 1], [1]);
val result2 = binarySubtraction ([1, 1, 1, 1], [1]);