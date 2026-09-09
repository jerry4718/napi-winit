---
name: napi-rs
description: napi-rs v3 参考——类型转换映射、函数/类/枚举导出形态、异步与 ThreadsafeFunction、迭代器、错误处理、引用与生命周期、cargo features，以及 `#[napi]` 全部属性的选项/目标/运行时与 TS 效果速查。编写或审查 napi 导出代码、控制生成的 index.d.ts、排查 napi 宏编译错误时使用。
---

# napi-rs v3 参考（napi / napi-derive）

前半是使用指南（类型转换、导出形态、异步、错误处理、生命周期、features），后半是 `#[napi]` 属性速查表。

## 本项目环境

- napi 3.12.2（features: `napi6` + `async`）/ napi-derive 3.6.3（features: `type-def`）。
- napi-derive 的 default 特性是 `type-def` + `strict`，本项目未关闭：**strict 生效**，选项用在不支持的目标上是编译错误。
- 未开 `napi8`：`type_tag` 的 stamp/check 是 no-op。未开 `tokio_rt`：`async_iterator` 不可用；`async_runtime` 在运行时未启用时是 no-op。
- 生成 .d.ts 需要 `type-def` 特性（本项目已开）。

## 总则

**运行时转换与 TypeScript 生成是两条独立通路。** `ts_*` 系列选项和 `skip_typescript` 只改变 napi-derive（type-def 特性）生成的声明，不添加任何运行时校验或转换。

术语约定：

- **Function**：导出的自由函数。
- **Method**：实例方法、静态方法、factory、constructor、getter、setter 中该选项适用的场合。
- **Class**：以 class 身份导出的 struct；object / array / transparent struct 是**值形状**（value shape），没有 class 身份。
- **Field**：struct 字段，或结构化枚举变体的字段。

## 类型转换映射

参数需 `FromNapiValue`，返回值需 `ToNapiValue`；TS 生成类型不保证双向可转。除标注外均为双向。

| Rust | JS / TS | 方向 | 要点 / feature |
| --- | --- | --- | --- |
| `()` / `Undefined` | `undefined`；返回变 `void` | 双向 | `strict` 下输入须为 `undefined` |
| `Null` | `null` | 双向 | 普通输入接受并丢弃任意值；`strict` 下须为 `null` |
| `bool` | `boolean` | 双向 | 复制 |
| `i8` `u8` `i16` `u16` `i32` `u32` `f64` | `number` | 双向 | JS Number 即 IEEE-754 double |
| `f32` | `number` | **仅 Rust→JS** | 无 `FromNapiValue`；输入用 `f64` |
| `i64` | `number` | 双向 | 超 JS 安全整数范围丢精度 |
| `BigInt` | `bigint` | 双向 | `napi6`；getter 返回 `(signed, value, lossless)` 报告收窄是否无损 |
| `u64` `u128` `i128` `usize` `isize` `i64n` | `bigint` | **仅 Rust→JS** | `napi6`；避免静默收窄任意 JS BigInt |
| `String` `Utf16String` `Latin1String` `OsString` `PathBuf` | `string` | 双向 | `Latin1String` 解码需 `latin1`；Windows 用 UTF-16 保留未配对代理 |
| `&str` `&OsStr` `&Path` | `string` | **仅 Rust→JS** | 输入用 `String` / `PathBuf` |
| `Symbol` | `symbol` | 双向 | 普通输入丢弃值不保留身份；保留已有值用 scoped `JsSymbol`；`Symbol::for_desc` 需 `napi9` |

集合：`Vec<T>` ↔ `Array<T>`（O(n) 逐元素）；`[T; N]` 仅 →JS；元组 ↔ TS tuple；`Array<'env>` ↔ `unknown[]`（scoped）；`HashMap` / `BTreeMap` ↔ `Record`（**非** JS `Map`，键须可转字符串）；`IndexMap` ↔ `Record`、`IndexSet` ↔ `Set`（`object_indexmap`，保插入序）；`HashSet` / `BTreeSet` ↔ `Set<T>`。

对象 / 类：`Object<'env>`（匿名对象，TS `object`，不能赋 method；每次属性访问跨 Node-API 边界，比原始类型慢）；`Unknown<'env>`（unchecked，须显式检查/强转）；`#[napi(object)] struct`（全字段必须 `pub`，生成 interface，**克隆语义**——两侧改动互不反映）；`#[napi] struct`（JS class）；`ClassInstance<'env, T>`（收类实例本体；`Vec<T>` 需 owned `FromNapiValue` 故不适用）；结构化 `#[napi] enum`（判别对象联合）。

Date / serde：`chrono::DateTime<Tz>` / `NaiveDateTime` ↔ `Date`（`chrono_date` = 启用 chrono + napi5，毫秒精度）；`serde_json::Value` ↔ JSON 兼容值（`serde-json`；拒函数 / undefined / symbol / external；要 BigInt 语义用 `BigInt` 而非 JSON）；`serde_json::Map<String, Value>` ↔ plain object；`serde-json-ordered` 启用 `preserve_order`。

**Option 不对称映射**：参数收 `T | null | undefined`（两个 nullish 均 → `None`）；返回 `None` → `null`（TS `T | null`）。`#[napi(object)]` 字段默认 `field?: T`（缺失 / undefined → `None`；显式 `null` 传给内部 T 转换通常报错；`None` 输出省略），`use_nullable` 时 `field: T | null`（`null` → `None`；缺失 / undefined 报错）。类字段访问器总存在，getter 对 `None` 发 `null`。需要区分 nullish 用 `Null` / `Undefined`；只收单个 nullish 用 `Either<T, Null>` / `Either<T, Undefined>`。

**Either**：`Either<A, B>` 至 `Either26` → TS union（变体名 `A` / `B` / `C`…）；输入按 `ValidateNapiValue` 从左到右测试、取首个匹配；重叠项从最具体到最不具体排列。

