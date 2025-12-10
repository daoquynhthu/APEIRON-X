# APEIRON-X 项目结构总览

## 项目架构

APEIRON-X 采用多语言架构（Polyglot Architecture），各组件使用最适合的语言实现。

## 目录结构

```
APEIRON-X/
│
├── Cargo.toml              # Rust workspace 配置
├── README.md               # 项目说明文档
├── .gitignore              # Git 忽略文件
├── PROJECT_STRUCTURE.md    # 本文件
│
├── compiler/               # HPM-DL 编译器 (Rust)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # 编译器入口
│       ├── parser/         # 解析器模块
│       │   ├── mod.rs
│       │   ├── lexer.rs    # 词法分析器
│       │   ├── grammar.rs  # 语法定义
│       │   └── ast.rs      # AST 定义
│       ├── typechecker/    # 类型检查器
│       ├── semantic/       # 语义分析器（归纳公理展开）
│       ├── ir/             # 中间表示 (IR)
│       ├── codegen/        # 代码生成（Operator Spec, Job Descriptor）
│       ├── packager/       # 工件打包（JSON-L, CBOR, MessagePack）
│       └── safety/         # 安全检查（human_force gates）
│
├── runtime/                # 运行时/调度器 (Rust)
│   ├── Cargo.toml
│   ├── build.rs            # gRPC 代码生成
│   ├── proto/
│   │   └── runtime.proto   # gRPC 服务定义
│   └── src/
│       ├── main.rs         # 运行时入口
│       ├── orchestrator/   # 主调度器
│       ├── grpc_api/       # gRPC API 实现
│       ├── scheduler/      # 任务调度器
│       ├── sandbox/        # 沙箱隔离
│       └── monitoring/     # 监控和审计
│
├── solver/                 # 数学求解引擎 (C++ + CUDA)
│   ├── CMakeLists.txt      # CMake 构建配置
│   ├── include/
│   │   └── apeiron_solver/
│   │       ├── tensor_engine.h      # 张量演化引擎接口
│   │       ├── topology_engine.h    # 同调分析器接口
│   │       └── transfinite_engine.h # 超限递归器接口
│   ├── src/
│   │   ├── tensor_engine/  # 张量演化引擎实现
│   │   │   ├── tensor_engine.cpp
│   │   │   ├── mps.cpp     # MPS 实现
│   │   │   ├── peps.cpp    # PEPS 实现
│   │   │   ├── tebd.cpp    # TEBD 算法
│   │   │   └── tdvp.cpp    # TDVP 算法
│   │   ├── topology_engine/ # 同调分析器
│   │   │   ├── cohomology.cpp
│   │   │   └── betti.cpp
│   │   ├── transfinite_engine/ # 超限递归器
│   │   │   └── transfinite.cpp
│   │   └── gpu_kernels/    # CUDA 内核
│   │       ├── tensor_ops.cu
│   │       ├── evolution.cu
│   │       └── truncation.cu
│   └── bindings/
│       └── python_bindings.cpp # Python 绑定
│
├── frontend/               # 前端界面 (TypeScript + React + Next.js)
│   ├── package.json
│   ├── tsconfig.json
│   ├── next.config.js
│   ├── tailwind.config.js
│   └── src/
│       └── app/
│           ├── layout.tsx  # 根布局
│           ├── page.tsx     # 首页
│           └── globals.css  # 全局样式
│           # TODO: 添加以下页面
│           # - hpm-dl-editor/     # HPM-DL 编辑器
│           # - axiom-graph-editor/ # 公理图构建器
│           # - certificate-viewer/ # 证书查看器
│           # - api-client/         # API 客户端
│
├── artifacts/              # 工件存储目录
│   └── .gitkeep
│   # 存储内容：
│   # - IR (JSON-L 格式)
│   # - Operators (CBOR/MessagePack)
│   # - Initial states
│   # - Intermediate snapshots
│   # - Final certificates
│   # - Topology graphs
│
├── infra/                  # 基础设施配置
│   ├── docker/
│   │   ├── Dockerfile.compiler  # 编译器镜像
│   │   ├── Dockerfile.runtime   # 运行时镜像
│   │   └── Dockerfile.solver    # 求解器镜像
│   └── kubernetes/
│       ├── runtime-deployment.yaml  # 运行时部署
│       └── solver-daemonset.yaml    # 求解器 DaemonSet
│
└── docs/                   # 文档目录
    ├── HPM-spec/          # HPM 规范文档
    ├── API-spec/          # API 规范文档
    └── TSN-mapping/       # TSN 映射文档（待添加）
```

