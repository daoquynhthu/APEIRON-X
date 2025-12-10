# APEIRON-X 项目进度书（2025-12-10）

## 已完成
- 依赖与构建
  - 统一 pest 版本（2.8.4）并启用 derive；编译器不再因依赖冲突报错。
  - runtime 使用 `protoc-bin-vendored`，build.rs 自动设置 `PROTOC`，避免系统 protoc 依赖。
  - `cargo vendor` 已完成，`.cargo/config.toml` 指向本地 `vendor`，支持离线构建。
- 编译器（CLI 优先）
  - pest 覆盖 HPM-DL 主语法；支持 ASCII 同义词（dot/otimes/tensor/x，dagger/transpose）。
  - 解析生成 AST（axiom/operator/state/constraint），表达式已结构化解析。
  - Operator 体解析矩阵/张量网/gpu_kernel，IR 输出带实现的算符（DenseMatrix/ TensorNetwork / CUDAKernel），不再是占位空 JSON。
  - 基础类型校验：矩阵尺寸与数据长度匹配；初态维度非空。
  - 安全报告结构可序列化，CLI `check` 输出 JSON。
- 文档与配置
  - CLI 工作流与语法说明（HPM-DL 设计稿）已更新。
  - 前端标记为可选后续层，当前主推 CLI/库工作流。

## 待处理 / 计划
- 解析与类型
  - 表达式从 Raw 转结构化 AST，完成类型/形状检查（算符-状态维度、bond 对齐）。
  - 解析 operator 体的矩阵/张量网数值并校验维度一致性。
- IR 与工件
  - 生成真实 TensorSpec/BondSpec 数据；operators/job 输出完整数值和拓扑信息。
  - 支持 CBOR/MessagePack/JSON-L 工件格式。
- 安全检查
  - 实现 entropy/anomaly/topology 阈值计算与报告。
  - human_force gate 更细粒度的标记与汇总。
- 运行时/求解器
  - runtime API 与调度、证书生成的最小可用实现。
  - solver 数值核心与 GUDHI 集成（后续阶段）。
- CLI/UX
  - 语法/类型错误的行列高亮与 codespan 报告。
  - 增强 CLI 参数（输出格式、严格/宽松模式）。

## 风险与依赖
- 网络访问 crates.io 可能因 SSL/网络波动失败；可使用镜像或预拉依赖。
- 运行时需要 C++/CUDA 环境和 protobuf 工具链（已用 vendored protoc 缓解）。

## 短期优先级（建议）
1) 表达式结构化解析 + 类型/形状检查（消除 Raw）。  
2) Operator 体数值解析 → 完整 IR/工件。  
3) 错误报告体验（行列、代码框、高亮）。  
4) runtime gRPC stub 打通最小 submit/echo 流程。

