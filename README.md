# APEIRON-X: HPM 数学宇宙公理验证器

APEIRON-X 是基于 **TSN 4.0（Topological-Semantic Network）** 的数学物理框架实现的一套：

> **动态逻辑—拓扑—张量联合演化系统**
> （Dynamical Logic–Topology–Tensor Evolution System）

## 项目概述

APEIRON-X 可验证任意公理系统在高维物理化空间中的：
- **一致性**
- **稳定性**
- **演化终态**
- **超限行为**
- **拓扑结构变化**

输出的是：**《存在性证明证书》（Existence Certificate）**

## 技术架构

### 多语言架构（Polyglot Architecture）

| 模块 | 语言 | 说明 |
|------|------|------|
| HPM-DL 编译器 | Rust | Parser/Typechecker/IR |
| 张量演化引擎 | C++ + CUDA | GPU 计算，稀疏张量 |
| 同调分析器 | C++ | GUDHI 绑定 |
| 超限递归器 | Rust | 高可靠性 |
| 诊断层 | Rust / Python | 稳定性预言机 |
| Orchestrator | Rust | 调度、运行时、沙箱 |
| 前端可视化 | TypeScript + React | Web 界面（当前优先 CLI，前端可选后续） |
| 分布式任务管理 | Kubernetes + Rust | GPU 分配 |

## 项目结构

```
APEIRON-X/
├── frontend/              # TypeScript + React
│   ├── hpm-dl-editor/     # HPM-DL 编辑器
│   ├── axiom-graph-editor/# 公理图构建器
│   ├── certificate-viewer/ # 证书查看器
│   └── api-client/         # API 客户端
│
├── compiler/              # Rust
│   ├── parser/            # Parser (nom/pest)
│   ├── typechecker/       # 类型检查器
│   ├── semantic/          # 语义分析器（归纳公理展开器）
│   ├── ir/                # IR Builder
│   ├── codegen/           # Operator Spec Generator
│   ├── packager/          # Artifact Packager
│   └── tests/             # 测试
│
├── runtime/               # Rust
│   ├── orchestrator/      # 调度器
│   ├── grpc-api/          # gRPC API
│   ├── scheduler/         # 任务调度
│   ├── sandbox/           # 沙箱隔离
│   └── monitoring/        # 监控
│
├── solver/                # C++ + CUDA + pybind11
│   ├── tensor-engine/     # 张量演化引擎
│   ├── topology-engine/   # 同调分析器
│   ├── transfinite-engine/# 超限递归器
│   ├── gpu-kernels/       # CUDA 内核
│   ├── bindings/          # Python/Rust bindings
│   └── tests/             # 测试
│
├── artifacts/             # IR, operators, snapshots, certificates
│
├── infra/                 # 基础设施
│   ├── kubernetes/        # K8s 配置
│   ├── docker/            # Docker 配置
│   └── ci-cd/             # CI/CD 配置
│
└── docs/                  # 文档
    ├── HPM-spec/          # HPM 规范
    ├── TSN-mapping/       # TSN 映射
    └── API-spec/          # API 规范
```

## 核心组件

### 1. HPM-DL 编译器
- 将 HPM-DL 转换为 IR
- 生成 Operator Catalog
- 生成 Numeric Operator Specs
- 生成 Job Descriptor
- 安全检查（human_force gates）

### 2. 数学求解引擎
- **Tensor Evolution Engine**: TEBD/TDVP 时间演化，MPS/PEPS 表示
- **Topological Engine**: 使用 GUDHI 计算 Betti 数序列
- **超限递归器**: 处理序数 α 层的展开

### 3. 运行时/调度器
- 启动求解器
- 管理 GPU 资源
- 分布式任务调度
- 沙箱隔离
- 审计与日志链路
- 证书生成

## 构建说明

### Rust 组件
```bash
cargo build --release
```

### C++/CUDA 组件
```bash
cd solver
mkdir build && cd build
cmake ..
make -j$(nproc)
```

### 前端
```bash
cd frontend
npm install
npm run build
```

## 安全机制

- `human_force` gating（危险算符）
- `entropy_blowup` detection
- `anomaly_threshold`
- `topology_surgery_throttle`
- `provenance_tracking`（哈希链）

## 许可证

MIT License

## 贡献

欢迎贡献！请阅读贡献指南。

