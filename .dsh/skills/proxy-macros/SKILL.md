---
name: proxy-macros
description: napi-winit 自研 proc 宏（crates/proc）的使用指南——proxy_enum / proxy_struct / proxy_wrap / proxy_impl / proxy_flags 的全部参数、生成物与陷阱。为 winit 类型编写 napi 代理、或排查宏展开 / 类型转换编译错误时使用。
---

# proxy 宏族（crates/proc）

五个属性宏把 winit（origin）类型包装为 napi（JS）类型，并自动生成双向转换：

| 宏 | 作用于 | 生成物 |
|---|---|---|
| `proxy_enum` | enum | `#[napi]` enum + `From<origin>` + `Into<origin>` |
| `proxy_struct` | struct | `#[napi]` struct（class 或 object）+ `From` + `Into` |
| `proxy_wrap` | struct（newtype） | `#[napi]` 包装 struct + 字段视图 getter/setter + `From` + `Into` |
| `proxy_impl` | impl 块 | `#[napi]` impl，方法体自动转发到 origin |
| `proxy_flags` | struct（bitflags） | bool 字段 struct + contains/insert 转换 + 全套旗标方法 |

通用心智模型：**proxy 是 origin 的镜像，`From<origin>`（from_origin）是 origin→proxy 的组合（compose），`Into<origin>`（into_origin）是 proxy→origin 的拆解（dispose）**。

## 通用规则

### origin 默认类型

所有宏的 `origin_type`/`origin` 缺省时取 `Origin<Ident>`（如 `enum Modifiers` → `OriginModifiers`）。本仓库总是显式写 `origin_type = winit::xxx::Yyy`。

**陷阱：meta 值必须是简单 Path**。`origin_type = Box<dyn winit::window::Window>` 这类含 `dyn` 的类型字面量会让 meta 解析中断、整组 meta 丢失（回退到默认 `Origin<Ident>`）。复杂类型先建 type alias 再引用：

```rust
pub(crate) type DynWindow = Box<dyn winit::window::Window>;
#[proxy_wrap(origin_type = DynWindow, field_name = inner)]
pub struct Window;
```

### 转换器（ConfUsage）

`from_origin` / `into_origin` / `conv_arg` / `conv_return` / `conv_get` / `conv_set` 的值都是"转换器表达式"，作用于被转换的值（记作输入）。生成代码按表达式形态区分：

| 形态 | 生成代码 | 例子 |
|---|---|---|
| Path | `path(输入)` | `option_into` → `option_into(input)`；`Some` → `Some(input)` |
| 闭包 | `(闭包)(输入)` | `\|v: f32\| v as f64` |
| 函数调用 | `call(输入)` | `Duration::from_millis` 形式：`call(input)` |
| 方法调用 | 原样嵌入（不注入输入） | `conv_arg = purpose.into()` 直接生成 `purpose.into()`，在表达式里引用参数名 |
| 块 | 原样嵌入 | `{ ... }` |
| 数组（Pipe） | 从左到右链式：`c(b(a(输入)))` | `[ Clone::clone, Into::into ]` → `Into::into(Clone::clone(input))` |

**缺省转换器是 `input.into()`**（要求 proxy 类型与 origin 类型之间存在 `From`）。`Option<T>` 字段不会自动处理嵌套转换——`Option<proxy>` 与 `Option<origin>` 之间必须显式写 `from_origin = option_into` / `into_origin = option_into`（`option_into`、`option_map`、`to_option_string`、`result_map` 等 helper 见 `src/utils/helpers.rs`）。

### 方向开关（所有宏通用）

```
skip_from_origin   不生成 From<origin>          （origin → proxy 断开）
skip_into_origin   不生成 Into<origin>          （proxy → origin 断开）
skip_to_js         napi object_to_js = false     （Rust → JS 序列化断开）
skip_from_js       napi object_from_js = false   （JS → Rust 反序列化断开）
skip_forward       = skip_from_origin + skip_to_js
skip_backward      = skip_into_origin + skip_from_js
```

事件域类型（WindowEvent、PointerSource 等）只从 Rust 流向 JS，标 `skip_backward` 或 `skip_from_js` 可跳过反序列化生成（napi 不支持的字段类型如 `f32` 只参与 ToNapiValue 时合法）。

### 字段级属性

字段/变体字段上写 `#[proxy_enum(...)]` / `#[proxy_struct(...)]` / `#[proxy_wrap(...)]`：

