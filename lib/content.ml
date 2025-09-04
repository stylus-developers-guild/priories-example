module Splitmix = PRNG.Splitmix.Pure

module Gen = struct
  [@@@warning "-8"]

  let pick r sum items =
    let v, r = Splitmix.int sum r in
    let rec loop i acc =
      let x, chance = Array.get items i in
      if v < acc + chance then (x, r) else loop (i + 1) (acc + chance)
    in
    loop 0 0

  let gen items =
    let sum = Array.fold_left (fun x (_, y) -> x + y) 0 items in
    Seq.unfold (fun current_r ->
        let v, new_r = pick current_r sum items in
        Some (v, new_r))

  let gen_reading =
    gen
      [|
        (`Stocks, 10);
        (`Bookshelf, 40);
        (`Desk_for_reading, 20);
        (`Bed, 10);
        (`Desk_for_writing, 20);
      |]

  let gen_financial_room =
    gen
      [|
        (`Chest, 10);
        (`Bed, 10);
        (`Locked_chest, 20);
        (`Desk_for_reading, 10);
        (`Desk_for_writing, 10);
        (`Desk_for_measurement, 10);
        (`Altar, 20);
        (`Sample_collection, 10);
      |]

  let gen_dormitory =
    gen
      [|
        (`Bed, 50);
        (`Prayer_stool, 10);
        (`Wash_basin, 10);
        (`Desk_with_bell, 30);
      |]

  let gen_kitchen =
    gen
      [|
        (`Fireplace, 30);
        (`Cauldron, 30);
        (`Table_for_bread, 20);
        (`Table_for_meat, 5);
        (`Shelf, 10);
      |]
end

let gen_room r =
  let x, r =
    Gen.pick r 4
      [|
        (`Reading_room, 1); (`Financial_room, 1); (`Dormitory, 1); (`Kitchen, 1);
      |]
  in
  let r', _ = Splitmix.split r in
  match x with
  | `Reading_room -> `Reading_room (Gen.gen_reading r')
  | `Financial_room -> `Financial_room (Gen.gen_financial_room r')
  | `Dormitory -> `Dormitory (Gen.gen_dormitory r')
  | `Kitchen -> `Kitchen (Gen.gen_kitchen r')

let gen =
  Seq.unfold (fun r ->
      let r, r' = Splitmix.split r in
      Some (gen_room r, r'))
