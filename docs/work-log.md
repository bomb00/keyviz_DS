# 작업 기록

## 2026-07-22. Keyviz 2.2.0 기능 개선.

- 변경 파일. Dock 아이콘 표시 설정, 로그인 시 자동 시작, 단축키 캡션, 활성 모니터 추적, 키 표시 애니메이션 관련 소스와 설정 파일을 변경했습니다.
- 버전. `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, About 화면을 `2.2.0`으로 통일했습니다.
- Dock 동작. 앱 활성화 정책을 바꾸지 않고 `set_dock_visibility`로 아이콘만 숨겨 입력 이벤트와 자동 사라짐 타이머가 계속 동작하도록 수정했습니다.
- 단축키 표시. Minimal 키캡의 눌림 축소 효과를 제거하고, Hotkeys 필터에서는 일반 키가 추가된 완성 단축키만 최종 폭으로 표시하도록 수정했습니다.
- 입력 권한. 로컬 앱의 지정 요구사항을 `org.keyviz` 식별자로 고정해 ad-hoc 재서명 시 입력 모니터링 권한을 안정적으로 관리하도록 설치했습니다.
- 검증. TypeScript 타입 검사, Vite 프로덕션 빌드, Rust `cargo check`를 실행했습니다.
