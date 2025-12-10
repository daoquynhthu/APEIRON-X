# HPM-DL 语言设计稿（CLI 优先）

> 目标：以“Coq 式 CLI/库工作流”为首要形态，提供可解析、可类型检查、可生成 IR 的描述语言，用于驱动求解器与运行时；前端为可选后续。

## 设计目标
- 可形式化描述公理、算符、初态、约束。
- 可生成结构化 IR（JSON-L），下游转换为算符工件（CBOR/MessagePack）与任务描述（Job Descriptor）。
- 可执行静态安全检查（human_force gating、熵/奇点阈值等）。
- 语法紧凑、便于 CLI/编辑器支持。

## 顶层结构
一个 HPM-DL 程序由四个部分组成：
```text
program ::= axioms* operators* initial_state? constraints*
```

### Axiom
```text
axiom ::= "axiom" IDENT ":" expr ("tags" "[" tag_list "]")? ("safety" safety_level)?
safety_level ::= "safe" | "requires_human_force" | "dangerous"
```

### Operator
```text
operator ::= "operator" IDENT ":" op_kind op_body
op_kind  ::= "hamiltonian" | "dissipative" | "topological_mutation" | "entropy_flow"

op_body  ::= "{" (matrix_spec | tensor_net_spec | gpu_kernel_spec)+ "}"
matrix_spec      ::= "matrix" "(" rows "," cols "," storage ")" "=" matrix_literal
storage          ::= "dense" | "sparse"
tensor_net_spec  ::= "tensor_net" "(" net_kind "," bond_dims "," trunc_spec? ")"
net_kind         ::= "mps" | "peps" | "mera"
trunc_spec       ::= "trunc" "(" method "," max_bond_dim "," tol ")"
method           ::= "svd" | "qr" | "rg"
gpu_kernel_spec  ::= "cuda_kernel" "(" STRING ")"
```

### Initial State
```text
initial_state ::= "state" IDENT ":" state_repr dims ("hamiltonian" "=" IDENT)? topo_constraints?
state_repr ::= "mps" | "peps" | "dense" | "sparse"
dims       ::= "[" INT ("," INT)* "]"
topo_constraints ::= "topology" "{" betti? generators? "}"
betti      ::= "betti" "=" "[" INT ("," INT)* "]"
generators ::= "generators" "=" "[" generator ("," generator)* "]"
generator  ::= "(" "dim" "=" INT "," "repr" "=" STRING ")"
```

### Constraint
```text
constraint ::= "constraint" IDENT ":" constraint_kind expr
constraint_kind ::= "entropy_bound" | "stability_threshold" | "topology_surgery_threshold" | "human_force_gate"
```

## 表达式 (expr) 与运算
```text
expr ::= literal
       | IDENT
       | expr binop expr
       | unop expr
       | IDENT "(" arg_list? ")"

binop ::= "+" | "-" | "*" | "/" | "^" | "·" | "⊗" | "dot" | "otimes" | "tensor" | "x"
unop  ::= "-" | "†" | "dagger" | "T" | "transpose" | "conj" | "trace"
arg_list ::= expr ("," expr)*

literal ::= INT | FLOAT
```

## 类型与类型规则（概要）
- 标量：`int`, `float`, `complex`, `bool`
- 张量：`tensor<shape=[d1,...,dn]>`
- 算符：`operator<kind=hamiltonian|dissipative|... , target=cpu|gpu>`
- 状态：`state<mps|peps|dense|sparse>`
- 约束：`constraint<kind>`

主要检查：
- 算符与状态维度匹配（形状、bond dim）。
- 熵/奇点约束与相应算符/状态的存在性。
- `human_force` 标记的算符/约束需明确标注并在安全报告中列出。

## 安全与治理（静态检查）
- `human_force_gate`：任何涉及危险算符需标注；编译器输出安全报告。
- `entropy_blowup`：对演化参数、算符范数、截断策略做静态阈值预估。
- `topology_surgery_throttle`：对拓扑突变触发频率/阈值做静态检查。
- `anomaly_threshold`：对异常指标（如散度/曲率阈值）做静态检查。

## 工件与格式
- IR：JSON-L（program/ir_program）
- Operators：CBOR 或 MessagePack（数值算符）
- Tensors：`.npy`
- Topology graphs：GraphML/JSON

## CLI 工作流（MVP）
```bash
# 解析 + 类型检查 + 语义展开 + 生成 IR/工件
apeiron-compiler compile input.hpmdl \
  --out-ir artifacts/program.jsonl \
  --out-ops artifacts/operators.cbor \
  --out-job artifacts/job.json

# （可选）安全报告
apeiron-compiler check input.hpmdl --report artifacts/safety.json

# 运行时提交（CLI 或 gRPC）
apeiron-runtime submit --job artifacts/job.json
```

## 示例（简化）
```text
axiom A1: H(t) = H0 + λ * V(t) tags [logic, hamiltonian] safety requires_human_force

operator H0: hamiltonian {
  matrix(2,2,dense) = [[0,1],[1,0]]
}

operator L1: dissipative {
  matrix(2,2,dense) = [[0,0],[0,γ]]
}

state psi0: mps [2,2,2]
  hamiltonian = H0
  topology { betti = [1,0]; generators = [(dim=1, repr=\"loop1\")] }

constraint c1: entropy_bound S(t) < 10.0
constraint c2: human_force_gate requires_review
```

## 后续细化路线
- 补充正式 BNF/EBNF 与词法规范。
- 完整类型系统与形状推断规则（含张量网络 bond 对齐）。
- 语义阶段：公理归纳展开、算符 catalog 构建、Job Descriptor 生成。
- 安全模型：量化阈值配置（entropy/anomaly/topology），人类批准流。
- CLI UX：错误定位（codespan/reporting）、JSON/表格化安全报告输出。

