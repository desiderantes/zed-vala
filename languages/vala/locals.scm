; locals.scm
; Taken from https://github.com/vala-lang/tree-sitter-vala
; Licensed under the LGPL 2.1
; This program is free software: you can redistribute it and/or modify
; it under the terms of the GNU Lesser General Public License as published by
; the Free Software Foundation, version 2.1 of the License
;
; This program is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU Lesser General Public License for more details.
;
; You should have received a copy of the GNU Lesser General Public License
; along with this program.  If not, see <http://www.gnu.org/licenses/>.

[
 (method_declaration)
 (local_function_declaration)
 (signal_declaration)
 (block)
] @local.scope

(parameter (identifier) @local.definition)
(local_declaration (assignment (identifier) @local.definition))
(local_function_declaration (identifier) @local.definition)

(member_access_expression . (identifier) @local.reference)
