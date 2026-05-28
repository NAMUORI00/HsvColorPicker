# HSV Color Picker

xcap 화면 캡처와 HSV 색상 필터링으로 객체를 실시간 탐지하는 Rust 데스크톱 앱입니다.

> Python 레거시 버전: [`python` 브랜치](https://github.com/NAMUORI00/HsvColorPicker/tree/python) · 태그 [`v0.1.0-python`](https://github.com/NAMUORI00/HsvColorPicker/releases/tag/v0.1.0-python)

## Features

- OpenCV/PIL/numpy 없이 HSV 변환·마스킹·팽창·윤곽선·바운딩 박스를 순수 Rust로 구현
- **egui** ImGui 스타일 UI — 이미지 메인 + 토글 가능한 플로팅 툴 패널
- **xcap** 기반 크로스 플랫폼 화면 캡처 (Windows / macOS / Linux)
- Real-time / Static Image 모드, `hsv_settings.json` 설정 저장·불러오기
- 단일 네이티브 바이너리, JIT 워밍업 없음

## UI Guide

| 영역 | 설명 |
|------|------|
| **메인 캔버스** | 선택한 뷰(Original / Mask / BBox)를 창 크기에 맞게 크게 표시 |
| **상단 툴바** | 뷰 전환, Tools 패널 Show/Hide, HSV 범위 요약 |
| **HSV Tools 패널** | 드래그·접기 가능한 플로팅 패널 — 모니터, 모드, HSV 슬라이더, 설정 저장 |

1. 상단 툴바에서 **Original** / **Mask** / **BBox** 뷰를 선택합니다.
2. **HSV Tools** 패널에서 H/S/V Min·Max 슬라이더로 탐지 색상 범위를 조정합니다.
3. **Monitoring Mode**
   - **Real-time**: 선택한 모니터 중앙 320×320 영역을 연속 캡처·탐지
   - **Static Image**: 한 번 캡처한 뒤 슬라이더 조정 시 즉시 재처리
4. **Load Settings** / **Save Settings**로 `hsv_settings.json`에 HSV 범위를 저장·불러옵니다.

## Keyboard Shortcuts

| 키 | 동작 |
|----|------|
| `1` | Original 뷰 |
| `2` | Mask 뷰 |
| `3` | BBox 뷰 |
| `F1` / `` ` `` | HSV Tools 패널 토글 |

## Configuration

설정 파일 [`hsv_settings.json`](hsv_settings.json):

| 필드 | 범위 | 설명 |
|------|------|------|
| `hue_min` / `hue_max` | 0–179 | Hue |
| `sat_min` / `sat_max` | 0–255 | Saturation |
| `val_min` / `val_max` | 0–255 | Value |

### 주요 색상 HSV 범위 예시

| 색상 | H | S | V |
|------|---|---|---|
| 빨간색 | 0–10 또는 170–179 | 100–255 | 100–255 |
| 파란색 | 100–130 | 100–255 | 100–255 |
| 초록색 | 40–80 | 100–255 | 100–255 |

## Project Structure

```
HsvColorPicker/
├── Cargo.toml
├── Cargo.lock
├── hsv_settings.json
└── src/
    ├── main.rs           # eframe 진입점
    ├── app.rs            # UI, 스레드, 뷰/모드 전환
    ├── capture.rs        # xcap 화면 캡처
    ├── settings.rs       # JSON 설정
    └── detection/        # HSV 탐지 파이프라인
        ├── hsv.rs
        ├── mask.rs
        ├── morph.rs
        └── contour.rs
```

## Branches

| 브랜치 / 태그 | 설명 |
|---------------|------|
| [`master`](https://github.com/NAMUORI00/HsvColorPicker/tree/master) | Rust (현재 개발) · [`v0.2.0`](https://github.com/NAMUORI00/HsvColorPicker/releases/tag/v0.2.0) |
| [`python`](https://github.com/NAMUORI00/HsvColorPicker/tree/python) | Python 레거시 (MSS + Tkinter + Numba) · [`v0.1.0-python`](https://github.com/NAMUORI00/HsvColorPicker/releases/tag/v0.1.0-python) |

## Environment Setup

### Rust 툴체인

1. [rustup](https://rustup.rs/) 설치 (stable 권장)
2. 버전 확인:

```bash
rustc --version
cargo --version
```

3. 저장소 클론:

```bash
git clone https://github.com/NAMUORI00/HsvColorPicker.git
cd HsvColorPicker
git checkout master
```

- Rust edition 2021 ([`Cargo.toml`](Cargo.toml))
- `Cargo.lock`이 포함되어 있어 동일한 의존성으로 재현 가능한 빌드가 가능합니다.

### Windows

| 항목 | 내용 |
|------|------|
| OS | Windows 10 이상 |
| 추가 설치 | [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ 빌드 도구) — `eframe` 및 네이티브 크레이트 컴파일용 |
| 화면 캡처 | xcap (DXGI) — 별도 런타임 패키지 불필요 |
| 권한 | 일반 사용자 실행 가능 |

### macOS

| 항목 | 내용 |
|------|------|
| 추가 설치 | Xcode Command Line Tools: `xcode-select --install` |
| 화면 캡처 권한 | **시스템 설정 → 개인정보 보호 → 화면 기록**에서 터미널 또는 앱 허용 |
| 아키텍처 | Apple Silicon / Intel 모두 Rust stable 지원 |

### Linux

[xcap](https://github.com/nashaofu/xcap)과 [eframe/winit](https://github.com/emilk/egui) 빌드에 필요한 시스템 패키지입니다.

**Debian / Ubuntu:**

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config libclang-dev \
  libxcb1-dev libxrandr-dev libdbus-1-dev \
  libpipewire-0.3-dev libwayland-dev libegl-dev \
  libgtk-3-dev libxkbcommon-dev libssl-dev
```

**Arch Linux:**

```bash
sudo pacman -S base-devel clang libxcb libxrandr dbus libpipewire \
  gtk3 libxkbcommon openssl
```

**Alpine Linux:**

```bash
sudo apk add build-base pkgconf llvm-dev clang-dev \
  libxcb-dev libxrandr-dev dbus-dev pipewire-dev \
  wayland-dev mesa-dev gtk+3.0-dev libxkbcommon-dev openssl-dev
```

- PipeWire **1.0+** 권장 (Ubuntu 22.04 이하에서 libspa 관련 빌드 오류 가능 → **Ubuntu 24.04+** 권장)
- Wayland / X11 모두 xcap 지원

## Build and Run

### 개발 빌드 (디버그)

빠른 반복 개발용입니다. 최적화가 적용되지 않습니다.

```bash
cargo build
cargo run
```

### 릴리스 빌드 (권장)

```bash
cargo build --release
cargo run --release
```

| OS | 실행 파일 경로 |
|----|----------------|
| Windows | `target\release\hsv-color-picker.exe` |
| Linux / macOS | `target/release/hsv-color-picker` |

빌드된 바이너리 직접 실행:

```bash
# Windows
.\target\release\hsv-color-picker.exe

# Linux / macOS
./target/release/hsv-color-picker
```

### 테스트

```bash
cargo test
```

- HSV 변환, 탐지, 설정 save/load, 화면 캡처 integration 등 6개 테스트 포함

### 클린 빌드

```bash
cargo clean
cargo build --release
```

### 첫 실행 시

- **프로젝트 루트**에서 실행하세요 (`hsv_settings.json`이 상대 경로로 로드됩니다).
- Real-time 모드에서는 시작 후 수 초 내 메인 캔버스에 프레임이 표시됩니다.

## Python Legacy Setup

Python 레거시 버전(`python` 브랜치) 실행 방법입니다. 상세 내용은 [python 브랜치 README](https://github.com/NAMUORI00/HsvColorPicker/tree/python)를 참고하세요.

```bash
git checkout python
pip install -r requirements.txt
python main.py
```

| 항목 | Rust (`master`) | Python (`python`) |
|------|-----------------|-------------------|
| 런타임 | Rust stable | Python 3.10+ |
| 의존성 | `Cargo.lock` | `requirements.txt` (numpy, mss, numba) |
| UI | egui | Tkinter |
| 화면 캡처 | xcap | MSS |

## Troubleshooting

| 증상 | 해결 |
|------|------|
| Linux `cargo build` linker / pkg-config 오류 | [Linux 시스템 패키지](#linux) 재확인 |
| Linux PipeWire / libspa 컴파일 오류 | PipeWire 1.0+ 설치, Ubuntu 24.04+ 권장 |
| macOS 캡처 실패 / 빈 화면 | **화면 기록** 권한 허용 |
| Windows `link.exe` not found | Visual Studio Build Tools (C++) 설치 |
| `hsv_settings.json` 미적용 | 프로젝트 **루트 디렉터리**에서 실행 |
| Real-time FPS 낮음 | 고해상도 모니터 전체 캡처 후 crop 방식 — 추후 최적화 예정 |

## Notes

- 선택한 모니터 **중앙 320×320** 영역만 캡처합니다.
- 최소 contour 면적 20 픽셀 이상만 탐지합니다.
- 조명·환경에 따라 HSV 값을 조정해야 할 수 있습니다.
