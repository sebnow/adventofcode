open Core

let is_digit = function '0' .. '9' -> true | _ -> false
let parse_digit c = int_of_char c - int_of_char '0'

let part1 input =
  let extract s =
    let find_digit s = List.find ~f:is_digit s |> Option.map ~f:parse_digit in
    let s' = String.to_list s in
    match (s' |> find_digit, List.rev s' |> find_digit) with
    | Some f, Some l -> (f * 10) + l
    | _ -> 0
  in
  String.split_on_chars input ~on:[ '\n' ]
  |> List.map ~f:extract
  |> List.fold_left ~f:( + ) ~init:0

let part2 _input = ""

let () =
  let input = In_channel.read_all "../../input/2023/day01.txt" in
  part1 input |> Printf.printf "Part 1: %d\n";
  part2 input |> Printf.printf "Part 2: %s\n"