**BigInt 补充**：传**入**只能用 `BigInt` 类型（不能收 `i128` / `u128` / `u64` / `i64n`，可能丢精度）；返回可用 `BigInt` / `i64n` / `u64` / `i128` / `u128`；返回 `i64` 是 `number` 不是 `BigInt`。取值用 `get_u64` / `get_u128` / `get_i128` 等，返回 `(signed, value, lossless)`。

**函数 / 异步类型**：`Function<'env, Args, Return>`（JS→Rust in scope；仅 owning thread 调用；多参用 `FnArgs<(...)>`）；`FunctionRef`（无 `ToNapiValue`，传回用 `borrow_back(env)` 取 scoped `Function`）；`ThreadsafeFunction`（`napi4`，从其他线程回调 JS）；`Promise<T>`（仅 JS→Rust，实现 `Future` 可 `await`）；`PromiseRaw<'env, T>`（scoped 句柄，可直接调 `then` / `catch` / `finally`）；`async fn` 返回 → `Promise`；`AsyncTask<T>` → `Promise<T::JsValue>`。

**External**：`External<T>` 创建空白 JS Object、底层持有原生值，**只能传回 Rust 使用**；生成 TS 类型 `ExternalObject<T>`。返回 owned 转移进 JS external；`&External<T>` / `&mut External<T>` 借用并类型检查；`ExternalRef<T>` 供 Rust 持 JS 引用。典型模式：返回 `External<T>` 配 `ts_return_type` 自定义声明，JS 闭包持有后传回给后续方法调用。

**Buffer 与 TypedArray 生命周期**：`Buffer` 是 `Uint8Array` 子类；`vec.into()` 建 Buffer 在支持 external buffer 的 runtime 零拷贝（Electron 的 V8 Memory Cage 例外，会拷贝）。

- Owned（`Buffer`、`Uint8Array` / `Int32Array` / `Float64Array` 等）：可活过调用、跨 async / 线程；来自 JS 时建 `napi_ref` 保活到 wrapper drop。**`Send` / `Sync` 不同步共享字节**——JS 可同时改底层存储，跨线程前先 `to_vec()`，否则 data race / UB。
- Borrowed（`BufferSlice<'env>`、`Uint8ArraySlice<'env>`、`&[u8]` / `&[f32]`…）：零拷贝，生命周期绑函数 scope，不能跨 `await`。`&[u32]` 参数的 TS 类型是 `Uint32Array`。
- 选型：仅同步零拷贝 → `&[...]`；需转 owned 或存 `Object` / `Unknown` → `*Slice`；要存储超过调用 / async → `Buffer`。API：`slice.into_buffer(env)`、`BufferSlice::copy_from(env, data)`、`unsafe BufferSlice::from_external(env, ptr, len, hint, finalizer)`、`unsafe as_mut`（原地改传入的 TypedArray）。

**所有权优先序**：owned 值（需跨线程 / await）→ scoped 句柄 / 切片（同步低拷贝）→ 引用保持型（`Buffer` / `ObjectRef` / `FunctionRef` / `Reference<T>`）→ `ThreadsafeFunction`（勿把 scoped 句柄带去其他线程）→ stream（增量生产）。

**TypedArray 引用语义**：传入 Rust 的是**引用**，无 Copy/Clone，Rust 改动反映到原 JS TypedArray；`to_vec()` 取副本。

## 导出形态：函数

- 普通 `fn` 加 `#[napi]` 即导出为 JS function；每个参数类型须实现 `FromNapiValue`，每个返回类型须实现 `ToNapiValue`。
- 转换速览：数字 / `bool` / `String` 双向直映；`Option<T>` 参数收 `T | null | undefined`、返回 `None` → `null`；`Vec` / tuple / `HashMap` / `#[napi(object)]` → 数组 / 对象；`Buffer` / typed-array 包装 → `Buffer` / `TypedArray`；`async fn` 或 `AsyncTask` 返回 → `Promise<T>`。完整矩阵见上文"类型转换映射"。
- `Result<T>` 返回在 d.ts 中表现为 `T`，`Err` 时抛 JS Error（方法同理）。

JS 回调作参数用 `Function<Args, Return>`；多参数回调用 `FnArgs<(A, B)>` 并以 `.into()` 传 tuple；`ThreadsafeFunction` / `Arc<ThreadsafeFunction>` 可直接作参数，仅在需动态创建时才用 `Function::build_threadsafe_function()`。

```rust
use napi::bindgen_prelude::*;

#[napi] // d.ts: (callback: (arg: number) => number) => void
pub fn call_function(callback: Function<u32, u32>) -> Result<u32> {
  callback.call(1)
}

#[napi] // params > 1: use FnArgs; tuple converts via .into()
pub fn call_function_with_args(callback: Function<FnArgs<(u32, u32)>, u32>) -> Result<u32> {
  callback.call((1, 2).into())
}

#[napi] // apply with this: use ClassInstance<T> as the this argument
pub fn call_function_with_apply(
  this: ClassInstance<RustClass>,
  callback: Function<(), ()>,
) -> Result<()> {
  callback.apply(this, ())
}
```

## 导出形态：类

JS 形态选择：`#[napi] struct` → 有原生身份 / 方法 / 引用的 class；`#[napi(object)]` → 拷贝式 plain object（记录 / 配置）；`#[napi(transparent)] struct W(T)` → 就是内层 `T`；`#[napi(array)]` tuple struct → Array / tuple。

