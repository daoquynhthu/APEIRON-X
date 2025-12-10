//! Grammar definitions for HPM-DL
//!
//! Pest grammar covering主要结构：axiom/operator/state/constraint、算符体、表达式、拓扑约束、标签/安全标记等。
//! 解析结果当前仅用于语法检查，AST 构造在后续实现。

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
file            = { SOI ~ stmt* ~ EOI }

stmt            = _{ axiom | operator | state_decl | constraint }

axiom           = { "axiom" ~ ws1 ~ ident ~ ":" ~ expr ~ tags? ~ safety? }
operator        = { "operator" ~ ws1 ~ ident ~ ":" ~ op_kind ~ op_body? }
state_decl      = { "state" ~ ws1 ~ ident ~ ":" ~ state_repr ~ dims ~ hamiltonian_ref? ~ topo_constraints? }
hamiltonian_ref = { ws1 ~ "hamiltonian" ~ ws0 ~ "=" ~ ws0 ~ ident }
constraint      = { "constraint" ~ ws1 ~ ident ~ ":" ~ constraint_kind ~ expr }

op_kind         = { "hamiltonian" | "dissipative" | "topological_mutation" | "entropy_flow" }

op_body         = { ws0 ~ "{" ~ ws0 ~ op_item_list? ~ ws0 ~ "}" }
op_item_list    = { op_item ~ (ws0 ~ "," ~ ws0 ~ op_item)* ~ ws0 ~ comma_trailing? }
op_item         = { matrix_spec | tensor_net_spec | gpu_kernel_spec }

matrix_spec     = { "matrix" ~ ws0 ~ "(" ~ ws0 ~ int ~ ws0 ~ "," ~ ws0 ~ int ~ ws0 ~ "," ~ ws0 ~ storage ~ ws0 ~ ")" ~ ws0 ~ "=" ~ ws0 ~ matrix_lit }
storage         = { "dense" | "sparse" }
matrix_lit      = { "[" ~ ws0 ~ row_list ~ ws0 ~ "]" }
row_list        = { row ~ (ws0 ~ "," ~ ws0 ~ row)* }
row             = { "[" ~ ws0 ~ number_list ~ ws0 ~ "]" }
number_list     = { number ~ (ws0 ~ "," ~ ws0 ~ number)* }

tensor_net_spec = { "tensor_net" ~ ws0 ~ "(" ~ ws0 ~ net_kind ~ ws0 ~ "," ~ ws0 ~ bond_dims ~ (ws0 ~ "," ~ ws0 ~ trunc_spec)? ~ ws0 ~ ")" }
net_kind        = { "mps" | "peps" | "mera" }
bond_dims       = { "[" ~ ws0 ~ int ~ (ws0 ~ "," ~ ws0 ~ int)* ~ ws0 ~ "]" }
trunc_spec      = { "trunc" ~ ws0 ~ "(" ~ ws0 ~ trunc_method ~ ws0 ~ "," ~ ws0 ~ int ~ ws0 ~ "," ~ ws0 ~ float ~ ws0 ~ ")" }
trunc_method    = { "svd" | "qr" | "rg" }

gpu_kernel_spec = { "cuda_kernel" ~ ws0 ~ "(" ~ ws0 ~ string ~ ws0 ~ ")" }

state_repr      = { "mps" | "peps" | "dense" | "sparse" }
dims            = { ws1 ~ "[" ~ ws0 ~ int ~ (ws0 ~ "," ~ ws0 ~ int)* ~ ws0 ~ "]" }

topo_constraints= { ws1 ~ "topology" ~ ws0 ~ "{" ~ ws0 ~ topo_items? ~ ws0 ~ "}" }
topo_items      = { topo_item ~ (ws0 ~ "," ~ ws0 ~ topo_item)* ~ ws0 ~ comma_trailing? }
topo_item       = { betti | generators }
betti           = { "betti" ~ ws0 ~ "=" ~ ws0 ~ "[" ~ ws0 ~ int ~ (ws0 ~ "," ~ ws0 ~ int)* ~ ws0 ~ "]" }
generators      = { "generators" ~ ws0 ~ "=" ~ ws0 ~ "[" ~ ws0 ~ generator ~ (ws0 ~ "," ~ ws0 ~ generator)* ~ ws0 ~ "]" }
generator       = { "(" ~ ws0 ~ "dim" ~ ws0 ~ "=" ~ ws0 ~ int ~ ws0 ~ "," ~ ws0 ~ "repr" ~ ws0 ~ "=" ~ ws0 ~ string ~ ws0 ~ ")" }

constraint_kind = { "entropy_bound" | "stability_threshold" | "topology_surgery_threshold" | "human_force_gate" }

tags            = { ws1 ~ "tags" ~ ws0 ~ "[" ~ ws0 ~ ident_list ~ ws0 ~ "]" }
ident_list      = { ident ~ (ws0 ~ "," ~ ws0 ~ ident)* }

safety          = { ws1 ~ "safety" ~ ws0 ~ safety_level }
safety_level    = { "safe" | "requires_human_force" | "dangerous" }

// Expressions (simplified precedence: unary > mul/div > add/sub)
expr            = { add_expr }
add_expr        = { mul_expr ~ (ws0 ~ add_op ~ ws0 ~ mul_expr)* }
add_op          = { "+" | "-" }
mul_expr        = { unary_expr ~ (ws0 ~ mul_op ~ ws0 ~ unary_expr)* }
mul_op          = { "*" | "/" | "·" | "⊗" | "dot" | "otimes" | "tensor" | "x" }
unary_expr      = { unary_op* ~ primary }
unary_op        = { "-" | "†" | "dagger" | "T" | "transpose" | "conj" | "trace" }
primary         = { number | ident | call | grouped }
call            = { ident ~ ws0 ~ "(" ~ ws0 ~ arg_list? ~ ws0 ~ ")" }
arg_list        = { expr ~ (ws0 ~ "," ~ ws0 ~ expr)* }
grouped         = { "(" ~ ws0 ~ expr ~ ws0 ~ ")" }

ident           = @{ (ASCII_ALPHANUMERIC | "_")+ }
int             = @{ ASCII_DIGIT+ }
float           = @{
      ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT* ~ exponent?
    | "." ~ ASCII_DIGIT+ ~ exponent?
    | ASCII_DIGIT+ ~ exponent
}
exponent        = { ("e"|"E") ~ ("+"|"-")? ~ ASCII_DIGIT+ }
number          = _{ float | int }

string          = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }

comma_trailing  = _{ ","? }

// Whitespace & comments
line_comment    = _{ "//" ~ (!NEWLINE ~ ANY)* }
ws0             = _{ ( " " | "\t" | NEWLINE | line_comment )* }
ws1             = _{ ( " " | "\t" | NEWLINE | line_comment )+ }
NEWLINE         = _{ "\n" | "\r\n" }
"#]
pub struct HpmDlParser;

