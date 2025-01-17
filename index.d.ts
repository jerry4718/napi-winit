/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum UnitType {
  Physical = 'Physical',
  Logical = 'Logical'
}
export interface Position {
  type: UnitType
  x: number
  y: number
}
export interface Size {
  type: UnitType
  width: number
  height: number
}
export interface PixelUnit {
  type: UnitType
  count: number
}
export const enum DeviceEventsEnum {
  Always = 'Always',
  WhenFocused = 'WhenFocused',
  Never = 'Never'
}
export const enum ControlFlowEnum {
  Poll = 'Poll',
  Wait = 'Wait',
  WaitUntil = 'WaitUntil'
}
export const enum EventEnum {
  NewEvents = 'NewEvents',
  WindowEvent = 'WindowEvent',
  DeviceEvent = 'DeviceEvent',
  UserEvent = 'UserEvent',
  Suspended = 'Suspended',
  Resumed = 'Resumed',
  AboutToWait = 'AboutToWait',
  LoopExiting = 'LoopExiting',
  MemoryWarning = 'MemoryWarning'
}
export const enum StartCauseEnum {
  ResumeTimeReached = 'ResumeTimeReached',
  WaitCancelled = 'WaitCancelled',
  Poll = 'Poll',
  Init = 'Init'
}
export const enum WindowEventEnum {
  ActivationTokenDone = 'ActivationTokenDone',
  Resized = 'Resized',
  Moved = 'Moved',
  CloseRequested = 'CloseRequested',
  Destroyed = 'Destroyed',
  DroppedFile = 'DroppedFile',
  HoveredFile = 'HoveredFile',
  HoveredFileCancelled = 'HoveredFileCancelled',
  Focused = 'Focused',
  KeyboardInput = 'KeyboardInput',
  ModifiersChanged = 'ModifiersChanged',
  Ime = 'Ime',
  CursorMoved = 'CursorMoved',
  CursorEntered = 'CursorEntered',
  CursorLeft = 'CursorLeft',
  MouseWheel = 'MouseWheel',
  MouseInput = 'MouseInput',
  PinchGesture = 'PinchGesture',
  PanGesture = 'PanGesture',
  DoubleTapGesture = 'DoubleTapGesture',
  RotationGesture = 'RotationGesture',
  TouchpadPressure = 'TouchpadPressure',
  AxisMotion = 'AxisMotion',
  Touch = 'Touch',
  ScaleFactorChanged = 'ScaleFactorChanged',
  ThemeChanged = 'ThemeChanged',
  Occluded = 'Occluded',
  RedrawRequested = 'RedrawRequested'
}
export const enum ImeEnum {
  Enabled = 'Enabled',
  Preedit = 'Preedit',
  Commit = 'Commit',
  Disabled = 'Disabled'
}
export const enum MouseButtonEnum {
  Left = 'Left',
  Right = 'Right',
  Middle = 'Middle',
  Back = 'Back',
  Forward = 'Forward',
  Other = 'Other'
}
export const enum MouseScrollDeltaType {
  Line = 'Line',
  Pixel = 'Pixel'
}
export const enum TouchPhaseEnum {
  Started = 'Started',
  Moved = 'Moved',
  Ended = 'Ended',
  Cancelled = 'Cancelled'
}
export const enum DeviceEventEnum {
  Added = 'Added',
  Removed = 'Removed',
  MouseMotion = 'MouseMotion',
  MouseWheel = 'MouseWheel',
  Motion = 'Motion',
  Button = 'Button',
  Key = 'Key'
}
export const enum ElementState {
  Pressed = 'Pressed',
  Released = 'Released'
}
export const enum Fullscreen {
  Exclusive = 'Exclusive',
  Borderless = 'Borderless'
}
export const enum WindowLevel {
  AlwaysOnBottom = 'AlwaysOnBottom',
  Normal = 'Normal',
  AlwaysOnTop = 'AlwaysOnTop'
}
export const enum Theme {
  Light = 'Light',
  Dark = 'Dark'
}
export const enum ImePurpose {
  Normal = 'Normal',
  Password = 'Password',
  Terminal = 'Terminal'
}
export const enum UserAttentionType {
  Critical = 'Critical',
  Informational = 'Informational'
}
export const enum CursorIcon {
  Default = 'Default',
  ContextMenu = 'ContextMenu',
  Help = 'Help',
  Pointer = 'Pointer',
  Progress = 'Progress',
  Wait = 'Wait',
  Cell = 'Cell',
  Crosshair = 'Crosshair',
  Text = 'Text',
  VerticalText = 'VerticalText',
  Alias = 'Alias',
  Copy = 'Copy',
  Move = 'Move',
  NoDrop = 'NoDrop',
  NotAllowed = 'NotAllowed',
  Grab = 'Grab',
  Grabbing = 'Grabbing',
  EResize = 'EResize',
  NResize = 'NResize',
  NeResize = 'NeResize',
  NwResize = 'NwResize',
  SResize = 'SResize',
  SeResize = 'SeResize',
  SwResize = 'SwResize',
  WResize = 'WResize',
  EwResize = 'EwResize',
  NsResize = 'NsResize',
  NeswResize = 'NeswResize',
  NwseResize = 'NwseResize',
  ColResize = 'ColResize',
  RowResize = 'RowResize',
  AllScroll = 'AllScroll',
  ZoomIn = 'ZoomIn',
  ZoomOut = 'ZoomOut'
}
export const enum NativeKeyCodeEnum {
  Unidentified = 'Unidentified',
  Android = 'Android',
  MacOS = 'MacOS',
  Windows = 'Windows',
  Xkb = 'Xkb'
}
export const enum NativeKeyEnum {
  Unidentified = 'Unidentified',
  Android = 'Android',
  MacOS = 'MacOS',
  Windows = 'Windows',
  Xkb = 'Xkb',
  Web = 'Web'
}
export const enum KeyEnum {
  Named = 'Named',
  Character = 'Character',
  Unidentified = 'Unidentified',
  Dead = 'Dead'
}
export const enum KeyCode {
  Backquote = 'Backquote',
  Backslash = 'Backslash',
  BracketLeft = 'BracketLeft',
  BracketRight = 'BracketRight',
  Comma = 'Comma',
  Digit0 = 'Digit0',
  Digit1 = 'Digit1',
  Digit2 = 'Digit2',
  Digit3 = 'Digit3',
  Digit4 = 'Digit4',
  Digit5 = 'Digit5',
  Digit6 = 'Digit6',
  Digit7 = 'Digit7',
  Digit8 = 'Digit8',
  Digit9 = 'Digit9',
  Equal = 'Equal',
  IntlBackslash = 'IntlBackslash',
  IntlRo = 'IntlRo',
  IntlYen = 'IntlYen',
  KeyA = 'KeyA',
  KeyB = 'KeyB',
  KeyC = 'KeyC',
  KeyD = 'KeyD',
  KeyE = 'KeyE',
  KeyF = 'KeyF',
  KeyG = 'KeyG',
  KeyH = 'KeyH',
  KeyI = 'KeyI',
  KeyJ = 'KeyJ',
  KeyK = 'KeyK',
  KeyL = 'KeyL',
  KeyM = 'KeyM',
  KeyN = 'KeyN',
  KeyO = 'KeyO',
  KeyP = 'KeyP',
  KeyQ = 'KeyQ',
  KeyR = 'KeyR',
  KeyS = 'KeyS',
  KeyT = 'KeyT',
  KeyU = 'KeyU',
  KeyV = 'KeyV',
  KeyW = 'KeyW',
  KeyX = 'KeyX',
  KeyY = 'KeyY',
  KeyZ = 'KeyZ',
  Minus = 'Minus',
  Period = 'Period',
  Quote = 'Quote',
  Semicolon = 'Semicolon',
  Slash = 'Slash',
  AltLeft = 'AltLeft',
  AltRight = 'AltRight',
  Backspace = 'Backspace',
  CapsLock = 'CapsLock',
  ContextMenu = 'ContextMenu',
  ControlLeft = 'ControlLeft',
  ControlRight = 'ControlRight',
  Enter = 'Enter',
  SuperLeft = 'SuperLeft',
  SuperRight = 'SuperRight',
  ShiftLeft = 'ShiftLeft',
  ShiftRight = 'ShiftRight',
  Space = 'Space',
  Tab = 'Tab',
  Convert = 'Convert',
  KanaMode = 'KanaMode',
  Lang1 = 'Lang1',
  Lang2 = 'Lang2',
  Lang3 = 'Lang3',
  Lang4 = 'Lang4',
  Lang5 = 'Lang5',
  NonConvert = 'NonConvert',
  Delete = 'Delete',
  End = 'End',
  Help = 'Help',
  Home = 'Home',
  Insert = 'Insert',
  PageDown = 'PageDown',
  PageUp = 'PageUp',
  ArrowDown = 'ArrowDown',
  ArrowLeft = 'ArrowLeft',
  ArrowRight = 'ArrowRight',
  ArrowUp = 'ArrowUp',
  NumLock = 'NumLock',
  Numpad0 = 'Numpad0',
  Numpad1 = 'Numpad1',
  Numpad2 = 'Numpad2',
  Numpad3 = 'Numpad3',
  Numpad4 = 'Numpad4',
  Numpad5 = 'Numpad5',
  Numpad6 = 'Numpad6',
  Numpad7 = 'Numpad7',
  Numpad8 = 'Numpad8',
  Numpad9 = 'Numpad9',
  NumpadAdd = 'NumpadAdd',
  NumpadBackspace = 'NumpadBackspace',
  NumpadClear = 'NumpadClear',
  NumpadClearEntry = 'NumpadClearEntry',
  NumpadComma = 'NumpadComma',
  NumpadDecimal = 'NumpadDecimal',
  NumpadDivide = 'NumpadDivide',
  NumpadEnter = 'NumpadEnter',
  NumpadEqual = 'NumpadEqual',
  NumpadHash = 'NumpadHash',
  NumpadMemoryAdd = 'NumpadMemoryAdd',
  NumpadMemoryClear = 'NumpadMemoryClear',
  NumpadMemoryRecall = 'NumpadMemoryRecall',
  NumpadMemoryStore = 'NumpadMemoryStore',
  NumpadMemorySubtract = 'NumpadMemorySubtract',
  NumpadMultiply = 'NumpadMultiply',
  NumpadParenLeft = 'NumpadParenLeft',
  NumpadParenRight = 'NumpadParenRight',
  NumpadStar = 'NumpadStar',
  NumpadSubtract = 'NumpadSubtract',
  Escape = 'Escape',
  Fn = 'Fn',
  FnLock = 'FnLock',
  PrintScreen = 'PrintScreen',
  ScrollLock = 'ScrollLock',
  Pause = 'Pause',
  BrowserBack = 'BrowserBack',
  BrowserFavorites = 'BrowserFavorites',
  BrowserForward = 'BrowserForward',
  BrowserHome = 'BrowserHome',
  BrowserRefresh = 'BrowserRefresh',
  BrowserSearch = 'BrowserSearch',
  BrowserStop = 'BrowserStop',
  Eject = 'Eject',
  LaunchApp1 = 'LaunchApp1',
  LaunchApp2 = 'LaunchApp2',
  LaunchMail = 'LaunchMail',
  MediaPlayPause = 'MediaPlayPause',
  MediaSelect = 'MediaSelect',
  MediaStop = 'MediaStop',
  MediaTrackNext = 'MediaTrackNext',
  MediaTrackPrevious = 'MediaTrackPrevious',
  Power = 'Power',
  Sleep = 'Sleep',
  AudioVolumeDown = 'AudioVolumeDown',
  AudioVolumeMute = 'AudioVolumeMute',
  AudioVolumeUp = 'AudioVolumeUp',
  WakeUp = 'WakeUp',
  Meta = 'Meta',
  Hyper = 'Hyper',
  Turbo = 'Turbo',
  Abort = 'Abort',
  Resume = 'Resume',
  Suspend = 'Suspend',
  Again = 'Again',
  Copy = 'Copy',
  Cut = 'Cut',
  Find = 'Find',
  Open = 'Open',
  Paste = 'Paste',
  Props = 'Props',
  Select = 'Select',
  Undo = 'Undo',
  Hiragana = 'Hiragana',
  Katakana = 'Katakana',
  F1 = 'F1',
  F2 = 'F2',
  F3 = 'F3',
  F4 = 'F4',
  F5 = 'F5',
  F6 = 'F6',
  F7 = 'F7',
  F8 = 'F8',
  F9 = 'F9',
  F10 = 'F10',
  F11 = 'F11',
  F12 = 'F12',
  F13 = 'F13',
  F14 = 'F14',
  F15 = 'F15',
  F16 = 'F16',
  F17 = 'F17',
  F18 = 'F18',
  F19 = 'F19',
  F20 = 'F20',
  F21 = 'F21',
  F22 = 'F22',
  F23 = 'F23',
  F24 = 'F24',
  F25 = 'F25',
  F26 = 'F26',
  F27 = 'F27',
  F28 = 'F28',
  F29 = 'F29',
  F30 = 'F30',
  F31 = 'F31',
  F32 = 'F32',
  F33 = 'F33',
  F34 = 'F34',
  F35 = 'F35'
}
export const enum NamedKey {
  Alt = 'Alt',
  AltGraph = 'AltGraph',
  CapsLock = 'CapsLock',
  Control = 'Control',
  Fn = 'Fn',
  FnLock = 'FnLock',
  NumLock = 'NumLock',
  ScrollLock = 'ScrollLock',
  Shift = 'Shift',
  Symbol = 'Symbol',
  SymbolLock = 'SymbolLock',
  Meta = 'Meta',
  Hyper = 'Hyper',
  Super = 'Super',
  Enter = 'Enter',
  Tab = 'Tab',
  Space = 'Space',
  ArrowDown = 'ArrowDown',
  ArrowLeft = 'ArrowLeft',
  ArrowRight = 'ArrowRight',
  ArrowUp = 'ArrowUp',
  End = 'End',
  Home = 'Home',
  PageDown = 'PageDown',
  PageUp = 'PageUp',
  Backspace = 'Backspace',
  Clear = 'Clear',
  Copy = 'Copy',
  CrSel = 'CrSel',
  Cut = 'Cut',
  Delete = 'Delete',
  EraseEof = 'EraseEof',
  ExSel = 'ExSel',
  Insert = 'Insert',
  Paste = 'Paste',
  Redo = 'Redo',
  Undo = 'Undo',
  Accept = 'Accept',
  Again = 'Again',
  Attn = 'Attn',
  Cancel = 'Cancel',
  ContextMenu = 'ContextMenu',
  Escape = 'Escape',
  Execute = 'Execute',
  Find = 'Find',
  Help = 'Help',
  Pause = 'Pause',
  Play = 'Play',
  Props = 'Props',
  Select = 'Select',
  ZoomIn = 'ZoomIn',
  ZoomOut = 'ZoomOut',
  BrightnessDown = 'BrightnessDown',
  BrightnessUp = 'BrightnessUp',
  Eject = 'Eject',
  LogOff = 'LogOff',
  Power = 'Power',
  PowerOff = 'PowerOff',
  PrintScreen = 'PrintScreen',
  Hibernate = 'Hibernate',
  Standby = 'Standby',
  WakeUp = 'WakeUp',
  AllCandidates = 'AllCandidates',
  Alphanumeric = 'Alphanumeric',
  CodeInput = 'CodeInput',
  Compose = 'Compose',
  Convert = 'Convert',
  FinalMode = 'FinalMode',
  GroupFirst = 'GroupFirst',
  GroupLast = 'GroupLast',
  GroupNext = 'GroupNext',
  GroupPrevious = 'GroupPrevious',
  ModeChange = 'ModeChange',
  NextCandidate = 'NextCandidate',
  NonConvert = 'NonConvert',
  PreviousCandidate = 'PreviousCandidate',
  Process = 'Process',
  SingleCandidate = 'SingleCandidate',
  HangulMode = 'HangulMode',
  HanjaMode = 'HanjaMode',
  JunjaMode = 'JunjaMode',
  Eisu = 'Eisu',
  Hankaku = 'Hankaku',
  Hiragana = 'Hiragana',
  HiraganaKatakana = 'HiraganaKatakana',
  KanaMode = 'KanaMode',
  KanjiMode = 'KanjiMode',
  Katakana = 'Katakana',
  Romaji = 'Romaji',
  Zenkaku = 'Zenkaku',
  ZenkakuHankaku = 'ZenkakuHankaku',
  Soft1 = 'Soft1',
  Soft2 = 'Soft2',
  Soft3 = 'Soft3',
  Soft4 = 'Soft4',
  ChannelDown = 'ChannelDown',
  ChannelUp = 'ChannelUp',
  Close = 'Close',
  MailForward = 'MailForward',
  MailReply = 'MailReply',
  MailSend = 'MailSend',
  MediaClose = 'MediaClose',
  MediaFastForward = 'MediaFastForward',
  MediaPause = 'MediaPause',
  MediaPlay = 'MediaPlay',
  MediaPlayPause = 'MediaPlayPause',
  MediaRecord = 'MediaRecord',
  MediaRewind = 'MediaRewind',
  MediaStop = 'MediaStop',
  MediaTrackNext = 'MediaTrackNext',
  MediaTrackPrevious = 'MediaTrackPrevious',
  New = 'New',
  Open = 'Open',
  Print = 'Print',
  Save = 'Save',
  SpellCheck = 'SpellCheck',
  Key11 = 'Key11',
  Key12 = 'Key12',
  AudioBalanceLeft = 'AudioBalanceLeft',
  AudioBalanceRight = 'AudioBalanceRight',
  AudioBassBoostDown = 'AudioBassBoostDown',
  AudioBassBoostToggle = 'AudioBassBoostToggle',
  AudioBassBoostUp = 'AudioBassBoostUp',
  AudioFaderFront = 'AudioFaderFront',
  AudioFaderRear = 'AudioFaderRear',
  AudioSurroundModeNext = 'AudioSurroundModeNext',
  AudioTrebleDown = 'AudioTrebleDown',
  AudioTrebleUp = 'AudioTrebleUp',
  AudioVolumeDown = 'AudioVolumeDown',
  AudioVolumeUp = 'AudioVolumeUp',
  AudioVolumeMute = 'AudioVolumeMute',
  MicrophoneToggle = 'MicrophoneToggle',
  MicrophoneVolumeDown = 'MicrophoneVolumeDown',
  MicrophoneVolumeUp = 'MicrophoneVolumeUp',
  MicrophoneVolumeMute = 'MicrophoneVolumeMute',
  SpeechCorrectionList = 'SpeechCorrectionList',
  SpeechInputToggle = 'SpeechInputToggle',
  LaunchApplication1 = 'LaunchApplication1',
  LaunchApplication2 = 'LaunchApplication2',
  LaunchCalendar = 'LaunchCalendar',
  LaunchContacts = 'LaunchContacts',
  LaunchMail = 'LaunchMail',
  LaunchMediaPlayer = 'LaunchMediaPlayer',
  LaunchMusicPlayer = 'LaunchMusicPlayer',
  LaunchPhone = 'LaunchPhone',
  LaunchScreenSaver = 'LaunchScreenSaver',
  LaunchSpreadsheet = 'LaunchSpreadsheet',
  LaunchWebBrowser = 'LaunchWebBrowser',
  LaunchWebCam = 'LaunchWebCam',
  LaunchWordProcessor = 'LaunchWordProcessor',
  BrowserBack = 'BrowserBack',
  BrowserFavorites = 'BrowserFavorites',
  BrowserForward = 'BrowserForward',
  BrowserHome = 'BrowserHome',
  BrowserRefresh = 'BrowserRefresh',
  BrowserSearch = 'BrowserSearch',
  BrowserStop = 'BrowserStop',
  AppSwitch = 'AppSwitch',
  Call = 'Call',
  Camera = 'Camera',
  CameraFocus = 'CameraFocus',
  EndCall = 'EndCall',
  GoBack = 'GoBack',
  GoHome = 'GoHome',
  HeadsetHook = 'HeadsetHook',
  LastNumberRedial = 'LastNumberRedial',
  Notification = 'Notification',
  MannerMode = 'MannerMode',
  VoiceDial = 'VoiceDial',
  TV = 'TV',
  TV3DMode = 'TV3DMode',
  TVAntennaCable = 'TVAntennaCable',
  TVAudioDescription = 'TVAudioDescription',
  TVAudioDescriptionMixDown = 'TVAudioDescriptionMixDown',
  TVAudioDescriptionMixUp = 'TVAudioDescriptionMixUp',
  TVContentsMenu = 'TVContentsMenu',
  TVDataService = 'TVDataService',
  TVInput = 'TVInput',
  TVInputComponent1 = 'TVInputComponent1',
  TVInputComponent2 = 'TVInputComponent2',
  TVInputComposite1 = 'TVInputComposite1',
  TVInputComposite2 = 'TVInputComposite2',
  TVInputHDMI1 = 'TVInputHDMI1',
  TVInputHDMI2 = 'TVInputHDMI2',
  TVInputHDMI3 = 'TVInputHDMI3',
  TVInputHDMI4 = 'TVInputHDMI4',
  TVInputVGA1 = 'TVInputVGA1',
  TVMediaContext = 'TVMediaContext',
  TVNetwork = 'TVNetwork',
  TVNumberEntry = 'TVNumberEntry',
  TVPower = 'TVPower',
  TVRadioService = 'TVRadioService',
  TVSatellite = 'TVSatellite',
  TVSatelliteBS = 'TVSatelliteBS',
  TVSatelliteCS = 'TVSatelliteCS',
  TVSatelliteToggle = 'TVSatelliteToggle',
  TVTerrestrialAnalog = 'TVTerrestrialAnalog',
  TVTerrestrialDigital = 'TVTerrestrialDigital',
  TVTimer = 'TVTimer',
  AVRInput = 'AVRInput',
  AVRPower = 'AVRPower',
  ColorF0Red = 'ColorF0Red',
  ColorF1Green = 'ColorF1Green',
  ColorF2Yellow = 'ColorF2Yellow',
  ColorF3Blue = 'ColorF3Blue',
  ColorF4Grey = 'ColorF4Grey',
  ColorF5Brown = 'ColorF5Brown',
  ClosedCaptionToggle = 'ClosedCaptionToggle',
  Dimmer = 'Dimmer',
  DisplaySwap = 'DisplaySwap',
  DVR = 'DVR',
  Exit = 'Exit',
  FavoriteClear0 = 'FavoriteClear0',
  FavoriteClear1 = 'FavoriteClear1',
  FavoriteClear2 = 'FavoriteClear2',
  FavoriteClear3 = 'FavoriteClear3',
  FavoriteRecall0 = 'FavoriteRecall0',
  FavoriteRecall1 = 'FavoriteRecall1',
  FavoriteRecall2 = 'FavoriteRecall2',
  FavoriteRecall3 = 'FavoriteRecall3',
  FavoriteStore0 = 'FavoriteStore0',
  FavoriteStore1 = 'FavoriteStore1',
  FavoriteStore2 = 'FavoriteStore2',
  FavoriteStore3 = 'FavoriteStore3',
  Guide = 'Guide',
  GuideNextDay = 'GuideNextDay',
  GuidePreviousDay = 'GuidePreviousDay',
  Info = 'Info',
  InstantReplay = 'InstantReplay',
  Link = 'Link',
  ListProgram = 'ListProgram',
  LiveContent = 'LiveContent',
  Lock = 'Lock',
  MediaApps = 'MediaApps',
  MediaAudioTrack = 'MediaAudioTrack',
  MediaLast = 'MediaLast',
  MediaSkipBackward = 'MediaSkipBackward',
  MediaSkipForward = 'MediaSkipForward',
  MediaStepBackward = 'MediaStepBackward',
  MediaStepForward = 'MediaStepForward',
  MediaTopMenu = 'MediaTopMenu',
  NavigateIn = 'NavigateIn',
  NavigateNext = 'NavigateNext',
  NavigateOut = 'NavigateOut',
  NavigatePrevious = 'NavigatePrevious',
  NextFavoriteChannel = 'NextFavoriteChannel',
  NextUserProfile = 'NextUserProfile',
  OnDemand = 'OnDemand',
  Pairing = 'Pairing',
  PinPDown = 'PinPDown',
  PinPMove = 'PinPMove',
  PinPToggle = 'PinPToggle',
  PinPUp = 'PinPUp',
  PlaySpeedDown = 'PlaySpeedDown',
  PlaySpeedReset = 'PlaySpeedReset',
  PlaySpeedUp = 'PlaySpeedUp',
  RandomToggle = 'RandomToggle',
  RcLowBattery = 'RcLowBattery',
  RecordSpeedNext = 'RecordSpeedNext',
  RfBypass = 'RfBypass',
  ScanChannelsToggle = 'ScanChannelsToggle',
  ScreenModeNext = 'ScreenModeNext',
  Settings = 'Settings',
  SplitScreenToggle = 'SplitScreenToggle',
  STBInput = 'STBInput',
  STBPower = 'STBPower',
  Subtitle = 'Subtitle',
  Teletext = 'Teletext',
  VideoModeNext = 'VideoModeNext',
  Wink = 'Wink',
  ZoomToggle = 'ZoomToggle',
  F1 = 'F1',
  F2 = 'F2',
  F3 = 'F3',
  F4 = 'F4',
  F5 = 'F5',
  F6 = 'F6',
  F7 = 'F7',
  F8 = 'F8',
  F9 = 'F9',
  F10 = 'F10',
  F11 = 'F11',
  F12 = 'F12',
  F13 = 'F13',
  F14 = 'F14',
  F15 = 'F15',
  F16 = 'F16',
  F17 = 'F17',
  F18 = 'F18',
  F19 = 'F19',
  F20 = 'F20',
  F21 = 'F21',
  F22 = 'F22',
  F23 = 'F23',
  F24 = 'F24',
  F25 = 'F25',
  F26 = 'F26',
  F27 = 'F27',
  F28 = 'F28',
  F29 = 'F29',
  F30 = 'F30',
  F31 = 'F31',
  F32 = 'F32',
  F33 = 'F33',
  F34 = 'F34',
  F35 = 'F35'
}
export const enum KeyLocation {
  Standard = 'Standard',
  Left = 'Left',
  Right = 'Right',
  Numpad = 'Numpad'
}
export const enum ModifiersKeyState {
  Pressed = 'Pressed',
  Unknown = 'Unknown'
}
export declare class TimeDuration {
  t_secs: number
  t_nanos: number
}
export declare class EventLoop {
  constructor()
}
export declare class ActiveEventLoop { }
export declare class DeviceEvents {
  get type(): DeviceEventsEnum
  get typeName(): string
}
export declare class ControlFlowWaitUntilSpec {
  get time(): TimeDuration
}
export declare class ControlFlow {
  get type(): ControlFlowEnum
  get typeName(): string
  get WaitUntil(): ControlFlowWaitUntilSpec
}
export declare class OwnedDisplayHandle { }
export declare class AsyncRequestSerial { }
export declare class UserPayload { }
export declare class EventNewEventsSpec {
  get elem0(): StartCause
}
export declare class EventWindowEventSpec {
  get window_id(): WindowId
  get event(): WindowEvent
}
export declare class EventDeviceEventSpec {
  get device_id(): DeviceId
  get event(): DeviceEvent
}
export declare class EventUserEventSpec {
  get elem0(): UserPayload
}
export declare class Event {
  get type(): EventEnum
  get typeName(): string
  get NewEvents(): EventNewEventsSpec
  get WindowEvent(): EventWindowEventSpec
  get DeviceEvent(): EventDeviceEventSpec
  get UserEvent(): EventUserEventSpec
}
export declare class StartCauseResumeTimeReachedSpec {
  get start(): TimeDuration
  get requested_resume(): TimeDuration
}
export declare class StartCauseWaitCancelledSpec {
  get start(): TimeDuration
  get requested_resume(): TimeDuration | null
}
export declare class StartCause {
  get type(): StartCauseEnum
  get typeName(): string
  get ResumeTimeReached(): StartCauseResumeTimeReachedSpec
  get WaitCancelled(): StartCauseWaitCancelledSpec
}
export declare class WindowEventActivationTokenDoneSpec {
  get serial(): AsyncRequestSerial
  get token(): ActivationToken
}
export declare class WindowEventResizedSpec {
  get elem0(): Size
}
export declare class WindowEventMovedSpec {
  get elem0(): Position
}
export declare class WindowEventDroppedFileSpec {
  get elem0(): string
}
export declare class WindowEventHoveredFileSpec {
  get elem0(): string
}
export declare class WindowEventFocusedSpec {
  get elem0(): boolean
}
export declare class WindowEventKeyboardInputSpec {
  get device_id(): DeviceId
  get event(): KeyEvent
  get is_synthetic(): boolean
}
export declare class WindowEventModifiersChangedSpec {
  get elem0(): Modifiers
}
export declare class WindowEventImeSpec {
  get elem0(): Ime
}
export declare class WindowEventCursorMovedSpec {
  get device_id(): DeviceId
  get position(): Position
}
export declare class WindowEventCursorEnteredSpec {
  get device_id(): DeviceId
}
export declare class WindowEventCursorLeftSpec {
  get device_id(): DeviceId
}
export declare class WindowEventMouseWheelSpec {
  get device_id(): DeviceId
  get delta(): MouseScrollDelta
  get phase(): TouchPhase
}
export declare class WindowEventMouseInputSpec {
  get device_id(): DeviceId
  get state(): ElementState
  get button(): MouseButton
}
export declare class WindowEventPinchGestureSpec {
  get device_id(): DeviceId
  get delta(): number
  get phase(): TouchPhase
}
export declare class WindowEventPanGestureSpec {
  get device_id(): DeviceId
  get delta(): Position
  get phase(): TouchPhase
}
export declare class WindowEventDoubleTapGestureSpec {
  get device_id(): DeviceId
}
export declare class WindowEventRotationGestureSpec {
  get device_id(): DeviceId
  get delta(): number
  get phase(): TouchPhase
}
export declare class WindowEventTouchpadPressureSpec {
  get device_id(): DeviceId
  get pressure(): number
  get stage(): number
}
export declare class WindowEventAxisMotionSpec {
  get device_id(): DeviceId
  get axis(): number
  get value(): number
}
export declare class WindowEventTouchSpec {
  get elem0(): Touch
}
export declare class WindowEventScaleFactorChangedSpec {
  get scale_factor(): number
  get inner_size_writer(): InnerSizeWriter
}
export declare class WindowEventThemeChangedSpec {
  get elem0(): Theme
}
export declare class WindowEventOccludedSpec {
  get elem0(): boolean
}
export declare class WindowEvent {
  get type(): WindowEventEnum
  get typeName(): string
  get ActivationTokenDone(): WindowEventActivationTokenDoneSpec
  get Resized(): WindowEventResizedSpec
  get Moved(): WindowEventMovedSpec
  get DroppedFile(): WindowEventDroppedFileSpec
  get HoveredFile(): WindowEventHoveredFileSpec
  get Focused(): WindowEventFocusedSpec
  get KeyboardInput(): WindowEventKeyboardInputSpec
  get ModifiersChanged(): WindowEventModifiersChangedSpec
  get Ime(): WindowEventImeSpec
  get CursorMoved(): WindowEventCursorMovedSpec
  get CursorEntered(): WindowEventCursorEnteredSpec
  get CursorLeft(): WindowEventCursorLeftSpec
  get MouseWheel(): WindowEventMouseWheelSpec
  get MouseInput(): WindowEventMouseInputSpec
  get PinchGesture(): WindowEventPinchGestureSpec
  get PanGesture(): WindowEventPanGestureSpec
  get DoubleTapGesture(): WindowEventDoubleTapGestureSpec
  get RotationGesture(): WindowEventRotationGestureSpec
  get TouchpadPressure(): WindowEventTouchpadPressureSpec
  get AxisMotion(): WindowEventAxisMotionSpec
  get Touch(): WindowEventTouchSpec
  get ScaleFactorChanged(): WindowEventScaleFactorChangedSpec
  get ThemeChanged(): WindowEventThemeChangedSpec
  get Occluded(): WindowEventOccludedSpec
}
export declare class DeviceId { }
export declare class RawKeyEvent { }
export declare class KeyEvent { }
export declare class Modifiers { }
export declare class ImePreeditSpec {
  get elem0(): string
  get elem1(): Position | null
}
export declare class ImeCommitSpec {
  get elem0(): string
}
export declare class Ime {
  get type(): ImeEnum
  get typeName(): string
  get Preedit(): ImePreeditSpec
  get Commit(): ImeCommitSpec
}
export declare class MouseButtonOtherSpec {
  get elem0(): number
}
export declare class MouseButton {
  get type(): MouseButtonEnum
  get typeName(): string
  get Other(): MouseButtonOtherSpec
}
export declare class MouseScrollDelta { }
export declare class InnerSizeWriter { }
export declare class TouchPhase {
  get type(): TouchPhaseEnum
  get typeName(): string
}
export declare class Touch { }
export declare class DeviceEventMouseMotionSpec {
  get delta(): Position
}
export declare class DeviceEventMouseWheelSpec {
  get delta(): MouseScrollDelta
}
export declare class DeviceEventMotionSpec {
  get axis(): number
  get value(): number
}
export declare class DeviceEventButtonSpec {
  get button(): number
  get state(): ElementState
}
export declare class DeviceEventKeySpec {
  get elem0(): RawKeyEvent
}
export declare class DeviceEvent {
  get type(): DeviceEventEnum
  get typeName(): string
  get MouseMotion(): DeviceEventMouseMotionSpec
  get MouseWheel(): DeviceEventMouseWheelSpec
  get Motion(): DeviceEventMotionSpec
  get Button(): DeviceEventButtonSpec
  get Key(): DeviceEventKeySpec
}
export declare class WindowAttributes {
  constructor()
  withInnerSize(this: object, size: Size): this
  withMinInnerSize(this: object, minSize: Size): this
  withMaxInnerSize(this: object, maxSize: Size): this
  withPosition(this: object, position: Position): this
  withResizable(this: object, resizable: boolean): this
  withEnabledButtons(this: object, buttons: WindowButtons): this
  withTitle(this: object, title: string): this
  withFullscreen(this: object, fullscreen?: Fullscreen | undefined | null): this
  withMaximized(this: object, maximized: boolean): this
  withVisible(this: object, visible: boolean): this
  withTransparent(this: object, transparent: boolean): this
  get transparent(): boolean
  withBlur(this: object, blur: boolean): this
  withDecorations(this: object, decorations: boolean): this
  withWindowLevel(this: object, level: WindowLevel): this
  withTheme(this: object, theme?: Theme | undefined | null): this
  withResizeIncrements(this: object, resizeIncrements: Size): this
  withContentProtected(this: object, protected: boolean): this
  withActive(this: object, active: boolean): this
}
export declare class WindowButtons {
  static all(): WindowButtons
  static empty(): WindowButtons
  isAll(): boolean
  isEmpty(): boolean
  hasClose(): boolean
  hasMinimize(): boolean
  hasMaximize(): boolean
  toggleClose(this: object): this
  toggleMinimize(this: object): this
  toggleMaximize(this: object): this
  insertClose(this: object): this
  insertMinimize(this: object): this
  insertMaximize(this: object): this
  removeClose(this: object): this
  removeMinimize(this: object): this
  removeMaximize(this: object): this
}
export declare class Icon {
  static fromRgba(rgba: Uint8Array, width: number, height: number): Icon
}
export declare class WindowId { }
export declare class ActivationToken { }
export declare class Window {
  static defaultAttributes(): WindowAttributes
  id(): WindowId
  scaleFactor(): number
  requestRedraw(): void
  prePresentNotify(): void
  resetDeadKeys(): void
  innerPosition(): Position
  outerPosition(): Position
  setOuterPosition(position: Position): void
  requestInnerSize(size: Size): Size | null
  outerSize(): Size
  setMinInnerSize(minSize?: Size | undefined | null): void
  setMaxInnerSize(minSize?: Size | undefined | null): void
  resizeIncrements(): Size | null
  setResizeIncrements(increments?: Size | undefined | null): void
  setTitle(title: string): void
  setTransparent(transparent: boolean): void
  setBlur(blur: boolean): void
  setVisible(visible: boolean): void
  isVisible(): boolean | null
  setResizable(resizable: boolean): void
  isResizable(): boolean
  setEnabledButtons(buttons: WindowButtons): void
  enabledButtons(): WindowButtons
  setMinimized(minimized: boolean): void
  isMinimized(): boolean | null
  setMaximized(maximized: boolean): void
  isMaximized(): boolean
  setFullscreen(fullscreen?: Fullscreen | undefined | null): void
  fullscreen(): Fullscreen | null
  setDecorations(decorations: boolean): void
  isDecorated(): boolean
  setWindowLevel(level: WindowLevel): void
  setWindowIcon(windowIcon?: Icon | undefined | null): void
  setImeCursorArea(position: Position, size: Size): void
  setImeAllowed(allowed: boolean): void
  setImePurpose(purpose: ImePurpose): void
  focusWindow(): void
  hasFocus(): boolean
  requestUserAttention(requestType?: UserAttentionType | undefined | null): void
  setTheme(theme?: Theme | undefined | null): void
  theme(): Theme | null
  setContentProtected(protected: boolean): void
  title(): string
}
export declare class Cursor {
  static fromIcon(icon: CursorIcon): Cursor
  static fromCustom(custom: CustomCursor): Cursor
}
export declare class CustomCursor {
  static fromRgba(rgba: Uint8Array, width: number, height: number, hotspotX: number, hotspotY: number): CustomCursorSource
}
export declare class CustomCursorSource { }
export declare class NativeKeyCodeAndroidSpec {
  get elem0(): number
}
export type NativeKeyCodeMacOSSpec = NativeKeyCodeMacOsSpec
export declare class NativeKeyCodeMacOsSpec {
  get elem0(): number
}
export declare class NativeKeyCodeWindowsSpec {
  get elem0(): number
}
export declare class NativeKeyCodeXkbSpec {
  get elem0(): number
}
export declare class NativeKeyCode {
  get type(): NativeKeyCodeEnum
  get typeName(): string
  get Android(): NativeKeyCodeAndroidSpec
  get MacOS(): NativeKeyCodeMacOsSpec
  get Windows(): NativeKeyCodeWindowsSpec
  get Xkb(): NativeKeyCodeXkbSpec
}
export declare class NativeKeyAndroidSpec {
  get elem0(): number
}
export type NativeKeyMacOSSpec = NativeKeyMacOsSpec
export declare class NativeKeyMacOsSpec {
  get elem0(): number
}
export declare class NativeKeyWindowsSpec {
  get elem0(): number
}
export declare class NativeKeyXkbSpec {
  get elem0(): number
}
export declare class NativeKeyWebSpec {
  get elem0(): string
}
export declare class NativeKey {
  get type(): NativeKeyEnum
  get typeName(): string
  get Android(): NativeKeyAndroidSpec
  get MacOS(): NativeKeyMacOsSpec
  get Windows(): NativeKeyWindowsSpec
  get Xkb(): NativeKeyXkbSpec
  get Web(): NativeKeyWebSpec
}
export declare class KeyNamedSpec {
  get elem0(): NamedKey
}
export declare class KeyCharacterSpec {
  get elem0(): string
}
export declare class KeyUnidentifiedSpec {
  get elem0(): NativeKey
}
export declare class KeyDeadSpec {
  get elem0(): string | null
}
export declare class Key {
  get type(): KeyEnum
  get typeName(): string
  get Named(): KeyNamedSpec
  get Character(): KeyCharacterSpec
  get Unidentified(): KeyUnidentifiedSpec
  get Dead(): KeyDeadSpec
}
export declare class ModifiersState {
  static all(): ModifiersState
  static empty(): ModifiersState
  isAll(): boolean
  isEmpty(): boolean
  hasShift(): boolean
  hasControl(): boolean
  hasAlt(): boolean
  hasSuper(): boolean
  toggleShift(this: object): this
  toggleControl(this: object): this
  toggleAlt(this: object): this
  toggleSuper(this: object): this
  insertShift(this: object): this
  insertControl(this: object): this
  insertAlt(this: object): this
  insertSuper(this: object): this
  removeShift(this: object): this
  removeControl(this: object): this
  removeAlt(this: object): this
  removeSuper(this: object): this
}
export declare class VideoModeHandle { }
export declare class MonitorHandle { }
