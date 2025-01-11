/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum PositionType {
  Physical = 0,
  Logical = 1
}
export interface PositionData {
  x: number
  y: number
}
export interface Position {
  type: PositionType
  data: PositionData
}
export const enum SizeType {
  Physical = 0,
  Logical = 1
}
export interface SizeData {
  width: number
  height: number
}
export interface Size {
  type: SizeType
  data: SizeData
}
export const enum UnitType {
  Physical = 0,
  Logical = 1
}
export interface PixelUnit {
  type: UnitType
  data: number
}
export const enum EventType {
  NewEvents = 0,
  WindowEvent = 1,
  DeviceEvent = 2,
  UserEvent = 3,
  Suspended = 4,
  Resumed = 5,
  AboutToWait = 6,
  LoopExiting = 7,
  MemoryWarning = 8
}
export const enum StartCause {
  ResumeTimeReached = 0,
  WaitCancelled = 1,
  Poll = 2,
  Init = 3
}
export const enum Fullscreen {
  Exclusive = 0,
  Borderless = 1
}
export declare function sum(a: number, b: number): number
export type JsTimeDuration = TimeDuration
export declare class TimeDuration {
  tSecs: number
  tNanos: number
}
export type JsEventLoop = EventLoop
export declare class EventLoop {
  constructor()
}
export type JsWindowAttributes = WindowAttributes
export declare class WindowAttributes {
  constructor()
  withInnerSize(this: object, size: Size): this
  withMinInnerSize(this: object, minSize: Size): this
  withMaxInnerSize(this: object, maxSize: Size): this
  withPosition(this: object, position: Position): this
  withResizable(this: object, resizable: boolean): this
  withEnabledButtons(this: object, buttons: number): this
  withTitle(this: object, title: string): this
  withFullscreen(this: object, fullscreen?: Fullscreen): this
  withMaximized(this: object, maximized: boolean): this
  withVisible(this: object, visible: boolean): this
  withTransparent(this: object, transparent: boolean): this
  get transparent(): boolean
  withBlur(this: object, blur: boolean): this
  withDecorations(this: object, decorations: boolean): this
  withContentProtected(this: object, protected: boolean): this
  withActive(this: object, active: boolean): this
}
export type JsWindowButtons = WindowButtons
export declare class WindowButtons {
  static all(): JsWindowButtons
  static empty(): JsWindowButtons
  removeClose(this: object): this
  removeMinimize(this: object): this
  removeMaximize(this: object): this
  insertClose(this: object): this
  insertMinimize(this: object): this
  insertMaximize(this: object): this
}