```rust
#[napi(constructor)] // all fields pub: default constructor(name: string, kind: number)
pub struct Animal { pub name: String, pub kind: u32 }

#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine { count: u32 } // private fields stay native; pub fields get getter+setter

#[napi]
impl JsQueryEngine {
  #[napi(constructor)] // custom constructor; must be pub
  pub fn new() -> Self { JsQueryEngine { count: 0 } }

  #[napi(factory)] // static withInitialCount(count: number): QueryEngine
  pub fn with_initial_count(count: u32) -> Self { JsQueryEngine { count } }

  #[napi] // async method needs napi4 + tokio_rt features
  pub async fn query(&self, query: String) -> napi::Result<String> { Ok(query) }

  #[napi(getter)] // get status(): number; must be a struct method (not an associated fn)
  pub fn status(&self) -> napi::Result<u32> { Ok(self.count) }

  #[napi(setter)] // set count(count: number); must be a struct method
  pub fn count(&mut self, count: u32) { self.count = count; }
}
```

- struct 未定义任何 `constructor` 时，JS 侧 `new` 抛错：`Class contains no constructor, cannot create it!`。
- 字段默认可写（生成 getter + setter，需 `ToNapiValue` + `FromNapiValue`）；`#[napi(readonly)]` 去掉 setter（仅需 `ToNapiValue`），`#[napi(skip)]` 两者都去。
- Class 作参数：`engine: &QueryEngine` / `&mut QueryEngine`（d.ts 参数即该 class 类型）；值是 GC 管理的原生实例，不从 plain object 克隆。
- 自定义 GC 释放：struct 上标 `#[napi(custom_finalize)]`，自行实现 `ObjectFinalize`（`fn finalize(self, mut env: Env) -> Result<()>`）。
- 所有 napi class 提供 `NativeClass::instance_of(env, &value) -> Result<bool>`。

## 导出形态：枚举

- 无字段枚举默认数字枚举：`export const enum Kind { Duck, Dog, Cat }`，变体值从 0 开始连续 `i32`；显式 Rust 整数 discriminant 保留，后续隐式变体从前值继续。TS 数字 enum 的反向映射（值 → 名）在 Rust 侧不存在。
- 字符串枚举与结构化枚举的选项见下方 `#[napi]` 属性速查的"枚举"表。

```rust
#[napi] // data-carrying variants -> discriminated object union (not an enum object)
pub enum Event {
  Ready,
  FileChanged { path: String }, // named fields keep their names
  Progress(u32, u32),           // tuple fields become field0, field1, ...
}
// d.ts:
// export type Event = { type: 'Ready' }
//   | { type: 'FileChanged'; path: string }
//   | { type: 'Progress'; field0: number; field1: number }
```

- 判别属性默认 `type`；字段 JS 名不得与判别名相同。结构化 enum 转换是 owned（收参读字段、返回新建 JS 对象）。
- 不支持把 Rust enum 的 `impl` 导出为 JS。

## 命名与导出组织

- 默认自动 `snake_case` → `camelCase`（函数、参数、字段同一规则）；`js_name` 覆盖。
- `#[napi] pub const DEFAULT_COST: u32 = 12;` → `export const DEFAULT_COST: number`。
- namespace 两种方式：inline mod（每个 `#[napi]` 子项导出在内部，不支持嵌套 napi mod），或对单个函数 / 类 / impl / enum / const / 类型别名加 `namespace = "..."`。class 与其 impl 块须用同一 namespace。

```rust
#[napi] // add #[napi(js_name = "...")] on the mod to rename the namespace object
mod xxh3 {
  #[napi]
  pub const ALIGNMENT: u32 = 16;
  #[napi(js_name = "xxh3_64")]
  pub fn xxh64(input: Buffer) -> u64 { todo!() }
}
// JS: xxh3.ALIGNMENT; xxh3.xxh3_64(...)
```

## 模块初始化

执行时序：Node 加载 `.node` → `#[napi_derive::module_init]`（ctor 机制，动态库加载时执行）→ `napi_register_module_v1` 注册所有 `#[napi]` 导出 → `#[napi(module_exports)]`（注册期间）→ 模块可用。

| | `#[napi_derive::module_init]` | `#[napi(module_exports)]` |
|---|---|---|
| 执行次数 | 每次动态库加载一次（同进程 worker 共享） | 每个 Node.js context 一次（main + 每个 worker） |
| 参数 | 无（无 Env、无 exports） | `Env` / `Object` 或其引用，均可选 |
| 返回 | `()` | `()` 或 `Result<()>` |
| 用途 | 全局一次性资源（如自定义 tokio runtime） | exports 定制 / per-context 初始化 |

```rust
// option 1: runs once per native library load; no Env, no exports
#[napi_derive::module_init]
fn init() { /* one-time setup */ }

// option 2: runs during napi_register_module_v1
#[napi(module_exports)]
pub fn customize_exports(mut exports: Object) -> Result<()> {
  exports.set_named_property("THREAD_SAFE_SYMBOL", Symbol::new("THREAD_SAFE"))?;
  Ok(())
}
```

## 参数注入：Env 与 This

- 需要底层 Node-API 时给 `#[napi]` fn 加一个 `Env` 参数，自动注入；`Env` 不出现在 JS 签名里。impl 块中同样可用（`env` 紧跟 `&self`）。
- 需要 JS 侧 `this` 时加 `this: This` 参数；JS 侧调用不传它。`This<'_>` 直接在 this 上取属性（`this.get::<T>(...)`）；`This<&Width>` 经 `this.object` 取绑定的原生对象。

```rust
#[napi] // d.ts: callEnv(length: number) — env does not appear in the JS signature
pub fn call_env(env: Env, length: u32) -> Result<External<Vec<u32>>> {
  env.adjust_external_memory(length as i64)?;
  Ok(External::new(vec![0; length as usize]))
}

#[napi]
pub fn plus_one(this: This<&Width>) -> i32 {
  this.object.value + 1 // plusOne.call(width) -> 2
}
```

## 异步导出

