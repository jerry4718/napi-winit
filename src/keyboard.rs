use proc::{mapping_bitflags, mapping_enum, simple_enum};
use crate::extra::convert::{ExInto};

use winit::keyboard:: {
    NativeKeyCode as OriginNativeKeyCode,
    NativeKey as OriginNativeKey,
    Key as OriginKey,
    KeyCode as OriginKeyCode,
    NamedKey as OriginNamedKey,
    KeyLocation as OriginKeyLocation,
    ModifiersState as OriginModifiersState,
    ModifiersKeyState as OriginModifiersKeyState,
    SmolStr,
};

use napi::bindgen_prelude::*;
use napi::{JsObject, NapiRaw, NapiValue};
use napi::sys::{napi_env, napi_value};
use crate::mark_ex_into;

mapping_enum!(
    enum NativeKeyCode {
        Unidentified,
        Android(u32),
        MacOS(u16),
        Windows(u16),
        Xkb(u32),
    }
);

mapping_enum!(
    enum NativeKey {
        Unidentified,
        Android(u32),
        MacOS(u16),
        Windows(u16),
        Xkb(u32),
        Web(#[conf_trans_type = String] SmolStr),
    }
);

mapping_enum!(
    enum Key<SmolStr> {
        Named(NamedKey),
        Character(#[conf_trans_type = String] SmolStr),
        Unidentified(NativeKey),
        Dead(#[conf_trans_type = Option::<String>] Option<char>),
    }
);

simple_enum!(
    enum KeyCode { Backquote, Backslash, BracketLeft, BracketRight, Comma, Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Equal, IntlBackslash, IntlRo, IntlYen, KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ, Minus, Period, Quote, Semicolon, Slash, AltLeft, AltRight, Backspace, CapsLock, ContextMenu, ControlLeft, ControlRight, Enter, SuperLeft, SuperRight, ShiftLeft, ShiftRight, Space, Tab, Convert, KanaMode, Lang1, Lang2, Lang3, Lang4, Lang5, NonConvert, Delete, End, Help, Home, Insert, PageDown, PageUp, ArrowDown, ArrowLeft, ArrowRight, ArrowUp, NumLock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, NumpadAdd, NumpadBackspace, NumpadClear, NumpadClearEntry, NumpadComma, NumpadDecimal, NumpadDivide, NumpadEnter, NumpadEqual, NumpadHash, NumpadMemoryAdd, NumpadMemoryClear, NumpadMemoryRecall, NumpadMemoryStore, NumpadMemorySubtract, NumpadMultiply, NumpadParenLeft, NumpadParenRight, NumpadStar, NumpadSubtract, Escape, Fn, FnLock, PrintScreen, ScrollLock, Pause, BrowserBack, BrowserFavorites, BrowserForward, BrowserHome, BrowserRefresh, BrowserSearch, BrowserStop, Eject, LaunchApp1, LaunchApp2, LaunchMail, MediaPlayPause, MediaSelect, MediaStop, MediaTrackNext, MediaTrackPrevious, Power, Sleep, AudioVolumeDown, AudioVolumeMute, AudioVolumeUp, WakeUp, Meta, Hyper, Turbo, Abort, Resume, Suspend, Again, Copy, Cut, Find, Open, Paste, Props, Select, Undo, Hiragana, Katakana, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35 }
);

simple_enum!(
    enum NamedKey { Alt, AltGraph, CapsLock, Control, Fn, FnLock, NumLock, ScrollLock, Shift, Symbol, SymbolLock, Meta, Hyper, Super, Enter, Tab, Space, ArrowDown, ArrowLeft, ArrowRight, ArrowUp, End, Home, PageDown, PageUp, Backspace, Clear, Copy, CrSel, Cut, Delete, EraseEof, ExSel, Insert, Paste, Redo, Undo, Accept, Again, Attn, Cancel, ContextMenu, Escape, Execute, Find, Help, Pause, Play, Props, Select, ZoomIn, ZoomOut, BrightnessDown, BrightnessUp, Eject, LogOff, Power, PowerOff, PrintScreen, Hibernate, Standby, WakeUp, AllCandidates, Alphanumeric, CodeInput, Compose, Convert, FinalMode, GroupFirst, GroupLast, GroupNext, GroupPrevious, ModeChange, NextCandidate, NonConvert, PreviousCandidate, Process, SingleCandidate, HangulMode, HanjaMode, JunjaMode, Eisu, Hankaku, Hiragana, HiraganaKatakana, KanaMode, KanjiMode, Katakana, Romaji, Zenkaku, ZenkakuHankaku, Soft1, Soft2, Soft3, Soft4, ChannelDown, ChannelUp, Close, MailForward, MailReply, MailSend, MediaClose, MediaFastForward, MediaPause, MediaPlay, MediaPlayPause, MediaRecord, MediaRewind, MediaStop, MediaTrackNext, MediaTrackPrevious, New, Open, Print, Save, SpellCheck, Key11, Key12, AudioBalanceLeft, AudioBalanceRight, AudioBassBoostDown, AudioBassBoostToggle, AudioBassBoostUp, AudioFaderFront, AudioFaderRear, AudioSurroundModeNext, AudioTrebleDown, AudioTrebleUp, AudioVolumeDown, AudioVolumeUp, AudioVolumeMute, MicrophoneToggle, MicrophoneVolumeDown, MicrophoneVolumeUp, MicrophoneVolumeMute, SpeechCorrectionList, SpeechInputToggle, LaunchApplication1, LaunchApplication2, LaunchCalendar, LaunchContacts, LaunchMail, LaunchMediaPlayer, LaunchMusicPlayer, LaunchPhone, LaunchScreenSaver, LaunchSpreadsheet, LaunchWebBrowser, LaunchWebCam, LaunchWordProcessor, BrowserBack, BrowserFavorites, BrowserForward, BrowserHome, BrowserRefresh, BrowserSearch, BrowserStop, AppSwitch, Call, Camera, CameraFocus, EndCall, GoBack, GoHome, HeadsetHook, LastNumberRedial, Notification, MannerMode, VoiceDial, TV, TV3DMode, TVAntennaCable, TVAudioDescription, TVAudioDescriptionMixDown, TVAudioDescriptionMixUp, TVContentsMenu, TVDataService, TVInput, TVInputComponent1, TVInputComponent2, TVInputComposite1, TVInputComposite2, TVInputHDMI1, TVInputHDMI2, TVInputHDMI3, TVInputHDMI4, TVInputVGA1, TVMediaContext, TVNetwork, TVNumberEntry, TVPower, TVRadioService, TVSatellite, TVSatelliteBS, TVSatelliteCS, TVSatelliteToggle, TVTerrestrialAnalog, TVTerrestrialDigital, TVTimer, AVRInput, AVRPower, ColorF0Red, ColorF1Green, ColorF2Yellow, ColorF3Blue, ColorF4Grey, ColorF5Brown, ClosedCaptionToggle, Dimmer, DisplaySwap, DVR, Exit, FavoriteClear0, FavoriteClear1, FavoriteClear2, FavoriteClear3, FavoriteRecall0, FavoriteRecall1, FavoriteRecall2, FavoriteRecall3, FavoriteStore0, FavoriteStore1, FavoriteStore2, FavoriteStore3, Guide, GuideNextDay, GuidePreviousDay, Info, InstantReplay, Link, ListProgram, LiveContent, Lock, MediaApps, MediaAudioTrack, MediaLast, MediaSkipBackward, MediaSkipForward, MediaStepBackward, MediaStepForward, MediaTopMenu, NavigateIn, NavigateNext, NavigateOut, NavigatePrevious, NextFavoriteChannel, NextUserProfile, OnDemand, Pairing, PinPDown, PinPMove, PinPToggle, PinPUp, PlaySpeedDown, PlaySpeedReset, PlaySpeedUp, RandomToggle, RcLowBattery, RecordSpeedNext, RfBypass, ScanChannelsToggle, ScreenModeNext, Settings, SplitScreenToggle, STBInput, STBPower, Subtitle, Teletext, VideoModeNext, Wink, ZoomToggle, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35 }
);

simple_enum!(
    enum KeyLocation {
        Standard,
        Left,
        Right,
        Numpad,
    }
);

mapping_bitflags!(ModifiersState: SHIFT; CONTROL; ALT; SUPER);

simple_enum!(
    enum ModifiersKeyState {
        Pressed,
        Unknown,
    }
);

mark_ex_into!(
    OriginNativeKeyCode,
    OriginNativeKey,
    OriginKey,
    OriginKeyCode,
    OriginNamedKey,
    OriginKeyLocation,
    OriginModifiersState,
    OriginModifiersKeyState,
    SmolStr,
    // local
    NativeKeyCode,
    NativeKey,
    Key,
    KeyCode,
    NamedKey,
    KeyLocation,
    ModifiersState,
    ModifiersKeyState
);