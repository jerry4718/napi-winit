import {
    EventLoop,
    EventType,
    Fullscreen,
    PositionType,
    SizeType,
    StartCause,
    sum,
    UnitType,
    WindowAttributes
} from "../index";

const event_loop = new EventLoop();

console.log({PositionType, SizeType, UnitType, EventLoop, EventType, StartCause, WindowAttributes, sum});

const attrs = new WindowAttributes()
    .withActive(true)
    .withFullscreen(Fullscreen.Borderless)
    .withTitle("ha la shao")
    .withInnerSize({type: SizeType.Logical, data: {width: 100, height: 100}})
    .withPosition({type: PositionType.Logical, data: {x: 500, y: 500}})
    .withTransparent(true);


console.log(attrs, { transparent: attrs.transparent });