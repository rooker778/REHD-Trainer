#![windows_subsystem = "windows"]
// Resident Evil HD Remaster (2026-09-28 build) standalone trainer.
// Cheat implementations reproduced from WeMod trainer #50372 and FLiNG's item-box cave.
// UI: custom dark theme in the spirit of FLiNG's trainer.

use std::ffi::c_void;

mod blobs;
use blobs::*;

static TOGGLE_ENABLE: &[u8] = include_bytes!("../assets/toggle_enable.wav");
static TOGGLE_DISABLE: &[u8] = include_bytes!("../assets/toggle_disable.wav");
static ICON_ICO: &[u8] = include_bytes!("../assets/trainer.ico");

type HANDLE = *mut c_void;
type HWND = *mut c_void;
type HINSTANCE = *mut c_void;
type BOOL = i32;
type DWORD = u32;
type UINT = u32;
type WPARAM = usize;
type LPARAM = isize;
type LRESULT = isize;

const PROCESS_VM_OPERATION: DWORD = 0x0008;
const PROCESS_VM_READ: DWORD = 0x0010;
const PROCESS_VM_WRITE: DWORD = 0x0020;
const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
const MEM_COMMIT: DWORD = 0x1000;
const MEM_RESERVE: DWORD = 0x2000;
const MEM_RELEASE: DWORD = 0x8000;
const PAGE_EXECUTE_READWRITE: DWORD = 0x40;
const STILL_ACTIVE: DWORD = 259;
const TH32CS_SNAPPROCESS: DWORD = 0x2;

const WM_CREATE: UINT = 0x0001;
const WM_DESTROY: UINT = 0x0002;
const WM_CLOSE: UINT = 0x0010;
const WM_PAINT: UINT = 0x000F;
const WM_ERASEBKGND: UINT = 0x0014;
const WM_MOUSEMOVE: UINT = 0x0200;
const WM_LBUTTONUP: UINT = 0x0202;
const WM_NCHITTEST: UINT = 0x0084;
const WM_TIMER: UINT = 0x0113;
const WM_SETCURSOR: UINT = 0x0020;
const WM_HOTKEY: UINT = 0x0312;

const WS_POPUP: DWORD = 0x8000_0000;
const WM_SETICON: UINT = 0x0080;
const IMAGE_ICON: UINT = 1;
const LR_LOADFROMFILE: UINT = 0x0010;
const GCLP_HICON: i32 = -14;
const GCLP_HICONSM: i32 = -34;
const WS_SYSMENU: DWORD = 0x0008_0000;
const WS_MINIMIZEBOX: DWORD = 0x0002_0000;
const WS_EX_APPWINDOW: DWORD = 0x0004_0000;
const SW_MINIMIZE: i32 = 6;
const HTCLIENT: LRESULT = 1;
const HTCAPTION: LRESULT = 2;

const SND_ASYNC: DWORD = 0x0001;
const SND_NODEFAULT: DWORD = 0x0002;
const SND_MEMORY: DWORD = 0x0004;

const BASE: u32 = 0x0040_0000;
const EXPECTED_SIZE: u32 = 0x009a_6000;
const GAME_EXE: &str = "bhd.exe";

const WIN_W: i32 = 520;
const WIN_H: i32 = 600;
const TITLE_H: i32 = 44;
const ROWS_Y: i32 = 164;
const ROW_STEP: i32 = 36;
const ROW_H: i32 = 34;
const ROW_X: i32 = 14;
const ROW_W: i32 = WIN_W - 28;
const SW_X: i32 = WIN_W - 62;
const SW_W: i32 = 40;
const SW_H: i32 = 18;

const BG: u32 = 0x0011_1513;
const TITLE_BG: u32 = 0x0015_1A17;
const PANEL: u32 = 0x001E_241F;
const PANEL_HI: u32 = 0x0028_302A;
const ROW_BG: u32 = 0x001B_211D;
const ROW_HOVER: u32 = 0x0023_2B25;
const ROW_ON: u32 = 0x001B_3023;
const LINE: u32 = 0x002A_332C;
const ACCENT: u32 = 0x0078_C878;
const ACCENT_DIM: u32 = 0x004F_7F54;
const TEXT: u32 = 0x00E6_EBE6;
const TEXT_DIM: u32 = 0x00A8_B4AA;
const MUTED: u32 = 0x0080_8F84;
const SWITCH_OFF: u32 = 0x003A_443D;
const KNOB: u32 = 0x00F0_F5F0;
const RED: u32 = 0x0060_6060; // placeholder, see DANGER
const DANGER: u32 = 0x005A_5AC8;

const NAMES: [&str; 10] = [
    "无限生命",
    "玩家速度",
    "一击必杀",
    "冻结敌人",
    "无限弹药",
    "无限物品",
    "无限防御道具",
    "无限存档（墨带）",
    "冻结冷却计时",
    "随时随地异次元箱",
];

struct Site {
    va: u32,
    len: u32,
    cave_off: u32,
    orig: &'static [u8],
}