- 门槛：napi 的 `async`（`tokio_rt` 别名）feature；Tokio 子 feature（`tokio_fs` / `tokio_time` 等）按需另开。启用后 napi-rs 提供托管 Tokio runtime，`async fn` 在其上执行并转 JS `Promise`。
- async 方法用 `&mut self` 必须标 `unsafe`（self 同时被 Node.js 持有）：`#[napi] pub async unsafe fn run(&mut self) {}`，否则编译错 `&mut self in async napi methods should be marked as unsafe`。
- `&self` / `&mut self` / `This<T>` 参数自动转 `Reference`：调用前隐式 `napi_create_reference`、调用后隐式删除，保证 self 存活到 await 结束。
- `AsyncBlock<T>`：从同步 `#[napi]` 函数返回，future 立即在 napi-rs runtime 启动并转 Promise；用于只能在 JS 线程创建的返回值（`BufferSlice<'static>` 等）。
- CPU 密集活用 AsyncTask（libuv 线程池），别阻塞 Tokio runtime。

```rust
#[napi]
pub async fn read_file_async(path: String) -> Result<Buffer> {
  let content = napi::tokio::fs::read(path).await?;
  Ok(content.into())
}
// export function readFileAsync(path: string): Promise<Buffer>
```

## 自定义异步运行时（async-runtime feature）

- `async-runtime` feature 只 implies `napi4`，刻意不拉 Tokio。未注册后端时 runtime 操作以 missing-backend 错误 reject。
- 后端 = `unsafe impl AsyncRuntime`（`spawn` / `block_on` / `enter` / `start` / `shutdown` / `spawn_blocking`，后三项有默认实现），每镜像注册一次、first-writer-wins，须在注册窗口关闭前完成——自然位置是 `#[module_init]`。
- `register_async_runtime(rt)`（infallible）或 `try_register_async_runtime(rt)`；`shutdown_async_runtime()` 显式触发后端 `shutdown`（Drop 不保证运行，一切资源在 shutdown 释放；WASM 目标不自动 shutdown）。
- shutdown 返回后 Node 可能立即卸载 native image：后端线程 / 任务 / 保留的 Waker 不得再执行其代码。
- panic 隔离要求 `panic = "unwind"` 构建；`panic = "abort"`（含官方 `wasm32-wasip1`）下 async fn panic 在 Promise settle 前 trap/abort。
- 现成实现：`napi-async-runtime`（`napi_async_runtime::install(RuntimeOptions::default())` 放在自己的 `#[module_init]` 里；`MultiThread` 走 Rayon 池仅 native，`CurrentThread` 用于 WASM）。

## AsyncTask（libuv 线程池）

- `Task` trait：`type Output`（compute 返回）/ `type JsValue`（resolve 返回）；`compute` 在 libuv 线程跑重计算，`resolve` 在主线程造 JS 值。impl 上的 `#[napi]` 只为生成 .d.ts（缺它则返回类型是 `Promise<unknown>`）。
- 可选方法：`reject(&mut self, Env, Error)`（出错时调用，返回 `Ok` 即恢复为 fulfill）；`finally(self, Env)`（resolve / reject 后清理）。
- AbortSignal：libuv 启动前 abort 可取消排队任务，Promise reject 且错误 `name` 为 `AbortError`；已开始无法取消 `compute`。适配器在转换参数时安装 `signal.onabort`（覆盖已有 onabort，不检查 `signal.aborted`）。
- `ScopedTask<'env>`：`resolve` / `reject` 收 `&'env Env`，`JsValue` 可带环境生命周期。低层 `Env::spawn(task)` 也可派发。

```rust
pub struct AsyncFib { input: u32 }

#[napi]
impl Task for AsyncFib {
  type Output = u32;
  type JsValue = u32;
  fn compute(&mut self) -> Result<Self::Output> { Ok(fib(self.input)) }
  fn resolve(&mut self, _: Env, output: u32) -> Result<Self::JsValue> { Ok(output) }
}

#[napi]
pub fn async_fib(input: u32, signal: AbortSignal) -> AsyncTask<AsyncFib> {
  AsyncTask::with_signal(AsyncFib { input }, signal)
}
// export function asyncFib(input: number, signal: AbortSignal): Promise<number>
```

## ThreadsafeFunction

- 用途：从其他线程调用 JS 回调（`napi_env` / `napi_value` / `napi_ref` 不能跨线程）。参数位置接收 `ThreadsafeFunction`，`Arc` clone 后在子线程 call。
- 泛型（按文档实例归纳）：第 1 参 = 传给回调的值；第 2 参 = JS 回调返回类型；第 4 参 = error status 类型（自定义需 `AsRef<str>`，必要时 `From<Status>`）；第 5 参 = CalleeHandled（默认 true）；第 6 参 = Weak（默认 false）；第 7 参 = MaxQueueSize（const）；第 3 参含义文档未明确命名。
- `CalleeHandled=true`（默认）：error-first 回调 `(err, value)`，必须以 `Result` 调用：`call(Ok(v))` / `call(Err(...))`。`false`：回调无错误参数、`call(v)` 直接传值，Rust 线程错误无法回传 JS。
- 回调参数 / 返回类型必须 `'static`（回调可能在导出函数返回后才运行）；scoped 值（`Unknown<'env>` 等）作回调参数会 E0521，先转 owned（`String` / `Buffer` / owned struct）再跨线程。
- `Weak=true`：该 tsfn 不再是事件循环存活理由，Node 可能提前退出。`MaxQueueSize=N`：队列满时 `Blocking` 等待空间、`NonBlocking` 立即返回 `Status::QueueFull`（检查 call 返回值）。
- 需要回调返回值：`call_with_return_value(Ok(1), Mode::Blocking, |ret, _| Ok(()))`；或 `call_async` / `call_async_catch`（CalleeHandled=false 时同步 throw 走 `napi_fatal_exception`，要结果必须用 `call_async_catch`）。

```rust
#[napi]
pub fn call_threadsafe_function(callback: ThreadsafeFunction<u32, ()>) -> Result<()> {
  let tsfn = Arc::new(callback);
  thread::spawn(move || tsfn.call(Ok(1), ThreadsafeFunctionCallMode::Blocking));
  Ok(())
}

// 由 Function 构建（回调参数可与传入值不同）
let tsfn = callback.build_threadsafe_function()
  .build_callback(|ctx: ThreadsafeCallContext<Data>| Ok(format!("Hello {}", ctx.value.name)))?;
tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
```

