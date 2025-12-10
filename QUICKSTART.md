# APEIRON-X 快速启动指南

## 前置要求

### Rust 组件
- Rust 1.75+ (使用 `rustup` 安装)
- Cargo (随 Rust 一起安装)

### C++/CUDA 组件
- CMake 3.20+
- CUDA Toolkit 12.0+
- C++20 编译器 (GCC 11+ 或 Clang 14+)
- Python 3.10+ (用于 Python bindings)
- pybind11

### 前端
- Node.js 18+
- npm 或 yarn

## 快速开始

### 1. 克隆项目（如果从仓库克隆）
```bash
git clone <repository-url>
cd APEIRON-X
```

### 2. 构建 Rust 组件
```bash
# 构建所有 Rust 组件
cargo build

# 或者构建特定组件
cargo build --package apeiron-compiler
cargo build --package apeiron-runtime
```

### 3. 构建 C++/CUDA 求解器
```bash
cd solver
mkdir build && cd build
cmake ..
make -j$(nproc)
```

### 4. 启动前端开发服务器
```bash
cd frontend
npm install
npm run dev
```

访问 http://localhost:3000 查看前端界面。

## 开发工作流

### 编译器开发
```bash
cd compiler
cargo run -- <input.hpmdl> -o <output.ir>
```

### 运行时开发
```bash
cd runtime
cargo run
```

运行时将在 `localhost:50051` 启动 gRPC 服务。

### 求解器开发
```bash
cd solver/build
python3 -c "import apeiron_solver_py; print('Solver loaded successfully')"
```

## 测试

### Rust 测试
```bash
cargo test
```

### C++ 测试
```bash
cd solver/build
ctest
```

## 文档

查看详细文档：
- `README.md` - 项目概述
- `PROJECT_STRUCTURE.md` - 项目结构说明
- `docs/` - 详细技术文档

## 常见问题

### Rust 编译错误
如果遇到依赖问题，尝试：
```bash
cargo clean
cargo update
```

### CUDA 编译错误
确保 CUDA 路径正确：
```bash
export CUDA_HOME=/usr/local/cuda
export PATH=$CUDA_HOME/bin:$PATH
export LD_LIBRARY_PATH=$CUDA_HOME/lib64:$LD_LIBRARY_PATH
```

### 前端错误
```bash
cd frontend
rm -rf node_modules .next
npm install
```