const SITES: [Site; 10] = [
    Site { va: 0x0069_e3bb, len: 7, cave_off: 0x040, orig: &[0x83, 0xBE, 0xBC, 0x13, 0x00, 0x00, 0x00] },
    Site { va: 0x0057_2242, len: 6, cave_off: 0x080, orig: &[0x89, 0x8E, 0xBC, 0x13, 0x00, 0x00] },
    Site { va: 0x0080_54e0, len: 8, cave_off: 0x0C0, orig: &[0x0F, 0x28, 0xD7, 0xF3, 0x0F, 0x58, 0x56, 0x40] },
    Site { va: 0x006a_bb7b, len: 7, cave_off: 0x140, orig: &[0x83, 0xBE, 0xBC, 0x13, 0x00, 0x00, 0x00] },
    Site { va: 0x006b_6f4e, len: 8, cave_off: 0x180, orig: &[0x8B, 0x81, 0xFC, 0x0F, 0x00, 0x00, 0x2B, 0x81, 0x04, 0x10, 0x00, 0x00] },
    Site { va: 0x0054_6ea7, len: 5, cave_off: 0x1C0, orig: &[0xFF, 0x70, 0x04, 0x8B, 0xCF] },
    Site { va: 0x004e_ef9d, len: 8, cave_off: 0x200, orig: &[0xF3, 0x0F, 0x58, 0x81, 0x4C, 0x47, 0x0E, 0x00] },
    Site { va: 0x004e_f72e, len: 8, cave_off: 0x240, orig: &[0x8B, 0x5B, 0x04, 0x4B, 0x80, 0x79, 0x1C, 0x00] },
    Site { va: 0x0048_ad0b, len: 8, cave_off: 0, orig: &[0xF3, 0x0F, 0x5C, 0x80, 0xF8, 0x82, 0x02, 0x00] },
    Site { va: 0x0054_4f53, len: 6, cave_off: 0x280, orig: &[0x89, 0x83, 0xA0, 0x60, 0x00, 0x00] },
];

struct State {
    hwnd: HWND,
    hproc: HANDLE,
    pid: u32,
    attached: bool,
    cave: u32,
    installed: [bool; 10],
    toggles: [bool; 10],
    speed: u32,
    hover_row: i32,
    hover_btn: u8,
    msg: String,
    fonts: [*mut c_void; 4],
}

static mut STATE: Option<State> = None;

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct PaintStruct {
    hdc: *mut c_void,
    f_erase: BOOL,
    rc: Rect,
    f_restore: BOOL,
    f_inc_update: BOOL,
    rgb_reserved: [u8; 32],
}

#[repr(C)]
struct SizeC {
    cx: i32,
    cy: i32,
}

#[repr(C)]
struct Msg {
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    time: DWORD,
    pt: Point,
    private: DWORD,
}

#[repr(C)]
struct WndClassW {
    style: UINT,
    lpfn_wndproc: Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: HINSTANCE,
    h_icon: *mut c_void,
    h_cursor: *mut c_void,
    hbr_background: *mut c_void,
    lpsz_menu_name: *const u16,
    lpsz_class_name: *const u16,
}