- `Function::build_threadsafe_function()` 返回 builder，默认 `max_queue_size=0`、`weak=false`、`callee_handled=true`、`error_status=napi::Status`。

## 迭代器协议

同步 `#[napi(iterator)]` + 实现 `Generator` / `ScopedGenerator`（基础 napi API，无额外 feature）；异步 `#[napi(async_iterator)]` + 实现 `AsyncGenerator`（需 `tokio_rt` / `async`）。两标记互斥；标记类不能有公开字段 `next` / `return` / `throw`（协议方法由 napi-rs 安装）。

```rust
#[napi]
impl Generator for Counter {
  type Yield = u32; type Next = u32; type Return = ();
  // Some(v) → {value, done:false}; None → {done:true}
  fn next(&mut self, value: Option<Self::Next>) -> Option<Self::Yield> { todo!() }
}
// 声明：class Counter extends Iterator<u32, void, u32>
```

- 约束：`Yield: ToNapiValue`；`Next: FromNapiValue`（`next(v)` 可选实参）；`Return: FromNapiValue`。适配器不记忆自然完成，之后的 `next()` 会再次调 Rust——需要保持 done 就自己存状态继续返回 `None`。
- 提前关闭（`for...of` break）→ 重写 `fn complete(&mut self, Option<Self::Return>) -> Option<Self::Yield>`（当前仅作清理 hook，返回值不暴露）。
- `throw(err)` 默认把原 JS 值作 `Err` 抛出并完成迭代；自定义 `fn catch<'env>(&'env mut self, Env, Unknown<'env>) -> std::result::Result<Option<Self::Yield>, Unknown<'env>>`（注意用 `std::result::Result`，错误侧是 `Unknown`）。
- `ScopedGenerator<'env>`：yield 借用当前 JS 环境的值（`type Yield = Object<'env>`）；scoped 值在 JS 线程立即转换，不得存入 class 或移到其他线程。
- 异步 `AsyncGenerator`：`fn next(&mut self, ...) -> impl Future<Output = Result<Option<u32>>> + Send + 'static`。**future 不能借 `self`**——同步更新状态后把所需数据 copy/clone 进 `async move`；scoped 类型（`Object<'env>` / `BufferSlice<'env>`）不能作 Yield。
- 异步陷阱：适配器不持久终态标志；并发多个 in-flight `next()` 不保证串行化，需设计并发安全状态机。异步 `complete` 的运行时 final value 是 `Option<Yield>` 而声明类型是 `Return`——若可能返回 `Some`，让 `Yield` = `Return` 同型。
- GC：`[Symbol.asyncIterator]()` 产生的迭代器持隐藏强引用防 class 被收集，迭代器 finalize 时释放；不取消 in-flight future，清理需幂等。同步迭代器就是 class 实例本身。

## 错误处理

- 约定：预期失败以 `Result<T>`（= `std::result::Result<T, Error>`）跨界；同步 fn `Err` → throw，async fn `Err` → Promise reject。
- `Error { status: Status, reason: String, cause: Option<Box<Error>> }`：`reason` → `error.message`；`status.as_ref()` → `error.code`（Status 是 Node-API 状态，非应用错误分类）；`cause` → `error.cause`。创建：`Error::from_reason(msg)` / `Error::new(Status::InvalidArg, msg)`；链式 `error.set_cause(Error::from(source))`。内置转换：`std::io::Error`、`std::ffi::NulError`。
- 参数转换在 Rust 函数调用前进行：类型错即 throw（与返回类型是否 `Result` 无关）。async fn 参数转换失败同步 throw（Promise 创建前）；`#[napi(return_if_invalid)]` 改为同步返回 undefined（d.ts 仍标 `Promise<T>`）。
- 自定义错误码：`Error<S>` 接受任意 `S: AsRef<str>`，只设 `error.code`、不改 JS 错误子类。低层 API 需要 `Status` 转自定义类型时另 `impl From<Status>`。
- JS 错误子类：`env.throw_range_error("...", Some("ERR_RANGE"))` 等 helpers（`throw_syntax_error` 需 napi9）。**调用 `Env::throw_*` 后必须立即 return**（环境有 pending exception）。
- 保留 JS 异常：`Err(unknown.into())` 记录 message/cause 并（native 构建）retain 原值，转回时复用原对象（保留子类 / stack / 自定义属性）；跨环境 / 线程时重建 fresh generic Error。
- `error_anyhow` feature：`anyhow::Error` → Error（GenericFailure，错误链格式化进 reason）。要稳定机器可读 code 就手动 map 领域错误。
- panic 不是普通错误：FFI 边界未捕获 panic 可终止进程。`#[napi(catch_unwind)]` 包 `std::panic::catch_unwind`，unwind payload → GenericFailure；仅 unwind 构建有效、部分操作不 unwind 直接 abort、只捕获该生成边界（不管 detached 线程）。轮询 Tokio 任务时的 panic 由 runtime 观察并通常 reject deferred Promise。
- 异步栈：跨线程后错误栈从 rejection 点开始；`deferred_trace` feature 在创建 deferred 时捕获 JS error 并在拒绝时复用调用侧栈（每个受影响 deferred 有开销）。

## 引用与生命周期

