# HSV 실시간 객체 탐지

MSS/xcap으로 화면을 캡처하고, HSV 색상 범위로 객체를 실시간 탐지하는 Rust 데스크톱 앱입니다.

> Python 레거시 버전은 [`python`](https://github.com/NAMUORI00/HsvColorPicker/tree/python) 브랜치에서 확인할 수 있습니다.

## 요구 사항

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- Windows / macOS / Linux

## 설치 및 실행

```bash
cargo run --release
```

릴리스 빌드:

```bash
cargo build --release
# target/release/hsv-color-picker.exe (Windows)
```

## 사용 방법

1. **HSV Control** 창에서 H/S/V Min·Max 슬라이더로 탐지 색상 범위를 조정합니다.
   - H (Hue): 0–179
   - S (Saturation): 0–255
   - V (Value): 0–255

2. **Monitoring Mode**
   - **Real-time**: 선택한 모니터 중앙 320×320 영역을 연속 캡처·탐지
   - **Static Image**: 한 번 캡처한 뒤 슬라이더 조정 시 즉시 재처리

3. **Show Monitor**로 3패널 결과를 표시합니다.
   - 왼쪽: 원본
   - 중앙: HSV 마스크
   - 오른쪽: 바운딩 박스

4. **Load Settings** / **Save Settings**로 `hsv_settings.json`에 HSV 범위를 저장·불러옵니다.

## 프로젝트 특징

- OpenCV/PIL/numpy/numba 없이 HSV 변환·마스킹·팽창·윤곽선·바운딩 박스를 순수 Rust로 구현
- **egui** UI + **xcap** 화면 캡처
- 단일 네이티브 바이너리, JIT 워밍업 없음

## 프로젝트 구조

```
HsvColorPicker/
├── Cargo.toml
├── hsv_settings.json
└── src/
    ├── main.rs           # eframe 진입점
    ├── app.rs            # UI, 스레드, 모드 전환
    ├── capture.rs        # xcap 화면 캡처
    ├── settings.rs       # JSON 설정
    └── detection/        # HSV 탐지 파이프라인
        ├── hsv.rs
        ├── mask.rs
        ├── morph.rs
        └── contour.rs
```

## 주요 색상 HSV 범위 예시

| 색상 | H | S | V |
|------|---|---|---|
| 빨간색 | 0–10 또는 170–179 | 100–255 | 100–255 |
| 파란색 | 100–130 | 100–255 | 100–255 |
| 초록색 | 40–80 | 100–255 | 100–255 |

## 주의사항

- 선택한 모니터 **중앙 320×320** 영역만 캡처합니다.
- 최소 contour 면적 20 픽셀 이상만 탐지합니다.
- 조명·환경에 따라 HSV 값을 조정해야 할 수 있습니다.