#[repr(C)]
struct ProcessEntry32W {
    dw_size: DWORD,
    cnt_usage: DWORD,
    th32_process_id: DWORD,
    th32_default_heap_id: usize,
    th32_module_id: DWORD,
    cnt_threads: DWORD,
    th32_parent_process_id: DWORD,
    pc_pri_class_base: i32,
    dw_flags: DWORD,
    sz_exe_file: [u16; 260],
}

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> HINSTANCE;
    fn CreateToolhelp32Snapshot(flags: DWORD, pid: DWORD) -> HANDLE;
    fn Process32FirstW(snapshot: HANDLE, entry: *mut ProcessEntry32W) -> BOOL;
    fn Process32NextW(snapshot: HANDLE, entry: *mut ProcessEntry32W) -> BOOL;
    fn OpenProcess(access: DWORD, inherit: BOOL, pid: DWORD) -> HANDLE;
    fn CloseHandle(h: HANDLE) -> BOOL;
    fn GetExitCodeProcess(h: HANDLE, code: *mut DWORD) -> BOOL;
    fn ReadProcessMemory(h: HANDLE, addr: *const c_void, buf: *mut c_void, size: usize, read: *mut usize) -> BOOL;
    fn WriteProcessMemory(h: HANDLE, addr: *mut c_void, buf: *const c_void, size: usize, written: *mut usize) -> BOOL;
    fn VirtualAllocEx(h: HANDLE, addr: *mut c_void, size: usize, alloc: DWORD, protect: DWORD) -> *mut c_void;
    fn VirtualFreeEx(h: HANDLE, addr: *mut c_void, size: usize, free_type: DWORD) -> BOOL;
    fn VirtualProtectEx(h: HANDLE, addr: *mut c_void, size: usize, new_protect: DWORD, old: *mut DWORD) -> BOOL;
    fn FlushInstructionCache(h: HANDLE, addr: *const c_void, size: usize) -> BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassW(class: *const WndClassW) -> u16;
    fn CreateWindowExW(ex_style: DWORD, class: *const u16, name: *const u16, style: DWORD,
                       x: i32, y: i32, w: i32, h: i32, parent: HWND, menu: *mut c_void,
                       instance: HINSTANCE, param: *mut c_void) -> HWND;
    fn DefWindowProcW(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
    fn GetMessageW(msg: *mut Msg, hwnd: HWND, min: UINT, max: UINT) -> BOOL;
    fn TranslateMessage(msg: *const Msg) -> BOOL;
    fn DispatchMessageW(msg: *const Msg) -> LRESULT;
    fn PostQuitMessage(code: i32);
    fn DestroyWindow(hwnd: HWND) -> BOOL;
    fn ShowWindow(hwnd: HWND, cmd: i32) -> BOOL;
    fn UpdateWindow(hwnd: HWND) -> BOOL;
    fn RegisterHotKey(hwnd: HWND, id: i32, modifiers: UINT, vk: UINT) -> BOOL;
    fn SetTimer(hwnd: HWND, id: usize, elapse: UINT, func: *mut c_void) -> usize;
    fn LoadImageW(instance: HINSTANCE, name: *const u16, kind: UINT, cx: i32, cy: i32, load: UINT) -> HANDLE;
    fn SendMessageW(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
    fn SetClassLongPtrW(hwnd: HWND, index: i32, value: isize) -> isize;
    fn LoadCursorW(instance: HINSTANCE, name: *const u16) -> *mut c_void;
    fn SetCursor(cursor: *mut c_void) -> *mut c_void;
    fn GetCursorPos(pt: *mut Point) -> BOOL;
    fn BeginPaint(hwnd: HWND, ps: *mut PaintStruct) -> *mut c_void;
    fn EndPaint(hwnd: HWND, ps: *const PaintStruct) -> BOOL;
    fn InvalidateRect(hwnd: HWND, rect: *const Rect, erase: BOOL) -> BOOL;
    fn GetClientRect(hwnd: HWND, rect: *mut Rect) -> BOOL;
    fn ScreenToClient(hwnd: HWND, pt: *mut Point) -> BOOL;
    fn GetSystemMetrics(index: i32) -> i32;
}

#[link(name = "gdi32")]
extern "system" {
    fn GetStockObject(object: i32) -> *mut c_void;
    fn CreateFontW(c_height: i32, c_width: i32, c_escapement: i32, c_orientation: i32,
                   c_weight: i32, b_italic: DWORD, b_underline: DWORD, b_strikeout: DWORD,
                   i_charset: DWORD, i_out_precision: DWORD, i_clip_precision: DWORD,
                   i_quality: DWORD, i_pitch_and_family: DWORD, face: *const u16) -> *mut c_void;
    fn CreateSolidBrush(color: DWORD) -> *mut c_void;
    fn SelectObject(dc: *mut c_void, obj: *mut c_void) -> *mut c_void;
    fn DeleteObject(obj: *mut c_void) -> BOOL;
    fn SetTextColor(dc: *mut c_void, color: DWORD) -> DWORD;
    fn SetBkMode(dc: *mut c_void, mode: i32) -> i32;
    fn TextOutW(dc: *mut c_void, x: i32, y: i32, s: *const u16, len: i32) -> BOOL;
    fn GetTextExtentPoint32W(dc: *mut c_void, s: *const u16, len: i32, size: *mut SizeC) -> BOOL;
    fn FillRect(dc: *mut c_void, rect: *const Rect, brush: *mut c_void) -> i32;
    fn RoundRect(dc: *mut c_void, x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> BOOL;
    fn CreateCompatibleDC(dc: *mut c_void) -> *mut c_void;
    fn CreateCompatibleBitmap(dc: *mut c_void, w: i32, h: i32) -> *mut c_void;
    fn BitBlt(dst: *mut c_void, x: i32, y: i32, w: i32, h: i32, src: *mut c_void, sx: i32, sy: i32, rop: DWORD) -> BOOL;
    fn DeleteDC(dc: *mut c_void) -> BOOL;
}

#[link(name = "winmm")]
extern "system" {
    fn PlaySoundW(sound: *const u16, module: HANDLE, flags: DWORD) -> BOOL;
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn make_font(size: i32, weight: i32) -> *mut c_void {
    let face = wide("微软雅黑");
    CreateFontW(-size, 0, 0, 0, weight, 0, 0, 0, 1, 0, 0, 5, 0, face.as_ptr())
}

unsafe fn fill_rect(dc: *mut c_void, x: i32, y: i32, w: i32, h: i32, color: u32) {
    let brush = CreateSolidBrush(color);
    let r = Rect { left: x, top: y, right: x + w, bottom: y + h };
    FillRect(dc, &r, brush);
    DeleteObject(brush);
}

unsafe fn round_rect(dc: *mut c_void, x: i32, y: i32, w: i32, h: i32, rad: i32, color: u32) {
    let brush = CreateSolidBrush(color);
    let old_brush = SelectObject(dc, brush);
    let none = GetStockObject(8); // NULL_PEN
    let old_pen = SelectObject(dc, none);
    RoundRect(dc, x, y, x + w, y + h, rad, rad);
    SelectObject(dc, old_pen);
    SelectObject(dc, old_brush);
    DeleteObject(brush);
}

unsafe fn measure_text(dc: *mut c_void, s: &str, font: *mut c_void) -> i32 {
    let old = SelectObject(dc, font);
    let w = wide(s);
    let mut size = SizeC { cx: 0, cy: 0 };
    GetTextExtentPoint32W(dc, w.as_ptr(), (w.len() - 1) as i32, &mut size);
    SelectObject(dc, old);
    size.cx
}

unsafe fn draw_text(dc: *mut c_void, x: i32, y: i32, s: &str, color: u32, font: *mut c_void) -> i32 {
    let old = SelectObject(dc, font);
    SetTextColor(dc, color);
    SetBkMode(dc, 1); // TRANSPARENT
    let w = wide(s);
    let len = (w.len() - 1) as i32;
    TextOutW(dc, x, y, w.as_ptr(), len);
    let mut size = SizeC { cx: 0, cy: 0 };
    GetTextExtentPoint32W(dc, w.as_ptr(), len, &mut size);
    SelectObject(dc, old);
    size.cx
}

unsafe fn draw_text_centered(dc: *mut c_void, cx: i32, y: i32, s: &str, color: u32, font: *mut c_void) {
    let tw = measure_text(dc, s, font);
    draw_text(dc, cx - tw / 2, y, s, color, font);
}

unsafe fn play_toggle(on: bool) {
    let wav: &[u8] = if on { TOGGLE_ENABLE } else { TOGGLE_DISABLE };
    PlaySoundW(
        wav.as_ptr() as *const u16,
        std::ptr::null_mut(),
        SND_MEMORY | SND_ASYNC | SND_NODEFAULT,
    );
}

fn required_hooks(idx: usize) -> &'static [usize] {
    match idx {
        0 => &[0, 1],
        1 => &[0, 2],
        2 => &[3],
        3 => &[0, 2],
        4 => &[4],
        5 => &[5],
        6 => &[6],
        7 => &[7],
        8 => &[8],
        9 => &[9],
        _ => &[],
    }
}

unsafe fn action_sound(idx: i32, on: bool) {
    let Some(st) = STATE.as_ref() else { return };
    if !st.attached {
        return;
    }
    if on {
        if required_hooks(idx as usize).iter().all(|&h| st.installed[h]) {
            play_toggle(true);
        }
    } else {
        play_toggle(false);
    }
}

unsafe fn read_mem(h: HANDLE, addr: u32, len: usize) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; len];
    let mut got = 0usize;
    if ReadProcessMemory(h, addr as *const c_void, buf.as_mut_ptr() as *mut c_void, len, &mut got) != 0 && got == len {
        Some(buf)
    } else {
        None
    }
}