- 默认规则：JS 值句柄只在 native 调用的 handle scope 内有效。owned primitive（`bool` / 整数 / `String`）是值拷贝；`JsNumber<'env>` / `JsString<'env>` 是 scoped 句柄，出不了调用 scope。
- 类实例生命周期流：JS `new Engine` → Rust 构造 → Box + `napi_wrap` 附到 JS 实例 → 传回 Rust 时 `napi_unwrap` 得 `&Engine` / `&mut Engine` → JS GC → `napi_finalize_cb` 删除 Rust struct。
- `Reference<T>` / `WeakReference` 包装 `napi_ref`，**非 Send**（drop 必须在创建线程）。`Reference<T>` 作参数注入 `#[napi]` 函数（与 `Env` / `This` 同类），使 Node 持有实例直到所有引用 drop。
- `share_with(env, closure)`：借引用构造 `'static` 数据，产出 `SharedReference<T, U>`（一个 napi class 持另一个 class 内部数据的方式）。`WeakReference`：`reference.downgrade()` 生成、`upgrade(env)? -> Option<T>`，用于循环引用。
- `ObjectRef`：`options.create_ref()?` 存入 struct → `ref.get_value(env)` 取回 → **必须返回给 JS 或显式 `unref(env)`**（仅 drop 只报 leak、删不掉 `napi_ref`）。`SymbolRef` / `ExternalRef` 同理；`FunctionRef` 例外地 `Send + Sync`，借回用 `borrow_back(&env)`（生命周期逃逸场景：`promise.finally(move |env| ...)` 里先 `create_ref()` 再 borrow back）。
- 清理机制（napi4）：owning JS 线程上 drop → 直接 unref；其他线程 drop → `napi_ref` 排队到 owning 环境的内部 TSFN、由其 JS 线程释放；Rust 侧释放仅使 JS 值可 GC。

## cargo features

- napi 默认 `default = ["napi4", "dyn-symbols"]`。feature 是编译期能力而非运行时 polyfill：host 不提供的 Node-API 函数即使 `dyn-symbols` 让库加载也不可调用。运行时核对 `process.versions.napi`。
- 级别门槛（累积）：napi2 uv event loop；napi3 cleanup hooks；napi4 ThreadsafeFunction / deferred / 跨线程 ref 清理；napi5 Date / finalizers；napi6 BigInt / instance data / type-tagging 相关；napi8 async cleanup hooks / object freeze+seal / type-tagging；napi9 全局 symbol / 模块文件名 / SyntaxError；napi10 external Latin-1/UTF-16 字符串。
- async / tokio：`tokio_rt` = Tokio 集成（implies `napi4`，导出 `async fn` 必需）；`async` = `tokio_rt` 别名；`tokio_full` = Tokio `full`（大依赖，不替代 tokio_rt）；组件 features（`tokio_fs` / `tokio_time` 等）**不 imply tokio_rt**，需显式列出。
- 转换：`serde-json`、`serde-json-ordered`、`chrono_date`（implies chrono + napi5）、`latin1`、`object_indexmap`、`web_stream`（implies futures-core / tokio-stream / tokio_rt / napi4，运行时需 Web Streams globals，Node 18+）、`error_anyhow`。
- 诊断 / 兼容：`deferred_trace`（跨线程 rejection 保留调用侧栈）；`tracing`（还需 `napi-derive/tracing` 才生成回调事件）；`compat-mode`（v2 遗留）；`experimental`；`noop`（不加载 Node 编译/测试纯 Rust 逻辑，生产不用）。iterator 属性虽标 experimental 但**不受** `experimental` feature 控制。
- napi-derive：`type-def`（默认开，生成 .d.ts）、`strict`（默认开，编译期报未使用的 `#[napi]` 选项——区别于运行时校验 JS 值的 `#[napi(strict)]` 属性）、`tracing` / `compat-mode` / `noop`（默认关）。
- 本项目用的 `napi6` + `async` 组合：有 BigInt / instance data / async fn；无 napi8（type-tagging 不生效）、无 tokio 组件（`tokio::fs` 等不可用，需要再开）。

---

# `#[napi]` 属性速查

## 命名与导出

| 选项 | 目标 | 运行时效果 | TS 效果 |
|---|---|---|---|
| `js_name = "name"` | Function / Method / struct / enum / const / type alias / field / module | 替换默认的 camelCase 函数/成员名或 PascalCase 类型名；mod 上是命名空间对象名。type alias 无运行时导出 | 使用同一导出名；type alias 上只重命名声明 |
| `namespace = "name"` | Function / struct / impl / enum / const / type alias | 注册到 `exports.name` 下。class 与其 impl 块必须用同一个 namespace。type alias 无运行时注册 | 声明放进同一命名空间；type alias 上是唯一效果 |
| `module_exports` | 仅自由函数 | 模块初始化期间以 exports 对象执行该函数 | 不生成函数声明 |
| `no_export` | 仅自由函数 | 生成 Node-API 回调包装但不注册到 exports，用于把 `*_c_callback` 传给低层 API | 不生成声明 |

内联 Rust 模块可以变成 JS namespace：只有同样带 `#[napi]` 的子项被导出，不支持嵌套 napi 模块。

```rust
#[napi(js_name = "math")]
mod arithmetic {
  #[napi]
  pub fn add(a: u32, b: u32) -> u32 {
    a + b
  }
}
// export namespace math { export function add(a: number, b: number): number }
```

`module_exports` 约束：必须是非泛型自由函数；参数只接受 `Env`、`Object` 或其二者的引用；返回值只接受 `()` 或 `Result<()>`。不能与 `constructor`、`factory`、`getter`、`setter`、`js_name`、`strict`、`return_if_invalid`、`no_export` 组合。

```rust
#[napi(module_exports)]
pub fn initialize(mut exports: Object) -> Result<()> {
  exports.set("build", "release")?;
  Ok(())
}
```

## 函数与方法

