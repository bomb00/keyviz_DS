// 눌린 키 조합을 사람이 읽는 단축키 캡션(⌘C→Copy 등)으로 변환한다
import { KeyEvent, MODIFIERS, RawKey } from "@/types/event";

// 주 수식키(Mod)는 macOS에선 ⌘(Meta), 그 외에선 ⌃(Ctrl)로 정규화한다
const IS_MAC = typeof navigator !== "undefined" && /Mac/i.test(navigator.userAgent);

type ModToken = "Mod" | "Ctrl" | "Alt" | "Shift";

// 조합에 포함된 수식키를 정규화 토큰 집합으로 변환
function modifierTokens(names: string[]): Set<ModToken> {
  const tokens = new Set<ModToken>();
  const has = (...keys: string[]) => keys.some((k) => names.includes(k));

  const meta = has(RawKey.MetaLeft, RawKey.MetaRight);
  const ctrl = has(RawKey.ControlLeft, RawKey.ControlRight);

  if (IS_MAC) {
    if (meta) tokens.add("Mod");
    if (ctrl) tokens.add("Ctrl");
  } else {
    if (ctrl || meta) tokens.add("Mod");
  }
  if (has(RawKey.Alt)) tokens.add("Alt");
  if (has(RawKey.ShiftLeft, RawKey.ShiftRight)) tokens.add("Shift");

  return tokens;
}

// 수식키가 아닌 '주 키' 이름을 캡션 토큰으로 변환 (KeyC→C, Num4→4 등)
function mainKeyToken(name: string): string | null {
  if (name.startsWith("Key")) return name.slice(3);
  if (name.startsWith("Num")) return name.slice(3);
  const map: Record<string, string> = {
    [RawKey.Space]: "Space",
    [RawKey.Tab]: "Tab",
    [RawKey.Return]: "Return",
    [RawKey.Escape]: "Esc",
    [RawKey.Delete]: "Delete",
    [RawKey.Backspace]: "Backspace",
    [RawKey.Comma]: ",",
    [RawKey.Dot]: ".",
    [RawKey.Slash]: "/",
  };
  return map[name] ?? null;
}

// 조합을 사전 조회용 정규화 문자열로 만든다 (단일 주 키 조합만 대상)
function canonical(names: string[]): string | null {
  const mods = modifierTokens(names);
  const mains = names.filter((n) => !MODIFIERS.has(n));
  if (mains.length !== 1) return null;

  const main = mainKeyToken(mains[0]);
  if (!main) return null;

  const order: ModToken[] = ["Mod", "Ctrl", "Alt", "Shift"];
  const parts = order.filter((m) => mods.has(m));
  return [...parts, main].join("+");
}

// 캡션 사전. Mod = macOS ⌘ / Windows·Linux ⌃
const CAPTIONS: Record<string, string> = {
  "Mod+C": "Copy",
  "Mod+V": "Paste",
  "Mod+X": "Cut",
  "Mod+Z": "Undo",
  "Mod+Shift+Z": "Redo",
  "Mod+A": "Select All",
  "Mod+S": "Save",
  "Mod+Shift+S": "Save As",
  "Mod+F": "Find",
  "Mod+G": "Find Next",
  "Mod+N": "New",
  "Mod+Shift+N": "New Window",
  "Mod+O": "Open",
  "Mod+P": "Print",
  "Mod+W": "Close",
  "Mod+Q": "Quit",
  "Mod+T": "New Tab",
  "Mod+Shift+T": "Reopen Tab",
  "Mod+R": "Reload",
  "Mod+B": "Bold",
  "Mod+I": "Italic",
  "Mod+U": "Underline",
  "Mod+D": "Duplicate",
  "Mod+K": "Command Palette",
  "Mod+,": "Preferences",
  "Mod+Space": "Spotlight",
  "Mod+Tab": "Switch App",
  "Mod+Shift+3": "Screenshot",
  "Mod+Shift+4": "Screenshot",
  "Alt+Tab": "Switch Window",
};

// 키 그룹에 대응하는 단축키 캡션을 반환한다. 없으면 null
export function getCaption(keys: KeyEvent[]): string | null {
  if (keys.length < 2) return null;
  const key = canonical(keys.map((k) => k.name));
  if (!key) return null;
  return CAPTIONS[key] ?? null;
}