- `field_name = ident`：组合/拆解时该字段的名字。tuple 字段默认是 `field_0`、`field_1`（按索引），**务必命名**，否则 JS 侧出现 `field0` 这类名字。
- `from_origin = <转换器>`：From 时该字段 origin→proxy 的转换。
- `into_origin = <转换器>`：Into 时该字段 proxy→origin 的转换。
- `Named` / `Unnamed`：强制字段的 kind（默认按语法形态判断）。

## proxy_enum

```rust
/**[winit::event_loop::DndAction]*/
#[proxy_enum(origin_type = winit::event_loop::DndAction, string_enum, non_exhaustive)]
pub enum DndAction { Move, Copy, Link, Ask, Private }
```

宏参数：

- `origin_type = <Type>`：origin 枚举。
- `string_enum`：JS 侧字符串枚举。From/Into 按 unit 变体逐个匹配（origin 变体必须真的是 unit）。
- `non_exhaustive`：追加 `NonExhaustive` 变体；From 加 `_ => Self::NonExhaustive`，Into 加 `Self::NonExhaustive => unreachable!(...)`。**origin 声明了 `#[non_exhaustive]` 时必须加**，否则 match 不完整。
- `code_name = <Ident>`：判别值字段名。另外**任一变体写了显式判别值（`Left = 0`）时自动进入 discriminant 模式**，字段名固定 `discriminant`，类型取 `#[repr]`（缺省 u8）。discriminant 模式下：proxy 变体生成 named 形式 `{ #code_name: #code_type }`，From 用 unsafe cast 读取 origin 变体的判别值，Into 按 origin 变体名直接构造。判别值模式优先于 string_enum（两者同写时，JS 形状是 `{ type: 'Left', discriminant: 0 }` 的 object union，不是纯字符串）。

变体字段写法（tuple 字段转 named，供 JS 使用）：

```rust
#[proxy_enum(origin_type = winit::keyboard::PhysicalKey)]
pub enum PhysicalKey {
    Code(#[proxy_enum(field_name = code)] KeyCode),
    Unidentified(#[proxy_enum(field_name = key)] NativeKeyCode),
}
```

tuple 多字段逐个命名（`src/event.rs`）：

```rust
#[proxy_enum(origin_type = winit::event::MouseScrollDelta, skip_backward, non_exhaustive)]
pub enum MouseScrollDelta {
    LineDelta(#[proxy_enum(field_name = x)] f64, #[proxy_enum(field_name = y)] f64),
    PixelDelta(#[proxy_enum(field_name = delta)] Position),
}
```

## proxy_struct

```rust
/**[winit::event::TabletToolData]*/
#[proxy_struct(origin_type = winit::event::TabletToolData, object, skip_from_js)]
pub struct TabletToolData {
    #[proxy_struct(from_origin = option_into, into_origin = option_into)]
    pub force: Option<Force>,
    pub tangential_force: Option<f32>,   // 缺省转换器 .into()，Option<f32> 恒等
    ...
}
```

- `object`：生成 `#[napi(object)]`（JS plain object，无 class 实例）。省略则生成 napi class。
- From 解构 origin（named 字段带 `..`，对 `#[non_exhaustive]` origin 安全；origin 是 tuple struct 时按位置解构）。
- proxy struct 必须是 named fields。

## proxy_wrap

把 origin 包装进 newtype（移动所有权），是 Window、Icon、EventLoop 这类"持有 origin 实体"的类型用的宏。

```rust
#[proxy_wrap(origin_type = DynWindow, field_name = inner)]
pub struct Window;
```

宏参数：

- `origin_type = <Type>`：被包装类型（可以是 type alias，见上文陷阱）。
- `field_name = <Ident>`：包装字段名；省略则生成 tuple struct `pub struct X(pub(crate) origin)`。
- `use_non_null`：包装字段改为 `NonNull<origin>`（**不持所有权**，访问用 `unsafe { self.x.as_ref()/as_mut() }`）。用于生命周期由外部管理、不便移动所有权的 origin。
- `no_getter` / `no_setter`：struct 级关闭 getter/setter 生成。
- `skip_*` 系列：From/Into 与 napi 方向开关。

**struct 定义里写的"字段"不是存储**——存储只有 `field_name` 那一个包装字段。定义里列的字段是 origin 的视图声明，用于生成 getter/setter（无视图字段时如 `pub struct Icon;` 不生成 impl 块）：

