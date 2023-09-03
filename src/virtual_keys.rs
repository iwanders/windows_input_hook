#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct VirtualKey {
    pub id: u8,
    pub name: &'static str,
    pub description: &'static str,
}

/*
// from https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
import re
el = t.split("</tr>")
for k in el:
    c = re.findall("<code>(.*)</code>", k)
    t = re.findall("<td.*?>(.*?)</td>", k)
    # print(t)
    if len(c) >= 1 and len(t) >= 1:
        z = t[1]
        name = c[0].replace("VK_", "");
        desc = t[2]
        print("VirtualKey {id: "+ z + ", name: \"" + name + "\", description: \"" + desc + "\"},")

*/
pub static VIRTUAL_KEYS: [VirtualKey; 143] = [
VirtualKey {id: 0x01, name: "LBUTTON", description: "Left mouse button"},
VirtualKey {id: 0x02, name: "RBUTTON", description: "Right mouse button"},
VirtualKey {id: 0x03, name: "CANCEL", description: "Control-break processing"},
VirtualKey {id: 0x04, name: "MBUTTON", description: "Middle mouse button (three-button mouse)"},
VirtualKey {id: 0x05, name: "XBUTTON1", description: "X1 mouse button"},
VirtualKey {id: 0x06, name: "XBUTTON2", description: "X2 mouse button"},
VirtualKey {id: 0x07, name: "-", description: "Undefined"},
VirtualKey {id: 0x08, name: "BACK", description: "BACKSPACE key"},
VirtualKey {id: 0x09, name: "TAB", description: "TAB key"},
// VirtualKey {id: 0x0A-0B, name: "-", description: "Reserved"},
VirtualKey {id: 0x0C, name: "CLEAR", description: "CLEAR key"},
VirtualKey {id: 0x0D, name: "RETURN", description: "ENTER key"},
// VirtualKey {id: 0x0E-0F, name: "-", description: "Undefined"},
VirtualKey {id: 0x10, name: "SHIFT", description: "SHIFT key"},
VirtualKey {id: 0x11, name: "CONTROL", description: "CTRL key"},
VirtualKey {id: 0x12, name: "MENU", description: "ALT key"},
VirtualKey {id: 0x13, name: "PAUSE", description: "PAUSE key"},
VirtualKey {id: 0x14, name: "CAPITAL", description: "CAPS LOCK key"},
VirtualKey {id: 0x15, name: "KANA", description: "IME Kana mode"},
VirtualKey {id: 0x15, name: "HANGUEL", description: "IME Hanguel mode (maintained for compatibility; use <code>VK_HANGUL</code>)"},
VirtualKey {id: 0x15, name: "HANGUL", description: "IME Hangul mode"},
VirtualKey {id: 0x16, name: "IME_ON", description: "IME On"},
VirtualKey {id: 0x17, name: "JUNJA", description: "IME Junja mode"},
VirtualKey {id: 0x18, name: "FINAL", description: "IME final mode"},
VirtualKey {id: 0x19, name: "HANJA", description: "IME Hanja mode"},
VirtualKey {id: 0x19, name: "KANJI", description: "IME Kanji mode"},
VirtualKey {id: 0x1A, name: "IME_OFF", description: "IME Off"},
VirtualKey {id: 0x1B, name: "ESCAPE", description: "ESC key"},
VirtualKey {id: 0x1C, name: "CONVERT", description: "IME convert"},
VirtualKey {id: 0x1D, name: "NONCONVERT", description: "IME nonconvert"},
VirtualKey {id: 0x1E, name: "ACCEPT", description: "IME accept"},
VirtualKey {id: 0x1F, name: "MODECHANGE", description: "IME mode change request"},
VirtualKey {id: 0x20, name: "SPACE", description: "SPACEBAR"},
VirtualKey {id: 0x21, name: "PRIOR", description: "PAGE UP key"},
VirtualKey {id: 0x22, name: "NEXT", description: "PAGE DOWN key"},
VirtualKey {id: 0x23, name: "END", description: "END key"},
VirtualKey {id: 0x24, name: "HOME", description: "HOME key"},
VirtualKey {id: 0x25, name: "LEFT", description: "LEFT ARROW key"},
VirtualKey {id: 0x26, name: "UP", description: "UP ARROW key"},
VirtualKey {id: 0x27, name: "RIGHT", description: "RIGHT ARROW key"},
VirtualKey {id: 0x28, name: "DOWN", description: "DOWN ARROW key"},
VirtualKey {id: 0x29, name: "SELECT", description: "SELECT key"},
VirtualKey {id: 0x2A, name: "PRINT", description: "PRINT key"},
VirtualKey {id: 0x2B, name: "EXECUTE", description: "EXECUTE key"},
VirtualKey {id: 0x2C, name: "SNAPSHOT", description: "PRINT SCREEN key"},
VirtualKey {id: 0x2D, name: "INSERT", description: "INS key"},
VirtualKey {id: 0x2E, name: "DELETE", description: "DEL key"},
VirtualKey {id: 0x2F, name: "HELP", description: "HELP key"},
// VirtualKey {id: 0x3A-40, name: "-", description: "Undefined"},
VirtualKey {id: 0x5B, name: "LWIN", description: "Left Windows key (Natural keyboard)"},
VirtualKey {id: 0x5C, name: "RWIN", description: "Right Windows key (Natural keyboard)"},
VirtualKey {id: 0x5D, name: "APPS", description: "Applications key (Natural keyboard)"},
VirtualKey {id: 0x5E, name: "-", description: "Reserved"},
VirtualKey {id: 0x5F, name: "SLEEP", description: "Computer Sleep key"},
VirtualKey {id: 0x60, name: "NUMPAD0", description: "Numeric keypad 0 key"},
VirtualKey {id: 0x61, name: "NUMPAD1", description: "Numeric keypad 1 key"},
VirtualKey {id: 0x62, name: "NUMPAD2", description: "Numeric keypad 2 key"},
VirtualKey {id: 0x63, name: "NUMPAD3", description: "Numeric keypad 3 key"},
VirtualKey {id: 0x64, name: "NUMPAD4", description: "Numeric keypad 4 key"},
VirtualKey {id: 0x65, name: "NUMPAD5", description: "Numeric keypad 5 key"},
VirtualKey {id: 0x66, name: "NUMPAD6", description: "Numeric keypad 6 key"},
VirtualKey {id: 0x67, name: "NUMPAD7", description: "Numeric keypad 7 key"},
VirtualKey {id: 0x68, name: "NUMPAD8", description: "Numeric keypad 8 key"},
VirtualKey {id: 0x69, name: "NUMPAD9", description: "Numeric keypad 9 key"},
VirtualKey {id: 0x6A, name: "MULTIPLY", description: "Multiply key"},
VirtualKey {id: 0x6B, name: "ADD", description: "Add key"},
VirtualKey {id: 0x6C, name: "SEPARATOR", description: "Separator key"},
VirtualKey {id: 0x6D, name: "SUBTRACT", description: "Subtract key"},
VirtualKey {id: 0x6E, name: "DECIMAL", description: "Decimal key"},
VirtualKey {id: 0x6F, name: "DIVIDE", description: "Divide key"},
VirtualKey {id: 0x70, name: "F1", description: "F1 key"},
VirtualKey {id: 0x71, name: "F2", description: "F2 key"},
VirtualKey {id: 0x72, name: "F3", description: "F3 key"},
VirtualKey {id: 0x73, name: "F4", description: "F4 key"},
VirtualKey {id: 0x74, name: "F5", description: "F5 key"},
VirtualKey {id: 0x75, name: "F6", description: "F6 key"},
VirtualKey {id: 0x76, name: "F7", description: "F7 key"},
VirtualKey {id: 0x77, name: "F8", description: "F8 key"},
VirtualKey {id: 0x78, name: "F9", description: "F9 key"},
VirtualKey {id: 0x79, name: "F10", description: "F10 key"},
VirtualKey {id: 0x7A, name: "F11", description: "F11 key"},
VirtualKey {id: 0x7B, name: "F12", description: "F12 key"},
VirtualKey {id: 0x7C, name: "F13", description: "F13 key"},
VirtualKey {id: 0x7D, name: "F14", description: "F14 key"},
VirtualKey {id: 0x7E, name: "F15", description: "F15 key"},
VirtualKey {id: 0x7F, name: "F16", description: "F16 key"},
VirtualKey {id: 0x80, name: "F17", description: "F17 key"},
VirtualKey {id: 0x81, name: "F18", description: "F18 key"},
VirtualKey {id: 0x82, name: "F19", description: "F19 key"},
VirtualKey {id: 0x83, name: "F20", description: "F20 key"},
VirtualKey {id: 0x84, name: "F21", description: "F21 key"},
VirtualKey {id: 0x85, name: "F22", description: "F22 key"},
VirtualKey {id: 0x86, name: "F23", description: "F23 key"},
VirtualKey {id: 0x87, name: "F24", description: "F24 key"},
// VirtualKey {id: 0x88-8F, name: "-", description: "Unassigned"},
VirtualKey {id: 0x90, name: "NUMLOCK", description: "NUM LOCK key"},
VirtualKey {id: 0x91, name: "SCROLL", description: "SCROLL LOCK key"},
// VirtualKey {id: 0x97-9F, name: "-", description: "Unassigned"},
VirtualKey {id: 0xA0, name: "LSHIFT", description: "Left SHIFT key"},
VirtualKey {id: 0xA1, name: "RSHIFT", description: "Right SHIFT key"},
VirtualKey {id: 0xA2, name: "LCONTROL", description: "Left CONTROL key"},
VirtualKey {id: 0xA3, name: "RCONTROL", description: "Right CONTROL key"},
VirtualKey {id: 0xA4, name: "LMENU", description: "Left ALT key"},
VirtualKey {id: 0xA5, name: "RMENU", description: "Right ALT key"},
VirtualKey {id: 0xA6, name: "BROWSER_BACK", description: "Browser Back key"},
VirtualKey {id: 0xA7, name: "BROWSER_FORWARD", description: "Browser Forward key"},
VirtualKey {id: 0xA8, name: "BROWSER_REFRESH", description: "Browser Refresh key"},
VirtualKey {id: 0xA9, name: "BROWSER_STOP", description: "Browser Stop key"},
VirtualKey {id: 0xAA, name: "BROWSER_SEARCH", description: "Browser Search key"},
VirtualKey {id: 0xAB, name: "BROWSER_FAVORITES", description: "Browser Favorites key"},
VirtualKey {id: 0xAC, name: "BROWSER_HOME", description: "Browser Start and Home key"},
VirtualKey {id: 0xAD, name: "VOLUME_MUTE", description: "Volume Mute key"},
VirtualKey {id: 0xAE, name: "VOLUME_DOWN", description: "Volume Down key"},
VirtualKey {id: 0xAF, name: "VOLUME_UP", description: "Volume Up key"},
VirtualKey {id: 0xB0, name: "MEDIA_NEXT_TRACK", description: "Next Track key"},
VirtualKey {id: 0xB1, name: "MEDIA_PREV_TRACK", description: "Previous Track key"},
VirtualKey {id: 0xB2, name: "MEDIA_STOP", description: "Stop Media key"},
VirtualKey {id: 0xB3, name: "MEDIA_PLAY_PAUSE", description: "Play/Pause Media key"},
VirtualKey {id: 0xB4, name: "LAUNCH_MAIL", description: "Start Mail key"},
VirtualKey {id: 0xB5, name: "LAUNCH_MEDIA_SELECT", description: "Select Media key"},
VirtualKey {id: 0xB6, name: "LAUNCH_APP1", description: "Start Application 1 key"},
VirtualKey {id: 0xB7, name: "LAUNCH_APP2", description: "Start Application 2 key"},
// VirtualKey {id: 0xB8-B9, name: "-", description: "Reserved"},
VirtualKey {id: 0xBA, name: "OEM_1", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ';:' key"},
VirtualKey {id: 0xBB, name: "OEM_PLUS", description: "For any country/region, the '+' key"},
VirtualKey {id: 0xBC, name: "OEM_COMMA", description: "For any country/region, the ',' key"},
VirtualKey {id: 0xBD, name: "OEM_MINUS", description: "For any country/region, the '-' key"},
VirtualKey {id: 0xBE, name: "OEM_PERIOD", description: "For any country/region, the '.' key"},
VirtualKey {id: 0xBF, name: "OEM_2", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '/?' key"},
VirtualKey {id: 0xC0, name: "OEM_3", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key"},
// VirtualKey {id: 0xC1-D7, name: "-", description: "Reserved"},
// VirtualKey {id: 0xD8-DA, name: "-", description: "Unassigned"},
VirtualKey {id: 0xDB, name: "OEM_4", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '[{' key"},
VirtualKey {id: 0xDC, name: "OEM_5", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '\\|' key"},
VirtualKey {id: 0xDD, name: "OEM_6", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ']}' key"},
VirtualKey {id: 0xDE, name: "OEM_7", description: "Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the 'single-quote/double-quote\' key"},
VirtualKey {id: 0xDF, name: "OEM_8", description: "Used for miscellaneous characters; it can vary by keyboard."},
VirtualKey {id: 0xE0, name: "-", description: "Reserved"},
VirtualKey {id: 0xE2, name: "OEM_102", description: "The <code>&lt;&gt;</code> keys on the US  standard keyboard, or the <code>\\|</code> key on the non-US 102-key keyboard"},
VirtualKey {id: 0xE5, name: "PROCESSKEY", description: "IME PROCESS key"},
VirtualKey {id: 0xE7, name: "PACKET", description: "Used to pass Unicode characters as if they were keystrokes. The <code>VK_PACKET</code> key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods."},
VirtualKey {id: 0xE8, name: "-", description: "Unassigned"},
VirtualKey {id: 0xF6, name: "ATTN", description: "Attn key"},
VirtualKey {id: 0xF7, name: "CRSEL", description: "CrSel key"},
VirtualKey {id: 0xF8, name: "EXSEL", description: "ExSel key"},
VirtualKey {id: 0xF9, name: "EREOF", description: "Erase EOF key"},
VirtualKey {id: 0xFA, name: "PLAY", description: "Play key"},
VirtualKey {id: 0xFB, name: "ZOOM", description: "Zoom key"},
VirtualKey {id: 0xFC, name: "NONAME", description: "Reserved"},
VirtualKey {id: 0xFD, name: "PA1", description: "PA1 key"},
VirtualKey {id: 0xFE, name: "OEM_CLEAR", description: "Clear key"},
];
