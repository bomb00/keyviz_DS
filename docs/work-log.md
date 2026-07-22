# 작업 기록

## 2026-07-22. Keyviz 2.2.0 기능 개선.

- 변경 파일. Dock 아이콘 표시 설정, 로그인 시 자동 시작, 단축키 캡션, 활성 모니터 추적, 키 표시 애니메이션 관련 소스와 설정 파일을 변경했습니다.
- 버전. `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, About 화면을 `2.2.0`으로 통일했습니다.
- 트러블슈팅. Minimal 키캡이 눌린 동안 `scale: 0.95`를 적용해 텍스트 크기가 달라 보이던 문제를 해결했습니다.
- 검증. TypeScript 타입 검사, Vite 프로덕션 빌드, Rust `cargo check`를 실행했습니다.
