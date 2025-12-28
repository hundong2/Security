# install rust 

# 러스트 컴파일 시작하기
macOS에서 Rust 설치·프로젝트 생성·컴파일/실행/디버깅을 간단히 안내합니다.
## 예시 파일
[Hello World (Rust Book, main.rs)](https://raw.githubusercontent.com/rust-lang/book/main/listings/ch01-02-hello-world/listing-01-01/src/main.rs)
## 답변
- 도구 설치(macOS 터미널):
  - brew install rustup-init
  - rustup-init    (설치 후 기본 stable 선택)
  - rustc --version && cargo --version
  - VS Code 확장: Rust Analyzer, CodeLLDB 설치
- 프로젝트 생성/빌드/실행:
  - cd /Users/donghun2/workspace/Security
  - cargo new hello-rust
  - cd hello-rust
  - cargo build            (디버그 빌드)
  - cargo run              (빌드+실행)
  - cargo build --release  (최적화 빌드)
- 단일 파일 컴파일:
  - rustc main.rs && ./main
- 포맷/정적분석/테스트:
  - rustup component add rustfmt clippy
  - cargo fmt
  - cargo clippy
  - cargo test
- 디버깅(VS Code):
  - 폴더 열기 → 브레이크포인트 설정 → Run and Debug에서 LLDB로 실행

샘플 코드:
````rust
fn main() {
    println!("Hello, Rust!");
}
````

의존성 추가(선택):
- cargo install cargo-edit
- cargo add anyhow

### 추가 자료
- [Rust Book(공식 문서)](https://doc.rust-lang.org/book/)
- [Cargo(빌드/패키지 매니저) 문서](https://doc.rust-lang.org/cargo/)
- [rustup(툴체인 관리)](https://rustup.rs/)
- [Rust Analyzer 확장](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [CodeLLDB 확장](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

# Rustup .cargo/env 설정과 PATH 적용 방법

## 예시 파일

[Hello World (Rust Book, main.rs)](https://raw.githubusercontent.com/rust-lang/book/main/listings/ch01-02-hello-world/listing-01-01/src/main.rs)
## 답변
- 의미: $HOME/.cargo/bin을 PATH에 넣어야 cargo/rustc 명령을 쓸 수 있습니다. “.”는 POSIX 셸에서 source(현재 셸에 설정 반영)와 동일합니다.
- macOS 기본 셸(zsh) 일시 적용:
  ````sh
  . "$HOME/.cargo/env"   # 또는: source "$HOME/.cargo/env"
  cargo --version
  rustc --version
  ````
- 영구 적용(.zshrc에 추가 후 재시작):
  ````sh
  // filepath: /Users/donghun2/.zshrc
  // ...existing code...
  . "$HOME/.cargo/env"   # Rust/Cargo PATH 설정
  // ...existing code...
  ````
  적용 즉시 반영:
  ````sh
  source ~/.zshrc   # 또는 터미널 재시작/exec zsh
  which cargo       # 예상: cargo
  cargo --version
  ````
- 다른 셸을 쓰는 경우:
  - fish:  
    ````sh
    source "$HOME/.cargo/env.fish"
    ````
  - nushell:  
    ````sh
    source $"($nu.home-path)/.cargo/env.nu"
    ````
- VS Code 통합 터미널: .zshrc를 수정한 뒤 VS Code를 완전히 재시작해야 새 PATH가 반영됩니다.

문제 해결 체크리스트:
- which cargo가 빈 출력이면 .cargo/env가 로드되지 않은 것입니다. .zshrc에 추가했는지 확인하세요.
- PATH에 $HOME/.cargo/bin이 포함되어야 합니다:  
  ````sh
  echo $PATH | tr ':' '\n' | grep '.cargo/bin'
  ````
### 추가 자료
- [Rustup 설치 안내](https://rust-lang.github.io/rustup/installation/)
- [Cargo 설치/시작하기](https://doc.rust-lang.org/cargo/getting-started/installation.html)### 추가 자료
- [Rustup 설치 안내](https://rust-lang.github.io/rustup/installation/)
- [Cargo 설치/시작하기](https://doc.rust-lang.org/cargo/getting-started/installation.html)