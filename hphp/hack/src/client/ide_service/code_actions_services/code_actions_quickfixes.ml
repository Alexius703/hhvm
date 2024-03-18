(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)
open Hh_prelude

let text_edits
    (classish_information :
      Pos.t Classish_positions.classish_information SMap.t)
    (quickfix : Pos.t Quickfix.t) : Code_action_types.edit list =
  match Quickfix.get_edits quickfix with
  | Quickfix.Eager eager_edits ->
    List.map eager_edits ~f:(fun (text, pos) -> Code_action_types.{ pos; text })
  | Quickfix.Classish_end { classish_end_new_text; classish_end_name } ->
    (match SMap.find_opt classish_end_name classish_information with
    | Some Classish_positions.{ classish_end; _ } ->
      [Code_action_types.{ pos = classish_end; text = classish_end_new_text }]
    | None ->
      let () =
        HackEventLogger.invariant_violation_bug
          ~data:classish_end_name
          "Could not find class position for quickfix"
      in
      [])

let convert_quickfix
    path
    (classish_information :
      Pos.t Classish_positions.classish_information SMap.t)
    (quickfix : Pos.t Quickfix.t) : Code_action_types.quickfix =
  let edits =
    lazy
      (Relative_path.Map.singleton
         path
         (text_edits classish_information quickfix))
  in

  Code_action_types.
    { title = Quickfix.get_title quickfix; edits; kind = `Quickfix }

let errors_to_quickfixes
    (ctx : Provider_context.t)
    (entry : Provider_context.entry)
    (errors : Errors.t)
    (path : Relative_path.t)
    (classish_information :
      Pos.t Classish_positions.classish_information SMap.t)
    (selection : Pos.t) : Code_action_types.quickfix list =
  let errors = Errors.get_error_list ~drop_fixmed:false errors in
  let errors_here =
    List.filter errors ~f:(fun e ->
        let e_pos = User_error.get_pos e in
        Pos.contains e_pos selection)
  in
  let quickfixes = List.bind ~f:User_error.quickfixes errors_here in
  let standard_quickfixes =
    List.map quickfixes ~f:(convert_quickfix path classish_information)
  in
  let quickfixes_from_refactors =
    errors_here |> List.bind ~f:(Quickfixes_from_refactors.find ctx entry)
  in
  standard_quickfixes @ quickfixes_from_refactors

let find ~entry pos ctx : Code_action_types.quickfix list =
  let cst = Ast_provider.compute_cst ~ctx ~entry in
  let tree = Provider_context.PositionedSyntaxTree.root cst in

  let classish_information =
    match entry.Provider_context.source_text with
    | Some source_text ->
      Classish_positions.classish_information
        tree
        source_text
        entry.Provider_context.path
    | None -> SMap.empty
  in

  let { Tast_provider.Compute_tast_and_errors.errors; _ } =
    Tast_provider.compute_tast_and_errors_quarantined ~ctx ~entry
  in
  let path = entry.Provider_context.path in
  errors_to_quickfixes ctx entry errors path classish_information pos