unsafe fn write_mem(h: HANDLE, addr: u32, data: &[u8]) -> bool {
    let mut written = 0usize;
    WriteProcessMemory(h, addr as *mut c_void, data.as_ptr() as *const c_void, data.len(), &mut written) != 0
        && written == data.len()
}

unsafe fn write_code(h: HANDLE, addr: u32, data: &[u8]) -> bool {
    let mut old = 0u32;
    if VirtualProtectEx(h, addr as *mut c_void, data.len(), PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return false;
    }
    let ok = write_mem(h, addr, data);
    let mut tmp = 0u32;
    VirtualProtectEx(h, addr as *mut c_void, data.len(), old, &mut tmp);
    if ok {
        FlushInstructionCache(h, addr as *const c_void, data.len());
    }
    ok
}

fn find_pid() -> Option<u32> {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == usize::MAX as HANDLE || snap.is_null() {
            return None;
        }
        let mut entry: ProcessEntry32W = std::mem::zeroed();
        entry.dw_size = std::mem::size_of::<ProcessEntry32W>() as u32;
        let mut found = None;
        if Process32FirstW(snap, &mut entry) != 0 {
            loop {
                let name = String::from_utf16_lossy(&entry.sz_exe_file);
                let name = name.trim_end_matches('\0');
                if name.eq_ignore_ascii_case(GAME_EXE) {
                    found = Some(entry.th32_process_id);
                    break;
                }
                if Process32NextW(snap, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snap);
        found
    }
}

unsafe fn verify_target(h: HANDLE) -> Result<(), String> {
    let head = read_mem(h, BASE, 0x1000).ok_or_else(|| "无法读取游戏内存（请以管理员身份运行）".to_string())?;
    if &head[0..2] != b"MZ" {
        return Err("bhd.exe 基址校验失败".into());
    }
    let e_lfanew = u32::from_le_bytes([head[0x3C], head[0x3D], head[0x3E], head[0x3F]]) as usize;
    if e_lfanew + 0x60 > head.len() || &head[e_lfanew..e_lfanew + 4] != b"PE\0\0" {
        return Err("PE 头校验失败".into());
    }
    let machine = u16::from_le_bytes([head[e_lfanew + 4], head[e_lfanew + 5]]);
    if machine != 0x014c {
        return Err("不是 32 位 bhd.exe".into());
    }
    let opt = e_lfanew + 24;
    let size = u32::from_le_bytes([head[opt + 56], head[opt + 57], head[opt + 58], head[opt + 59]]);
    if size != EXPECTED_SIZE {
        return Err(format!("游戏版本不匹配（模块大小 {:#x}，需要 2026-09-28 更新版）", size));
    }
    for s in SITES.iter() {
        let cur = read_mem(h, s.va, s.orig.len()).ok_or_else(|| "读取代码段失败".to_string())?;
        if cur != s.orig {
            return Err(format!("游戏版本不匹配（校验点 {:#x}）", s.va));
        }
    }
    Ok(())
}

unsafe fn allocate_cave(h: HANDLE) -> Option<u32> {
    let p = VirtualAllocEx(h, std::ptr::null_mut(), 0x1000, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if p.is_null() {
        return None;
    }
    let blob: &[u8] = &BLOB;
    let mut written = 0usize;
    if WriteProcessMemory(h, p, blob.as_ptr() as *const c_void, blob.len(), &mut written) == 0
        || written != blob.len()
    {
        VirtualFreeEx(h, p, 0, MEM_RELEASE);
        return None;
    }
    FlushInstructionCache(h, p, blob.len());
    Some(p as usize as u32)
}

unsafe fn install_hook(st: &mut State, idx: usize) -> bool {
    if st.installed[idx] {
        return true;
    }
    if idx == 8 {
        let nops = [0x90u8; 8];
        if !write_code(st.hproc, SITES[idx].va, &nops) {
            return false;
        }
    } else {
        let site = &SITES[idx];
        let target = st.cave + site.cave_off;
        let rel = (target as i64 - (site.va as i64 + 5)) as i32;
        let mut patch = vec![0xE9u8];
        patch.extend_from_slice(&rel.to_le_bytes());
        while patch.len() < site.len as usize {
            patch.push(0x90);
        }
        if !write_code(st.hproc, site.va, &patch) {
            return false;
        }
    }
    st.installed[idx] = true;
    true
}

unsafe fn uninstall_hook(st: &mut State, idx: usize) -> bool {
    if !st.installed[idx] {
        return true;
    }
    let ok = write_code(st.hproc, SITES[idx].va, SITES[idx].orig);
    if ok {
        st.installed[idx] = false;
    }
    ok
}

fn site_name(idx: usize) -> &'static str {
    match idx {
        0 | 1 => "无限生命",
        2 => "速度/冻结",
        3 => "一击必杀",
        4 => "无限弹药",
        5 => "无限物品",
        6 => "防御道具",
        7 => "无限存档",
        8 => "冷却计时",
        9 => "异次元箱",
        _ => "?",
    }
}

unsafe fn detach(st: &mut State) {
    for i in 0..10 {
        uninstall_hook(st, i);
    }
    st.installed = [false; 10];
    if !st.hproc.is_null() {
        if st.cave != 0 {
            VirtualFreeEx(st.hproc, st.cave as *mut c_void, 0, MEM_RELEASE);
        }
        CloseHandle(st.hproc);
    }
    st.hproc = std::ptr::null_mut();
    st.pid = 0;
    st.attached = false;
    st.cave = 0;
}

unsafe fn try_attach(st: &mut State) -> Result<(), String> {
    let pid = find_pid().ok_or_else(|| "等待 bhd.exe 启动…".to_string())?;
    if st.attached && st.pid == pid {
        return Ok(());
    }
    if st.attached {
        if st.pid != pid {
            st.toggles = [false; 10];
        }
        detach(st);
    }
    let rights = PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION;
    let mut h = OpenProcess(rights, 0, pid);
    if h.is_null() {
        h = OpenProcess(0x1F0FFF, 0, pid);
    }
    if h.is_null() {
        return Err("无法打开游戏进程（尝试以管理员身份运行）".into());
    }
    st.hproc = h;
    st.pid = pid;
    if let Err(e) = verify_target(h) {
        CloseHandle(h);
        st.hproc = std::ptr::null_mut();
        st.pid = 0;
        return Err(e);
    }
    match allocate_cave(h) {
        Some(cave) => {
            st.cave = cave;
            st.attached = true;
            st.msg.clear();
            Ok(())
        }
        None => {
            CloseHandle(h);
            st.hproc = std::ptr::null_mut();
            st.pid = 0;
            Err("无法在游戏内分配代码空间".into())
        }
    }
}

unsafe fn sync() {
    let st = STATE.as_mut().unwrap();
    if !st.attached {
        if let Err(e) = try_attach(st) {
            st.msg = e;
            return;
        }
    }
    let mut code = 0u32;
    if GetExitCodeProcess(st.hproc, &mut code) == 0 || code != STILL_ACTIVE {
        detach(st);
        st.toggles = [false; 10];
        st.msg = "游戏已退出，已自动关闭全部功能，等待重新启动…".to_string();
        return;
    }

    let t = st.toggles;
    let health = t[0];
    let speed_on = t[1];
    let ohk = t[2];
    let freeze = t[3];
    let ammo = t[4];
    let items = t[5];
    let defense = t[6];
    let saves = t[7];
    let cooldown = t[8];
    let itembox = t[9];
    let speed = st.speed as f32;

    let need = [
        health || speed_on || freeze,
        health,
        speed_on || freeze,
        ohk,
        ammo,
        items,
        defense,
        saves,
        cooldown,
        itembox,
    ];
    let mut ok = true;
    for i in 0..10 {
        if need[i] {
            ok &= install_hook(st, i);
        } else {
            uninstall_hook(st, i);
        }
    }

    let cave = st.cave;
    let flag: u32 = if health { 1 } else { 0 };
    let pspeed: f32 = if speed_on { speed } else { 1.0 };
    let espeed: f32 = if freeze { 0.0 } else { 1.0 };
    write_mem(st.hproc, cave + 4, &flag.to_le_bytes());
    write_mem(st.hproc, cave + 8, &pspeed.to_le_bytes());
    write_mem(st.hproc, cave + 0xC, &espeed.to_le_bytes());

    if ok {
        st.msg.clear();
    } else {
        let failed: Vec<&str> = (0..10).filter(|&i| need[i] && !st.installed[i]).map(site_name).collect();
        st.msg = format!("写入失败：{}", failed.join("、"));
    }
}

unsafe fn paint(hwnd: HWND) {
    let mut ps: PaintStruct = std::mem::zeroed();
    let hdc = BeginPaint(hwnd, &mut ps);
    let mut rc = Rect { left: 0, top: 0, right: 0, bottom: 0 };
    GetClientRect(hwnd, &mut rc);
    let w = rc.right;
    let h = rc.bottom;
    let mdc = CreateCompatibleDC(hdc);
    let bmp = CreateCompatibleBitmap(hdc, w, h);
    let old_bmp = SelectObject(mdc, bmp);

    fill_rect(mdc, 0, 0, w, h, BG);
    fill_rect(mdc, 0, 0, w, TITLE_H, TITLE_BG);

    let st = STATE.as_ref();
    let (fonts, toggles, speed, hover_row, hover_btn, attached, pid, msg) = match st {
        Some(s) => (s.fonts, s.toggles, s.speed, s.hover_row, s.hover_btn, s.attached, s.pid, s.msg.clone()),
        None => ([std::ptr::null_mut(); 4], [false; 10], 2u32, -1, 0u8, false, 0u32, String::new()),
    };
    let f_header = fonts[0];
    let f_title = fonts[1];
    let f_row = fonts[2];
    let f_small = fonts[3];

    // title bar
    draw_text(mdc, 16, 13, "RE HD Remaster 修改器", TEXT, f_title);
    let ver = "2026-09-28 版";
    draw_text(mdc, w - 186, 16, ver, MUTED, f_small);
    if hover_btn == 1 {
        round_rect(mdc, w - 78, 8, 34, 28, 6, PANEL_HI);
    }
    if hover_btn == 2 {
        round_rect(mdc, w - 42, 8, 34, 28, 6, DANGER);
    }
    fill_rect(mdc, w - 68, 21, 14, 2, TEXT);
    draw_text_centered(mdc, w - 25, 10, "✕", TEXT, f_row);

    // header
    fill_rect(mdc, 20, 58, 4, 30, ACCENT);
    draw_text(mdc, 32, 54, "《生化危机 HD 重制版》", TEXT, f_header);
    draw_text(mdc, 33, 92, "十项属性修改器  ·  仅对应 2026-09-28 更新版", ACCENT, f_small);
    fill_rect(mdc, 20, 124, w - 40, 1, LINE);

    // column labels
    draw_text(mdc, 30, 138, "快捷键", MUTED, f_small);
    draw_text(mdc, 110, 138, "对应功能", MUTED, f_small);

    // rows
    for i in 0..10usize {
        let y = ROWS_Y + i as i32 * ROW_STEP;
        let on = toggles[i];
        let hover = hover_row == i as i32;
        let bg = if on { ROW_ON } else if hover { ROW_HOVER } else { ROW_BG };
        round_rect(mdc, ROW_X, y, ROW_W, ROW_H, 8, bg);

        let key = if i == 9 { "F10".to_string() } else { format!("F{}", i + 1) };
        draw_text(mdc, 30, y + 8, &key, if on { ACCENT } else { MUTED }, f_row);
        draw_text(mdc, 110, y + 8, NAMES[i], if on { TEXT } else { TEXT_DIM }, f_row);

        if i == 1 {
            let base = y + 6;
            round_rect(mdc, 286, base, 26, 22, 6, PANEL_HI);
            draw_text_centered(mdc, 299, base + 3, "−", TEXT, f_row);
            round_rect(mdc, 342, base, 26, 22, 6, PANEL_HI);
            draw_text_centered(mdc, 355, base + 3, "+", TEXT, f_row);
            let sv = speed.to_string();
            draw_text_centered(mdc, 327, base + 3, &sv, ACCENT, f_row);
        }

        let sx = SW_X;
        let sy = y + 8;
        round_rect(mdc, sx, sy, SW_W, SW_H, 9, if on { ACCENT } else { SWITCH_OFF });
        let kx = if on { sx + 23 } else { sx + 3 };
        round_rect(mdc, kx, sy + 2, 14, 14, 7, KNOB);
    }

    // footer
    fill_rect(mdc, 20, 524, w - 40, 1, LINE);
    let (status, color) = if attached {
        let count = toggles.iter().filter(|x| **x).count();
        (format!("已连接  bhd.exe   PID {}   ·   已启用 {} 项", pid, count), ACCENT)
    } else {
        let text = if msg.is_empty() { "等待 bhd.exe 启动…".to_string() } else { msg };
        let color = if text.contains("不匹配") || text.contains("失败") { DANGER } else { MUTED };
        (text, color)
    };
    round_rect(mdc, 20, 538, 8, 8, 4, color);
    draw_text(mdc, 34, 532, &status, color, f_small);
    draw_text(mdc, 20, 562, "F1–F10 或点击切换  ·  关闭窗口自动还原所有补丁", MUTED, f_small);

    BitBlt(hdc, 0, 0, w, h, mdc, 0, 0, 0x00CC0020);
    SelectObject(mdc, old_bmp);
    DeleteObject(bmp);
    DeleteDC(mdc);
    EndPaint(hwnd, &ps);
}

unsafe fn hit_test(hwnd: HWND, x: i32, y: i32) -> LRESULT {
    let mut p = Point { x, y };
    ScreenToClient(hwnd, &mut p);
    if p.y < TITLE_H && p.y >= 0 {
        if p.x >= WIN_W - 80 {
            return HTCLIENT;
        }
        return HTCAPTION;
    }
    HTCLIENT
}

unsafe fn handle_click(hwnd: HWND, x: i32, y: i32) {
    if y >= 0 && y < TITLE_H {
        if x >= WIN_W - 40 {
            if let Some(st) = STATE.as_mut() {
                detach(st);
            }
            DestroyWindow(hwnd);
        } else if x >= WIN_W - 80 {
            ShowWindow(hwnd, SW_MINIMIZE);
        }
        return;
    }
    if y >= ROWS_Y && y < ROWS_Y + 10 * ROW_STEP {
        let i = ((y - ROWS_Y) / ROW_STEP) as usize;
        if i == 1 && x >= 286 && x < 312 {
            if let Some(st) = STATE.as_mut() {
                st.speed = st.speed.saturating_sub(1).max(1);
            }
            sync();
            InvalidateRect(hwnd, std::ptr::null(), 0);
            return;
        }
        if i == 1 && x >= 342 && x < 368 {
            if let Some(st) = STATE.as_mut() {
                st.speed = (st.speed + 1).min(10);
            }
            sync();
            InvalidateRect(hwnd, std::ptr::null(), 0);
            return;
        }
        let on = {
            let Some(st) = STATE.as_mut() else { return };
            st.toggles[i] = !st.toggles[i];
            st.toggles[i]
        };
        sync();
        action_sound(i as i32, on);
        InvalidateRect(hwnd, std::ptr::null(), 0);
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            STATE = Some(State {
                hwnd,
                hproc: std::ptr::null_mut(),
                pid: 0,
                attached: false,
                cave: 0,
                installed: [false; 10],
                toggles: [false; 10],
                speed: 2,
                hover_row: -1,
                hover_btn: 0,
                msg: String::new(),
                fonts: [make_font(26, 700), make_font(19, 700), make_font(16, 400), make_font(13, 400)],
            });
            SetTimer(hwnd, 1, 250, std::ptr::null_mut());
            for i in 0..10 {
                RegisterHotKey(hwnd, (i + 1) as i32, 0, 0x70 + i as u32); // F1..F10
            }
            0
        }
        WM_PAINT => {
            paint(hwnd);
            0
        }
        WM_ERASEBKGND => 1,
        WM_TIMER => {
            sync();
            InvalidateRect(hwnd, std::ptr::null(), 0);
            0
        }
        WM_MOUSEMOVE => {
            let x = (lparam & 0xFFFF) as u16 as i16 as i32;
            let y = ((lparam >> 16) & 0xFFFF) as u16 as i16 as i32;
            let row = if y >= ROWS_Y && y < ROWS_Y + 10 * ROW_STEP {
                ((y - ROWS_Y) / ROW_STEP) as i32
            } else {
                -1
            };
            let btn = if y >= 0 && y < TITLE_H {
                if x >= WIN_W - 40 { 2 } else if x >= WIN_W - 80 { 1 } else { 0 }
            } else {
                0
            };
            if let Some(st) = STATE.as_mut() {
                if st.hover_row != row || st.hover_btn != btn {
                    st.hover_row = row;
                    st.hover_btn = btn;
                    InvalidateRect(hwnd, std::ptr::null(), 0);
                }
            }
            0
        }
        WM_LBUTTONUP => {
            let x = (lparam & 0xFFFF) as u16 as i16 as i32;
            let y = ((lparam >> 16) & 0xFFFF) as u16 as i16 as i32;
            handle_click(hwnd, x, y);
            0
        }
        WM_SETCURSOR => {
            let hit = (lparam & 0xFFFF) as u16 as i32;
            if hit == HTCLIENT as i32 {
                let mut p = Point { x: 0, y: 0 };
                GetCursorPos(&mut p);
                ScreenToClient(hwnd, &mut p);
                let interactive = (p.y >= 0 && p.y < TITLE_H && p.x >= WIN_W - 80)
                    || (p.y >= ROWS_Y && p.y < ROWS_Y + 10 * ROW_STEP);
                let id = if interactive { 32649usize } else { 32512usize };
                let cur = LoadCursorW(std::ptr::null_mut(), id as *const u16);
                SetCursor(cur);
                return 1;
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_NCHITTEST => {
            let sx = (lparam & 0xFFFF) as u16 as i16 as i32;
            let sy = ((lparam >> 16) & 0xFFFF) as u16 as i16 as i32;
            hit_test(hwnd, sx, sy)
        }
        WM_HOTKEY => {
            let id = wparam as i32;
            if (1..=10).contains(&id) {
                let on = {
                    let Some(st) = STATE.as_mut() else { return 0 };
                    st.toggles[id as usize - 1] = !st.toggles[id as usize - 1];
                    st.toggles[id as usize - 1]
                };
                sync();
                action_sound(id - 1, on);
                InvalidateRect(hwnd, std::ptr::null(), 0);
            }
            0
        }
        WM_CLOSE => {
            if let Some(st) = STATE.as_mut() {
                detach(st);
            }
            DestroyWindow(hwnd);
            0
        }
        WM_DESTROY => {
            if let Some(st) = STATE.as_mut() {
                for f in st.fonts.iter() {
                    if !f.is_null() {
                        DeleteObject(*f);
                    }
                }
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() {
    unsafe {
        let instance = GetModuleHandleW(std::ptr::null());
        let icon_file = std::env::temp_dir().join(format!("rehd_trainer_icon_{}.ico", std::process::id()));
        let _ = std::fs::write(&icon_file, ICON_ICO);
        let icon_path_text = icon_file.to_string_lossy().to_string();
        let icon_path_w = wide(&icon_path_text);
        let icon_big = LoadImageW(std::ptr::null_mut(), icon_path_w.as_ptr(), IMAGE_ICON, 48, 48, LR_LOADFROMFILE);
        let icon_small = LoadImageW(std::ptr::null_mut(), icon_path_w.as_ptr(), IMAGE_ICON, 16, 16, LR_LOADFROMFILE);
        let _ = std::fs::remove_file(&icon_file);
        let class_name = wide("REHDTrainerWindow");
        let wc = WndClassW {
            style: 0,
            lpfn_wndproc: Some(wndproc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: instance,
            h_icon: icon_big,
            h_cursor: LoadCursorW(std::ptr::null_mut(), 32512 as *const u16),
            hbr_background: std::ptr::null_mut(),
            lpsz_menu_name: std::ptr::null(),
            lpsz_class_name: class_name.as_ptr(),
        };
        if RegisterClassW(&wc) == 0 {
            return;
        }
        let title = wide("RE HD Remaster 修改器");
        let sw = GetSystemMetrics(0);
        let sh = GetSystemMetrics(1);
        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_POPUP | WS_SYSMENU | WS_MINIMIZEBOX,
            (sw - WIN_W) / 2,
            (sh - WIN_H) / 2,
            WIN_W,
            WIN_H,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null_mut(),
        );
        if hwnd.is_null() {
            return;
        }
        ShowWindow(hwnd, 1);
        if !icon_big.is_null() {
            SendMessageW(hwnd, WM_SETICON, 1, icon_big as isize);
            SetClassLongPtrW(hwnd, GCLP_HICON, icon_big as isize);
        }
        if !icon_small.is_null() {
            SendMessageW(hwnd, WM_SETICON, 0, icon_small as isize);
            SetClassLongPtrW(hwnd, GCLP_HICONSM, icon_small as isize);
        }
        UpdateWindow(hwnd);
        let mut msg: Msg = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
