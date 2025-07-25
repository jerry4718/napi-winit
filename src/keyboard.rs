use napi::bindgen_prelude::*;

use winit::keyboard::SmolStr;

use proc::{proxy_enum, proxy_flags};

use crate::utils::helpers::to_option_string;

#[proxy_enum(origin_enum = winit::keyboard::NativeKeyCode, skip_backward)]
pub enum NativeKeyCode {
    Unidentified,
    Android(#[proxy_enum(field_name = "code")] u32),
    MacOS(#[proxy_enum(field_name = "code")] u16),
    Windows(#[proxy_enum(field_name = "code")] u16),
    Xkb(#[proxy_enum(field_name = "code")] u32),
}

#[proxy_enum(origin_enum = winit::keyboard::NativeKey, skip_backward)]
pub enum NativeKey {
    Unidentified,
    Android(#[proxy_enum(field_name = "code")] u32),
    MacOS(#[proxy_enum(field_name = "code")] u16),
    Windows(#[proxy_enum(field_name = "code")] u16),
    Xkb(#[proxy_enum(field_name = "code")] u32),
    Web(#[proxy_enum(field_name = "code")] String),
}

#[proxy_enum(origin_enum = winit::keyboard::Key::<SmolStr>, skip_backward)]
pub enum Key {
    Named(#[proxy_enum(field_name = "name")] NamedKey),
    Character(#[proxy_enum(field_name = "ch")] String),
    Unidentified(#[proxy_enum(field_name = "ch")] NativeKey),
    Dead(#[proxy_enum(field_name = "ch", from_origin = to_option_string)] Option<String>),
}

#[proxy_enum(origin_enum = winit::keyboard::PhysicalKey, skip_backward)]
pub enum PhysicalKey {
    Code(KeyCode),
    Unidentified(NativeKeyCode),
}

#[proxy_enum(origin_enum = winit::keyboard::KeyCode, string_enum, skip_backward, non_exhaustive)]
#[derive(Clone)]
pub enum KeyCode {
    Backquote, Backslash, BracketLeft, BracketRight, Comma,
    Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
    Equal, IntlBackslash, IntlRo, IntlYen,
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG,
    KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN,
    KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT,
    KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    Minus, Period, Quote, Semicolon, Slash, AltLeft, AltRight, Backspace, CapsLock, ContextMenu,
    ControlLeft, ControlRight, Enter, SuperLeft, SuperRight, ShiftLeft, ShiftRight,
    Space, Tab, Convert, KanaMode,
    Lang1, Lang2, Lang3, Lang4, Lang5,
    NonConvert, Delete, End, Help, Home, Insert, PageDown, PageUp,
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp,
    NumLock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadBackspace, NumpadClear, NumpadClearEntry, NumpadComma, NumpadDecimal,
    NumpadDivide, NumpadEnter, NumpadEqual, NumpadHash, NumpadMemoryAdd, NumpadMemoryClear,
    NumpadMemoryRecall, NumpadMemoryStore, NumpadMemorySubtract, NumpadMultiply, NumpadParenLeft,
    NumpadParenRight, NumpadStar, NumpadSubtract, Escape, Fn, FnLock, PrintScreen, ScrollLock, Pause,
    BrowserBack, BrowserFavorites, BrowserForward, BrowserHome, BrowserRefresh, BrowserSearch,
    BrowserStop, Eject, LaunchApp1, LaunchApp2, LaunchMail, MediaPlayPause, MediaSelect, MediaStop,
    MediaTrackNext, MediaTrackPrevious, Power, Sleep, AudioVolumeDown, AudioVolumeMute, AudioVolumeUp,
    WakeUp, Meta, Hyper, Turbo, Abort, Resume, Suspend, Again, Copy, Cut, Find, Open, Paste, Props,
    Select, Undo, Hiragana, Katakana,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20,
    F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35
}

#[proxy_enum(origin_enum = winit::keyboard::NamedKey, string_enum, skip_backward, non_exhaustive)]
#[derive(Clone)]
pub enum NamedKey {
    Alt, AltGraph, CapsLock, Control, Fn, FnLock, NumLock, ScrollLock, Shift, Symbol, SymbolLock,
    Meta, Hyper, Super, Enter, Tab, Space, ArrowDown, ArrowLeft, ArrowRight, ArrowUp, End, Home,
    PageDown, PageUp, Backspace, Clear, Copy, CrSel, Cut, Delete, EraseEof, ExSel, Insert, Paste,
    Redo, Undo, Accept, Again, Attn, Cancel, ContextMenu, Escape, Execute, Find, Help, Pause, Play,
    Props, Select, ZoomIn, ZoomOut, BrightnessDown, BrightnessUp, Eject, LogOff, Power, PowerOff,
    PrintScreen, Hibernate, Standby, WakeUp, AllCandidates, Alphanumeric, CodeInput, Compose, Convert,
    FinalMode, GroupFirst, GroupLast, GroupNext, GroupPrevious, ModeChange, NextCandidate, NonConvert,
    PreviousCandidate, Process, SingleCandidate, HangulMode, HanjaMode, JunjaMode, Eisu, Hankaku,
    Hiragana, HiraganaKatakana, KanaMode, KanjiMode, Katakana, Romaji, Zenkaku, ZenkakuHankaku, Soft1,
    Soft2, Soft3, Soft4, ChannelDown, ChannelUp, Close, MailForward, MailReply, MailSend, MediaClose,
    MediaFastForward, MediaPause, MediaPlay, MediaPlayPause, MediaRecord, MediaRewind, MediaStop,
    MediaTrackNext, MediaTrackPrevious, New, Open, Print, Save, SpellCheck, Key11, Key12,
    AudioBalanceLeft, AudioBalanceRight, AudioBassBoostDown, AudioBassBoostToggle, AudioBassBoostUp,
    AudioFaderFront, AudioFaderRear, AudioSurroundModeNext, AudioTrebleDown, AudioTrebleUp,
    AudioVolumeDown, AudioVolumeUp, AudioVolumeMute, MicrophoneToggle, MicrophoneVolumeDown,
    MicrophoneVolumeUp, MicrophoneVolumeMute, SpeechCorrectionList, SpeechInputToggle,
    LaunchApplication1, LaunchApplication2, LaunchCalendar, LaunchContacts, LaunchMail,
    LaunchMediaPlayer, LaunchMusicPlayer, LaunchPhone, LaunchScreenSaver, LaunchSpreadsheet,
    LaunchWebBrowser, LaunchWebCam, LaunchWordProcessor, BrowserBack, BrowserFavorites,
    BrowserForward, BrowserHome, BrowserRefresh, BrowserSearch, BrowserStop, AppSwitch, Call, Camera,
    CameraFocus, EndCall, GoBack, GoHome, HeadsetHook, LastNumberRedial, Notification, MannerMode,
    VoiceDial, TV, TV3DMode, TVAntennaCable, TVAudioDescription, TVAudioDescriptionMixDown,
    TVAudioDescriptionMixUp, TVContentsMenu, TVDataService, TVInput, TVInputComponent1,
    TVInputComponent2, TVInputComposite1, TVInputComposite2, TVInputHDMI1, TVInputHDMI2, TVInputHDMI3,
    TVInputHDMI4, TVInputVGA1, TVMediaContext, TVNetwork, TVNumberEntry, TVPower, TVRadioService,
    TVSatellite, TVSatelliteBS, TVSatelliteCS, TVSatelliteToggle, TVTerrestrialAnalog,
    TVTerrestrialDigital, TVTimer, AVRInput, AVRPower, ColorF0Red, ColorF1Green, ColorF2Yellow,
    ColorF3Blue, ColorF4Grey, ColorF5Brown, ClosedCaptionToggle, Dimmer, DisplaySwap, DVR, Exit,
    FavoriteClear0, FavoriteClear1, FavoriteClear2, FavoriteClear3, FavoriteRecall0, FavoriteRecall1,
    FavoriteRecall2, FavoriteRecall3, FavoriteStore0, FavoriteStore1, FavoriteStore2, FavoriteStore3,
    Guide, GuideNextDay, GuidePreviousDay, Info, InstantReplay, Link, ListProgram, LiveContent, Lock,
    MediaApps, MediaAudioTrack, MediaLast, MediaSkipBackward, MediaSkipForward, MediaStepBackward,
    MediaStepForward, MediaTopMenu, NavigateIn, NavigateNext, NavigateOut, NavigatePrevious,
    NextFavoriteChannel, NextUserProfile, OnDemand, Pairing, PinPDown, PinPMove, PinPToggle, PinPUp,
    PlaySpeedDown, PlaySpeedReset, PlaySpeedUp, RandomToggle, RcLowBattery, RecordSpeedNext, RfBypass,
    ScanChannelsToggle, ScreenModeNext, Settings, SplitScreenToggle, STBInput, STBPower, Subtitle,
    Teletext, VideoModeNext, Wink, ZoomToggle,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20,
    F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35
}

#[proxy_enum(origin_enum = winit::keyboard::KeyLocation, string_enum, skip_backward)]
pub enum KeyLocation { Standard, Left, Right, Numpad }

#[proxy_flags(origin = winit::keyboard::ModifiersState, flags = (SHIFT, CONTROL, ALT, SUPER))]
pub struct ModifiersState;

#[proxy_enum(origin_enum = winit::keyboard::ModifiersKeyState, string_enum, skip_backward)]
pub enum ModifiersKeyState { Pressed, Unknown }