## 核心模块说明

### 1. Compiler (编译器)
- **语言**: Rust
- **功能**: 
  - 解析 HPM-DL 源代码
  - 类型检查和语义分析
  - 生成 IR
  - 生成 Operator Catalog 和 Job Descriptor
  - 安全检查（human_force gates）
  - 工件打包和存储

### 2. Runtime (运行时)
- **语言**: Rust
- **功能**:
  - 任务调度和资源管理
  - GPU 资源分配
  - 分布式任务调度
  - 沙箱隔离
  - 监控和审计
  - 证书生成
  - gRPC API 服务

### 3. Solver (求解器)
- **语言**: C++ + CUDA + Python bindings
- **功能**:
  - **Tensor Evolution Engine**: 
    - MPS/PEPS 表示
    - TEBD/TDVP 时间演化
    - 多 GPU 计算
    - 张量截断（SVD, QR, RG）
  - **Topology Engine**:
    - Betti 数序列计算
    - 拓扑生成元识别
    - 拓扑手术触发检测
  - **Transfinite Engine**:
    - 序数 α 层展开
    - 演化时间重置
    - 不动点检测

### 4. Frontend (前端)
- **语言**: TypeScript + React + Next.js
- **功能**:
  - HPM-DL 编辑器（Monaco Editor）
  - 公理图构建器（React Flow）
  - 证书查看器
  - 拓扑图可视化（Three.js）
  - API 客户端

## 数据流

```
HPM-DL Source
    ↓
Compiler (Parser → Typechecker → Semantic → IR → Codegen)
    ↓
IR + Operator Catalog + Job Descriptor
    ↓
Artifacts Storage (S3/MinIO)
    ↓
Runtime (Orchestrator → Scheduler)
    ↓
Solver (Tensor Engine + Topology Engine + Transfinite Engine)
    ↓
Certificate + State Snapshots + Topology Signature
    ↓
Frontend (Visualization)
```

## 构建说明

### Rust 组件
```bash
# 构建所有 Rust 组件
cargo build --release

# 构建特定组件
cargo build --release --package apeiron-compiler
cargo build --release --package apeiron-runtime
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

## 部署说明

### Docker
```bash
# 构建镜像
docker build -f infra/docker/Dockerfile.compiler -t apeiron/compiler .
docker build -f infra/docker/Dockerfile.runtime -t apeiron/runtime .
docker build -f infra/docker/Dockerfile.solver -t apeiron/solver .
```

### Kubernetes
```bash
# 部署运行时
kubectl apply -f infra/kubernetes/runtime-deployment.yaml

# 部署求解器（需要 GPU 节点）
kubectl apply -f infra/kubernetes/solver-daemonset.yaml
```

## 下一步开发任务

1. **Compiler**:
   - [ ] 实现 HPM-DL 词法分析器
   - [ ] 实现 HPM-DL 语法解析器
   - [ ] 实现类型检查器
   - [ ] 实现语义分析器（归纳公理展开）
   - [ ] 实现 IR 生成器
   - [ ] 实现代码生成器

2. **Runtime**:
   - [ ] 实现 gRPC 服务
   - [ ] 实现任务调度器
   - [ ] 实现 GPU 资源管理器
   - [ ] 实现沙箱管理器
   - [ ] 实现监控系统

3. **Solver**:
   - [ ] 实现 MPS/PEPS 数据结构
   - [ ] 实现 TEBD/TDVP 算法
   - [ ] 实现 CUDA 内核
   - [ ] 集成 GUDHI 库
   - [ ] 实现超限递归器

4. **Frontend**:
   - [ ] 实现 HPM-DL 编辑器
   - [ ] 实现公理图构建器
   - [ ] 实现证书查看器
   - [ ] 实现拓扑可视化

5. **Infrastructure**:
   - [ ] 完善 CI/CD 配置
   - [ ] 添加监控和告警
   - [ ] 实现日志聚合