| 选项 | 运行时效果 | TS 效果 |
|---|---|---|
| `constructor` | 暴露 JS 构造器，不能 async。struct 简写形式下 public 字段变成构造参数 | 生成 `constructor(...)` |
| `factory` | 关联方法（返回 `Self`/`Result<Self>`）暴露为静态工厂，可以 async | 生成静态方法，返回类型为类或 `Promise<Class>` |
| `getter` / `getter = name` | JS property getter；不带名字时 `get_value` → `value` | 生成 get 访问器 |
| `setter` / `setter = name` | JS property setter；不带名字时 `set_value` → `value` | 生成 set 访问器 |
| `strict` | 转换前对每个 JS 参数调 `ValidateNapiValue`，不匹配则抛错 | 无 |
| `return_if_invalid` | 做校验，但对非法参数返回 `undefined` 而不是抛错 | 无 |
| `catch_unwind` | 在生成的回调边界捕获 unwinding panic，把 payload 转成 JS `Error` | 无 |
| `async_runtime` | 同步函数/方法执行期间进入 napi-rs 的 Tokio 运行时（运行时未启用时是 no-op）；配合 `napi/tokio_rt` 使用 | 无 |
| `enumerable = false` | 清除属性描述符的 enumerable 位（省略 `= 值` 等价 `true`） | 无 |
| `writable = false` / `configurable = false` | 同上，作用于 writable / configurable 位 | 无 |

关键约束：

- `strict` 与 `return_if_invalid` **互斥**。二者校验的是 Rust 类型的 `ValidateNapiValue` 实现，不是任意 schema 校验；嵌套 `Vec<T>` 的元素逐个转换，初始数组检查通过后转换仍可能失败。
- 校验发生在生成的 JS 回调里、async Rust future 创建之前。async 导出上 `strict` 可以同步抛错；`return_if_invalid` 对非法输入同步返回 `undefined` 而不是 Promise。二者不改变 async 返回类型的声明，异常路径要自行在文档中说明。
- `catch_unwind` 不是进程安全边界：捕获不了 abort panic，Rust 也不保证所有 panic 可 unwind。预期内的失败用 `Result`。

## 类与值形状

| 选项 | 目标 | 运行时效果 | TS 效果 |
|---|---|---|---|
| `object` | struct | 与 JS 对象互转，所有字段必须 pub，没有 class 身份 | 生成 interface |
| `array` | tuple struct | 与 JS 数组互转 | 生成 tuple 类型 |
| `transparent` | 单字段 tuple struct | 转换委托给内部字段，不产生包装对象 | 生成内部类型的别名 |
| `object_from_js = false` | object / array / transparent struct / enum | 不生成 `FromNapiValue`（不能从 JS 接收） | 无 |
| `object_to_js = false` | object / array / transparent struct / enum | 不生成 `ToNapiValue`（不能返回给 JS） | 无 |
| `use_nullable`（或 `use_nullable = true`） | Class / object / array / 结构化 enum | object 与结构化 enum 字段：`None` 输出为 `null` 而不是省略属性，且输入属性必填；array：写入/要求 tuple 索引而不是留洞。Class 的访问器与 constructor 转换不变 | 生成必填的 `T \| null` 属性或 tuple 元素；class 上这是唯一效果 |
| `custom_finalize` | Class struct | 不再生成默认的空 `ObjectFinalize`，类必须自己实现 | 无 |
| `type_tag = "salt"` | Class struct | 用 crate 内唯一 salt 替换类型标签里的 `crate@version` 分量 | 无（纯运行时） |
| `iterator` | Class struct | 每个实例实现同步迭代器协议 | 扩展 `Iterator<Yield, Return, Next>` |
| `async_iterator` | Class struct | 实现异步迭代器协议 | 添加 `[Symbol.asyncIterator](): AsyncGenerator<...>` |

方向控制是编译期开关：关掉一个方向就删掉对应的转换 trait 实现。用于只含回调的输入形状、或含无法从 JS 读回数据的输出形状。

```rust
#[napi(object, object_to_js = false)]
pub struct Request {
  pub path: String,
  pub on_chunk: ThreadsafeFunction<Buffer>,
}

#[napi(transparent)]
pub struct UserId(pub String);

#[napi(array)]
pub struct Point(pub f64, pub f64);
```

`Option<T>` 字段的默认语义（object / 结构化 enum）：缺属性 → `None`，输出 `None` → 省略属性；出现的值按内层 `T` 转换，所以 `null` / `undefined` 并非普遍接受。`use_nullable = true` 时：属性必填、`null` 转为 `None`、输出用 `null`；缺属性或 `undefined` 仍被拒绝。array 对"缺 tuple 索引"与"必填索引装 null"做同样的区分。class 上访问器和简写 constructor 参数走普通 `Option<T>` 转换、getter 对 `None` 返回 `null`，`use_nullable` 只改生成的 TS 形状。

## type_tag

每个 `#[napi]` class 派生一个 128 位类型标签，来源是内容身份串 `crate@version::module_path::ClassName`。在 napi8 原生构建上，标签在 `napi_wrap` 之后立即盖到每个实例的 JS 对象上，并在每次盲目指针转换（方法接收者、`&T` / `&mut T` 参数、`ClassInstance<T>`）之前校验。错类、原型伪装、`method.call(wrongThis)` 会被拒绝并抛可捕获的 `Value is not an instance of class`，而不是发生类型混淆的 cast。Rust 不允许重复的 `module_path::ident`，所以即使两个类共享 js_name 和 namespace，标签也必然不同。

默认身份以 `crate@version` 为键：两个不相关的 addon 若 crate 名、版本、模块路径、类名全部相同，会派生相同标签。`type_tag = "..."` 用 crate 内唯一 salt 替换 `crate@version` 分量（标签变为 `salt::module_path::ClassName`，salt 用 UUID）：

```rust
#[napi(type_tag = "6f9619ff-8b86-d011-b42d-00cf4fc964ff")]
pub struct MyClass {
  pub value: i32,
}
```

salt 是编译期常量，标签跨进程重载、跨同 addon 的两份独立加载保持稳定。该属性只在运行时生效，不会出现在生成的 TS 里。crate 名可能被广泛 vendored（或与无关 addon 同进程加载）时才需要它，否则默认派生足够。

## 字段

