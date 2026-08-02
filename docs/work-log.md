# 작업 기록

## 2026-07-22. Keyviz 2.2.0 기능 개선.

- 변경 파일. Dock 아이콘 표시 설정, 로그인 시 자동 시작, 단축키 캡션, 활성 모니터 추적, 키 표시 애니메이션 관련 소스와 설정 파일을 변경했습니다.
- 버전. `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, About 화면을 `2.2.0`으로 통일했습니다.
- Dock 동작. 앱 활성화 정책을 바꾸지 않고 `set_dock_visibility`로 아이콘만 숨겨 입력 이벤트와 자동 사라짐 타이머가 계속 동작하도록 수정했습니다.
- 단축키 표시. Minimal 키캡의 눌림 축소 효과를 제거하고, Hotkeys 필터에서는 일반 키가 추가된 완성 단축키만 최종 폭으로 표시하도록 수정했습니다.
- 입력 권한. 로컬 앱의 지정 요구사항을 `org.keyviz` 식별자로 고정해 ad-hoc 재서명 시 입력 모니터링 권한을 안정적으로 관리하도록 설치했습니다.
- 검증. TypeScript 타입 검사, Vite 프로덕션 빌드, Rust `cargo check`를 실행했습니다.
- 단축키 유지 옵션. 마지막 키를 뗄 때까지 전체 단축키를 유지하고 한 번에 사라지게 하는 `Keep Shortcut Together` 설정을 추가했습니다.
- Shift 표시 수정. Shift가 Command보다 먼저 눌려도 현재 물리 입력 전체로 표시 그룹을 시작해 `Command + Shift + 문자` 조합에서 Shift가 빠지지 않도록 수정했습니다.
- macOS 입력 안정성. modifier별 기기 플래그로 좌우 키 상태를 판정하고, macOS가 이벤트 탭을 비활성화하면 즉시 다시 활성화하도록 수정했습니다.
- 최종 검증. TypeScript 타입 검사, Vite 프로덕션 빌드, Rust `cargo check`, 앱 서명 검증을 통과했으며 `/Applications/keyviz.app`에 설치해 실제 입력을 확인했습니다. 프로젝트에는 lint 스크립트가 없고 DMG 포장 스크립트는 기존 오류로 실패했습니다.

## 2026-08-02. 눌린 채 남는 키 자동 해제.

- 변경 파일. `src/stores/key_event.ts`, `src-tauri/src/app/commands.rs`, `src-tauri/src/lib.rs`를 변경했습니다.
- 문제. 키를 뗄 때 release 이벤트가 유실되면 해당 키가 눌린 상태로 남아 화면에서 사라지지 않았습니다.
- 프론트 처리. 키별 누른 시각을 `pressedAt`에 기록하고 `tick()`에서 `MAX_KEY_HOLD_MS`(30초)를 넘긴 키를 강제로 해제하도록 수정했습니다.
- 백엔드 처리. `clear_pressed_keys` 커맨드를 추가해 Rust 쪽 `pressed_keys` 상태에서도 같은 키를 함께 제거하도록 수정했습니다.
- 저장 제외. `pressedAt`은 물리 상태이므로 `partialize`에서 제외해 영구 저장되지 않도록 했습니다.
- 검증. TypeScript 타입 검사(에러 0), Vite 프로덕션 빌드(종료 코드 0), Rust `cargo check`(종료 코드 0)를 통과했습니다.

## 2026-08-02. DMG 앱 번들 서명 누락 수정.

- 변경 파일. `src-tauri/tauri.conf.json`을 변경했습니다.
- DMG 포장 오류. 이전에 기록된 포장 실패는 재현되지 않았습니다. `src-tauri/target/release/bundle/macos/`에 남아 있던 실패 잔여물 `rw.55129.*.dmg`, `rw.63382.*.dmg`를 제거한 뒤 빌드하니 정상 생성되었고 `hdiutil verify` 체크섬이 VALID였습니다.
- 서명 문제. `bundle.macOS` 설정이 없어 Tauri가 앱 번들 서명 단계를 건너뛰었고, DMG에는 `_CodeSignature`가 없는 링커 임시 서명(`keyviz-0df704fb3fd882c5`, linker-signed) 상태의 앱이 들어갔습니다. 기존 설치본이 정상이었던 것은 이전에 수동으로 재서명했기 때문입니다.
- 수정. `bundle.macOS.signingIdentity`를 `"-"`로 지정해 빌드가 ad-hoc 번들 서명을 자동으로 수행하도록 했습니다.
- 검증. 재빌드한 DMG 안의 앱이 `Identifier=org.keyviz`, flags `0x10002`(adhoc, runtime)로 서명되었고 `codesign --verify --deep --strict`가 종료 코드 0으로 통과했으며 `Contents/_CodeSignature/CodeResources`가 생성되었습니다.
- 남은 확인. 서명 시 hardened runtime이 함께 적용되므로(설치본은 `0x2`, 신규 빌드는 `0x10002`) 설치 후 입력 모니터링 동작을 실제로 확인해야 합니다. 공증(notarization)은 Apple 개발자 계정 환경변수가 없어 건너뜁니다.
