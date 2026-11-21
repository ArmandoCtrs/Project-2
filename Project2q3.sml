val chinese : string list =
  ["ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi"];

val english : string list =
  ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

fun indexOf (word: string, lst: string list): int =
  let
    fun go (_, []) = ~1
      | go (i, x :: xs) =
          if x = word then i else go (i + 1, xs)
  in
    go (0, lst)
  end

fun position (word : string, candidates : string list, pos: int) : int =
  indexOf (word, candidates);

fun lookupChinese (word : string) : int =
  position (word, chinese, 0);

fun lookupEnglish (word : string) : int =
  position (word, english, 0);

fun translate (word : string) : int =
  case Int.compare (lookupChinese(word), ~1) of
    GREATER => lookupChinese(word)
  | EQUAL => case Int.compare (lookupEnglish(word), ~1) of
            GREATER => lookupEnglish(word)
          | EQUAL => ~1
          | LESS => ~1
  | LESS => ~1;

fun translateAndBuild [] = []
  | translateAndBuild (head::tail) = (if translate head > ~1 then translate head else ~1)::translateAndBuild tail;

(*
fun printTranslation (numbers : int list) : unit =
  raise Fail "TODO"

fun printAddition (numbers : int list) : unit =
  raise Fail "TODO"

fun printMultiplication (numbers : int list) : unit =
  raise Fail "TODO"
*)

fun go input = 
  let
    val list_built = translateAndBuild input
    val list_built_and_filtered = List.filter (fn x => x > 0) list_built
  in
    list_built_and_filtered
  end;

go ["yi", "nine", "josh", "ba", "test", "ten"];