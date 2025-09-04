
let find x =
  match
    Array.find_opt
      (fun (y, _) -> x = y)
      [|
        (`Stocks, "XX\nXX");
        (`Bookshelf, "XXX");
        (`Desk_for_reading, "XXXX");
        (`Bed, "XXXX");
        (`Desk_for_writing, "XXX\nX");
        (`Chest, "X");
        (`Locked_chest, "X");
        (`Desk_for_measurement, "XXXX\nX  X");
        (`Altar, "X");
        (`Sample_collection, "XX");
        (`Wooden_bedstead, "X");
        (`Prayer_stool, "XX");
        (`Wash_basin, "XX");
        (`Desk_with_bell, "X\nXXXX");
        (`Table_for_bread, "XXX");
        (`Table_for_meat, "XXXX");
        (`Shelf, "XX");
        (`Fireplace, "X X\n X \nX X");
        (`Cauldron, "XXX\nXXX\nXXX");
      |]
  with
  | Some (_, x) -> x
  | None -> raise Not_found
