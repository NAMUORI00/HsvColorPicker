# HSV Color Picker

xcap 화면 캡처와 HSV 색상 필터링으로 객체를 실시간 탐지하는 Rust 데스크톱 앱입니다.

> Python 레거시 버전: [`python` 브랜치](https://github.com/NAMUORI00/HsvColorPicker/tree/python) · 태그 [`v0.1.0-python`](https://github.com/NAMUORI00/HsvColorPicker/releases/tag/v0.1.0-python)

## Features

- OpenCV/PIL/numpy 없이 HSV 변환·마스킹·팽창·윤곽선·바운딩 박스를 순수 Rust로 구현
- **egui** ImGui 스타일 UI — 이미지 메인 + 토글 가능한 플로팅 툴 패널
- **xcap** 기반 크로스 플랫폼 화면 캡처 (Windows / macOS / Linux)
- Real-time / Static Image 모드, `hsv_settings.json` 설정 저장·불러오기
- 단일 네이티브 바이너리, JIT 워밍업 없음

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- Windows / macOS / Linux

## Quick Start

```bash
cargo run --release
```

릴리스 빌드:

```bash
cargo build --release
# Windows: target/release/hsv-color-picker.exe
# Linux/macOS: target/release/hsv-color-picker
```

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

## Development

```bash
cargo test
cargo build --release
```

## Notes

- 선택한 모니터 **중앙 320×320** 영역만 캡처합니다.
- 최소 contour 면적 20 픽셀 이상만 탐지합니다.
- 조명·환경에 따라 HSV 값을 조정해야 할 수 있습니다.
