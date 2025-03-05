#!/bin/bash

# 设置国内镜像
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 下载并运行rustup安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