```rust
// src/event.rs 真实用例
#[proxy_wrap(origin_type = winit::event::KeyEvent, skip_into_origin, no_setter)]
pub struct KeyEvent {
    pub physical_key: PhysicalKey,

    #[proxy_wrap(get_ref, conv_get = [Clone::clone, Into::into])]
    pub logical_key: Key,

    #[proxy_wrap(get_ref, conv_get = [Clone::clone, option_into])]
    pub text: Option<String>,
    ...
}
```

字段级参数：

- `get_ref`：getter 里 `ref val`（按借用读，避免要求 origin 字段 Copy/Clone）。
- `conv_get` / `conv_set`：读取/写入时的转换器。
- `no_getter` / `no_setter`：字段级关闭。

getter/setter 转发形态：getter 是 `let #origin_type { <字段>: val, .. } = self.<包装字段>; <conv_get>(val)`；setter 是 `self.<包装字段>.<字段> = <conv_set>(val)`。所以视图字段的类型必须与 origin 字段类型（或转换结果）一致。

From/Into 是整字段移动：`From<origin>` 直接 `X { inner: value }`，`Into<origin>` 直接 `self.inner`——没有字段级转换介入。

## proxy_impl

作用于 `#[napi]` 代理类型的 impl 块，把 trait 风格的**函数签名**批量生成为转发方法。impl 块内只能写签名（分号结尾），带方法体的常规 `fn` 会 `unimplemented!`：

```rust
// src/window.rs 真实用例（access_expr 缺省为 self.inner，此处未写）
#[proxy_impl(conv_return = [ result_map(Into::into), result_err_reason ])]
fn outer_position(&self) -> Result<Position>;
```

- `access_expr = <Expr>`：转发目标（默认 `self.inner`）。impl 级声明一次，函数级可覆盖。
- `conv_arg = <转换器>`：参数进入 origin 调用前的转换。写在参数上：`fn set_ime_purpose(&self, #[proxy_impl(conv_arg = purpose.into())] purpose: ImePurpose);`——方法调用形态在表达式里直接引用参数名。
- `skip_conv_arg`：该参数原样传入（不套 `.into()`）。
- `conv_return = <转换器>`：返回值转换（`result_map(...)`、`option_map(...)`、pipe 数组等）。
- `skip_conv_return`：原样返回。
- 可见性：签名未写 vis 时生成 `pub`；写了（`pub(crate)`）则保留。
- `&self` / `&mut self` 保留；非 self 参数按 `conv_arg`/`skip_conv_arg` 逐个转换后传入 `access_expr.<方法名>(...)`。

## proxy_flags

包装 bitflags 风格的 origin（要求 origin 支持 `contains` / `empty` / `insert`）：

```rust
#[proxy_flags(origin = winit::window::WindowButtons, flags = (CLOSE, MINIMIZE, MAXIMIZE))]
pub struct WindowButtons;
```

- `origin = <Type>`、`flags = (FLAG_A, FLAG_B, ...)`（必须，tuple 字面量）。
- 生成：`#[napi]` struct，每旗标一个 `pub(crate) flag_<lower>: bool`；`From<origin>`（`origin.contains(Origin::FLAG)`）、`Into<origin>`（`empty()` + 逐个 `insert`）。
- 方法：`all()` / `empty()` 工厂、`is_all()` / `is_empty()`、每旗标 `has_<lower>()`、`toggle_<lower>()` / `insert_<lower>()` / `remove_<lower>()`（链式，返回 this）。

## 实战配方

1. **事件域单向类型**（Rust → JS 只读）：`skip_backward`（enum）或 `skip_from_js`（struct/object），省掉反序列化生成；napi 不支持 `f32` 参数但支持 `f32` 输出，单向类型字段可直接用 origin 的 `f32`。
2. **Option<proxy> ↔ Option<origin>**：双向都写 `option_into`；需要映射内部值时用 `option_map(|v| ...)`。
3. **origin 是 `#[non_exhaustive]` 枚举**：proxy_enum 必须加 `non_exhaustive`，否则 From 的 match 不完整。
4. **非穷尽 origin struct 字段**：From 生成时自动带 `..`，无需处理。
5. **返回 `Result<T>` 的 origin 方法**：proxy_impl 用 `conv_return = [ result_map(Into::into), result_err_reason ]` 这类 pipe 先映射成功值再归一化错误。
6. **排查展开错误**：`cargo expand <路径>` 看生成代码；meta 解析失败（尤其复杂类型字面量）表现为"某组 meta 静默丢失 + 回退默认值"，先检查 meta 值是否简单 Path。

