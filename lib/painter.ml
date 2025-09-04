module Splitmix = PRNG.Splitmix.Pure

let width x = Float.(to_int (sqrt (of_int x)))

let paint_room r buf i_name i_str =
  let size = Array.length buf in
  let w = width size in
  let p, _ = Splitmix.int size r in
  let exception Collision in
  let fold_iter x y z = ignore (String.fold_left x y z) in
  let can_insert =
    try
      fold_iter
        (fun (lines, i) ->
          let has_passed_line = i - p + (p mod w) - (lines * w) >= w in
          function
          | '\n' -> (lines + 1, p + ((lines + 1) * w))
          | ' ' -> (lines, i + 1)
          | 'X'
            when i >= Array.length buf
                 || Array.get buf i != `Empty
                 || has_passed_line ->
              raise Collision
          | 'X' -> (lines, i + 1)
          | _ -> invalid_arg "Improperly formatted string")
        (0, p) i_str;
      true
    with Collision -> false
  in
  if can_insert then
    fold_iter
      (fun i -> function
        | '\n' -> p + w
        | ' ' -> i + 1
        | 'X' ->
            Array.set buf i i_name;
            i + 1
        | _ -> assert false)
      p i_str;
  ()

let painted_room_to_str arr =
  String.init (Array.length arr) (fun i ->
      match Array.get arr i with
      | `Empty -> ' '
      | `Stocks -> 'S'
      | `Bookshelf -> 'B'
      | `Desk_for_reading -> 'r'
      | `Bed -> 'b'
      | `Desk_for_writing -> 'w'
      | `Chest -> 'c'
      | `Locked_chest -> 'C'
      | `Desk_for_measurement -> 'm'
      | `Altar -> 'A'
      | `Sample_collection -> 's'
      | `Wooden_bedstead -> 'd'
      | `Prayer_stool -> 'p'
      | `Wash_basin -> 'u'
      | `Desk_with_bell -> '!'
      | `Table_for_bread -> 't'
      | `Table_for_meat -> 'T'
      | `Shelf -> 'h'
      | `Fireplace -> 'f'
      | `Cauldron -> 'o')

let pp_painted_room_str fmt s =
  let l = String.length s in
  let w = width l in
  for i = 0 to (l / w) - 1 do
    Format.fprintf fmt "|%s|@." (String.sub s (i * w) w)
  done

let%test_unit "painting large strings is okay" =
  let buf = Array.make 100 `Empty in
  let r = Splitmix.seed "Testing" in
  paint_room r buf `Chest "XXXXXXXX\nXXXXXXXX"

let paint_content ?(max_attempts = 30) r buf room =
  let max_attempts, r = Splitmix.int max_attempts r in
  let g =
    match room with
    | `Reading_room g | `Financial_room g | `Dormitory g | `Kitchen g -> g
  in
  Seq.take max_attempts g
  |> Seq.fold_left
       (fun r n ->
         let s = Dimensions.find n in
         let r', r = Splitmix.split r in
         paint_room r' buf n s;
         r)
       r
  |> ignore

let%test_unit "we can paint a real world situation" =
  let buf = Array.make 100 `Empty in
  let r = Splitmix.seed "123123" in
  let g = fst (Option.get (Seq.uncons (Content.gen r))) in
  print_endline
    (match g with
    | `Reading_room _ -> "Reading room"
    | `Financial_room _ -> "Financial room"
    | `Dormitory _ -> "Dormitory"
    | `Kitchen _ -> "Kitchen");
  paint_content r buf g;
  Format.printf "%a" pp_painted_room_str (painted_room_to_str buf)