| 选项 | 目标 | 运行时效果 | TS 效果 |
|---|---|---|---|
| `js_name = "name"` | struct 或结构化 enum 的字段 | JS 侧改用该属性名 | 声明用改后的属性名 |
| `skip` | Class 或值形状字段 | class：省略生成的属性访问器（值形状转换仍读写该字段） | 省略该字段 |
| `readonly` | Class 或值形状字段 | class：只生成 getter；不改值形状转换 | 加 `readonly` |
| `writable` / `enumerable` / `configurable` | 暴露的字段 | 控制 class 属性描述符位。object 与结构化 enum 输出永远是 writable + enumerable + configurable 的数据属性 | 无 |
| `ts_type = "..."` | 暴露的字段 | 无 | 替换推断的字段类型 |
| `skip_typescript` | 暴露的字段 | 字段运行时仍在 | 只在声明中省略该字段 |

- 普通 class 上 `skip` 删掉 JS 访问器，`skip_typescript` 保留访问器只藏声明。object / array / 结构化 enum 上 `skip` 和 `readonly` 只影响声明，运行时转换照常处理字段。
- **陷阱**：`skip` 与 `#[napi(constructor)]` struct 简写一起用会出错——生成的 constructor 仍会消费每个字段，但该字段已不在 TS 签名里。

## 枚举

| 选项 | 目标 | 运行时效果 | TS 效果 |
|---|---|---|---|
| `string_enum`（或 `= "case"`） | 无字段枚举 | 变体转字符串而非整数值；`= "case"` 时按所选命名风格转换变体名 | 生成字符串值枚举成员 |
| `value = "literal"` | string_enum 的变体 | 覆盖该变体的 JS 字符串 | 用字面量值 |
| `discriminant = "key"` | 结构化 enum | 把判别属性从默认 `type` 改为指定名 | 判别联合用同一属性 |
| `discriminant_case = "case"` | 结构化 enum | 改变判别值里变体名的编码方式 | 同上 |
| `use_nullable` | 结构化 enum | 对变体字段应用 nullable 字段行为 | 控制可选属性 vs `T \| null` |
| `object_from_js` / `object_to_js` | 任意 enum | 开关某一方向的生成转换 | 无 |

```rust
#[napi(string_enum = "kebab-case")]
pub enum Mode {
  ReadOnly,
  #[napi(value = "read-write")]
  Writable,
}

#[napi(discriminant = "kind", discriminant_case = "camelCase")]
pub enum Event {
  Ready,
  FileChanged { path: String },
  Progress(u32, u32),
}
```

- 接受的 case 名：`lowercase`、`UPPERCASE`、`PascalCase`、`camelCase`、`snake_case`、`UPPER_SNAKE`、`kebab-case`、`UPPER-KEBAB-CASE`。
- `string_enum` 只接受无字段变体，不能与显式 Rust 判别值组合。
- 含任何数据变体的枚举是**结构化 enum**：每个变体变成"判别符 + 自身字段"的对象；JS 名与判别符相同的字段会被拒绝。

## TypeScript 覆写

| 选项 | 目标 | 声明效果 | 关键约束 |
|---|---|---|---|
| `ts_arg_type = "..."` | 单个函数参数 | 替换该参数的推断类型 | 参数级上下文属性；与函数级 `ts_args_type` 互斥 |
| `ts_args_type = "..."` | Function / Method | 替换整个逗号分隔参数列表 | 与一切参数级 `ts_arg_type` 互斥 |
| `ts_return_type = "..."` | Function / Method | 替换推断的返回类型。async 函数要写完整的预期类型，一般是 `Promise<T>` | |
| `ts_generic_types = "..."` | Function / Method | 在参数前插入 `<...>` 里的内容 | 必须是合法的 TS 泛型参数语法 |
| `ts_type = "..."` | Function / Method 或 field | 函数：替换导出名之后的整个签名后缀；字段：替换其类型 | 函数级不能与 `ts_args_type` / `ts_return_type` 组合；它会连泛型段一起替换，泛型要写进 `ts_type` 里，不能与 `ts_generic_types` 组合 |
| `skip_typescript` | Function / Method / field / enum / const / type alias | 保留运行时导出，省略声明。type alias 本无运行时导出，等于整个消失 | 不能用在整个 struct 或 impl 块上 |

```rust
#[napi(
  ts_generic_types = "T",
  ts_args_type = "value: T",
  ts_return_type = "T"
)]
pub fn identity<'env>(value: Unknown<'env>) -> Unknown<'env> {
  value
}

#[napi(ts_type = "(operation: 'add' | 'subtract', a: number, b: number): number")]
pub fn calculate(operation: String, a: i32, b: i32) -> i32 {
  match operation.as_str() {
    "add" => a + b,
    "subtract" => a - b,
    _ => 0,
  }
}
```

这些字符串会被原样插入声明，napi-rs 既不解析为 TypeScript 也不验证它们与运行时行为一致。保持运行时转换为权威，并对生成的 .d.ts 做测试。要在生成文件顶部追加自定义类型或 import，用 NAPI 配置的声明文件头（`dtsHeader` / `dtsHeaderFile`），这是文件级设置，与上述按导出的覆写无关。

## 选项索引

通用解析器接受：`catch_unwind`、`async_runtime`、`module_exports`、`js_name`、`constructor`、`factory`、`getter`、`setter`、`readonly`、`enumerable`、`writable`、`configurable`、`skip`、`strict`、`return_if_invalid`、`object`、`object_from_js`、`object_to_js`、`custom_finalize`、`namespace`、`type_tag`、`iterator`、`async_iterator`、`ts_args_type`、`ts_return_type`、`ts_type`、`ts_generic_types`、`string_enum`、`use_nullable`、`discriminant`、`discriminant_case`、`transparent`、`array`、`no_export`、`skip_typescript`。

上下文解析器额外接受：函数参数上的 `ts_arg_type`，string-enum 变体上的 `value`。
