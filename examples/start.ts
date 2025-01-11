import {
    EventLoop,
    EventType,
    Fullscreen,
    StartCause,
    sum,
    UnitType,
    WindowAttributes
} from "../index";

const event_loop = new EventLoop();

console.log({UnitType, EventLoop, EventType, StartCause, WindowAttributes, sum});

const attrs = new WindowAttributes()
    .withActive(true)
    .withFullscreen(Fullscreen.Borderless)
    .withTitle("ha la shao")
    .withInnerSize({type: UnitType.Logical, width: 100, height: 100})
    .withPosition({type: UnitType.Logical, x: 500, y: 500})
    .withTransparent(true);


console.log(attrs, { transparent: attrs.transparent });