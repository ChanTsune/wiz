# Wiz Core v0.1 — semantic specification draft

Status: **Proposal, not an implemented or approved language standard**  
Scope: Wiz `main` branch, reviewed 2026-11-03.  
Repository: https://github.com/ChanTsune/wiz  
This document is a proposed future language contract. Existing parsing, typing, LLVM lowering, and the standard library do not yet satisfy this contract. All code snippets are illustrative pseudo-Wiz unless explicitly identified as existing code.

## 0. Classification

- **DECIDED**: explicitly selected design direction: ownership/borrowing/lifetime static checks as the ultimate goal; length-delimited UTF-8 strings.
- **PROPOSED**: concrete rules recommended here, subject to language owner approval.
- **DEFERRED**: feature not promised by Core v0.1, even if grammar/AST currently recognizes it.

Terms **MUST**, **MUST NOT**, and **SHOULD** indicate normative intentions of this draft, not present compiler guarantees. An implementation is not Core-conforming until it rejects ill-typed and unsafe programs without compiler crashes and has positive/negative tests for the rule.

## 1. Values, ownership, copying, and destruction [PROPOSED]

1. Values are statically typed. An owning value has exactly one live owning place, excluding explicit `Copy` types.
2. Local assignment/argument passing/return by value **moves** non-`Copy` values. Reads through a moved-from place are compile-time errors; the old place has no destructor obligation.
3. `Copy` values copy implicitly; numeric scalars, `Bool`, raw pointers, and shared references are `Copy`. `&mut T` and `String` are not `Copy`. Arrays are `Copy` if the element type is `Copy`. A user aggregate must explicitly opt into `Copy` and cannot own resources that require destruction.
4. `Clone` is an explicit operation and never a synonym for a bitwise copy of an owner.
5. Every initialized owning value is destroyed exactly once when its lifetime ends normally, including early `return`. Destruction of locals occurs in reverse order of completed initialization. Field destruction order is provisionally reverse declaration order, pending a final ABI and destructor review. Abort/unwind behavior is DEFERRED.
6. Partial moves, self-referential owning structures, and custom destructors in the initial Core implementation are DEFERRED; the checker should reject unsupported cases rather than silently alias or duplicate ownership. Resource types require a compiler-supported destructor protocol before being considered safe Core types.
7. Raw pointers do **not** imply ownership. Allocation/deallocation ownership must be wrapped in an owning resource type (e.g., `String`, `Vector`, `FileDescriptor`). Explicit `close()` should consume or invalidate a resource to prevent a later double close.

## 2. `val`, `var`, and access permissions [PROPOSED]

1. `val x = value`: initialized once, not rebound, and cannot be mutably borrowed in safe code.
2. `var x = value`: reassignable when no live borrow forbids it; can be mutably borrowed.
3. `val` and `var` are rules for **places**, not claims about external memory reachable through unsafe raw pointers. A `val` field cannot be reassigned or mutably borrowed through ordinary safe field access; a modifiable stored field should be declared `var`.
4. Immutable sharing is allowed through `&T`; mutable access to the same place requires a unique `&mut T`.
5. Methods that mutate a receiver must require `&mut self`. Methods accepting `&self` must not mutate the receiver's reachable safe state except through a future explicit interior-mutability primitive with audited unsafe internals.
6. No automatic mutable borrowing of ordinary arguments. A method receiver may be implicitly borrowed only if the method-resolution algorithm can unambiguously identify a valid receiver mode, and must reject mutable receiver borrow from `val`.

## 3. References and lifetimes [PROPOSED]

Reference type design (syntax not yet implemented):

- `&'a T`: non-null, aligned, valid shared borrow for region `'a`.
- `&'a mut T`: non-null, aligned, valid exclusive borrow for region `'a`.
- `&T` / `&mut T`: inferred lifetime when unambiguous.
- Shared references are freely copyable but do not grant mutation.
- Mutable references are not copyable. Safe reborrowing is permitted, with exclusivity enforced for the active reborrow.
- A borrow cannot outlive its referent, coexist with conflicting accesses, or escape from a local variable into a longer-lived result.
- The checker SHOULD use last-use/non-lexical regions rather than unnecessarily holding every borrow to its lexical scope end.
- Elision: a returned reference is tied to `self` for methods; for free functions with exactly one borrowed input, it is tied to that input; multiple borrowed inputs require an explicit lifetime relationship.
- References embedded in structs require an explicit lifetime parameter or a compiler-proven equivalent. Lifetime variance and higher-ranked lifetime support are DEFERRED.
- `&str` is a borrow and follows exactly these rules; a heap-backed string slice cannot outlive its `String` owner.

## 4. Raw pointers, unsafe, and FFI [PROPOSED]

1. New types: `*const T` and `*mut T`. Existing `*T` must receive a deliberate migration strategy; no silent promotion to a safe reference is allowed.
2. Raw pointers can be null, dangling or aliased and do not carry compiler-enforced lifetimes. Converting a valid reference to a raw pointer is allowed, but dereferencing, offsetting without proof, or invoking an untrusted FFI function requires explicit `unsafe` authorization.
3. The purpose of `unsafe` is to make extra proof obligations local; it does not disable the soundness requirements of safe functions. Safe wrappers must verify preconditions before exposing valid references.
4. An array reference is not implicitly a pointer to its first element. Use an explicit `as_ptr`/`as_mut_ptr`-style operation or `&array[0]` when nonempty. The returned pointer is not an owning value and may only be dereferenced while its originating allocation remains valid.
5. C ABI declarations must be explicitly marked, e.g. `extern "C"`. `#[no_mangle]` alone does not express calling convention or transfer ownership.
6. C-compatible signatures use C-compatible scalars, structures and raw pointers. `&str`, `String`, and non-`repr(C)` aggregate representations MUST NOT silently cross a C ABI boundary.
7. APIs that retain a supplied pointer require a wrapper that tracks or transfers the resource lifetime. FFI return values that own memory must be wrapped in an owner with correct deallocation.

## 5. String values [DECIDED direction + PROPOSED details]

1. A `str` is a **possibly unsized sequence of well-formed UTF-8 bytes**; it is not a NUL-terminated C string. A `str` cannot be held directly as a sized local value; use `&str`.
2. An `&str` is conceptually `(data_pointer, byte_length)` and borrows a valid UTF-8 range. It is **not** C-ABI compatible; internal ABI/layout are not frozen by this document.
3. `String` is a uniquely owning, growable UTF-8 buffer; its conceptual state is `(pointer, byte_length, capacity, allocation owner)`, with `0 <= length <= capacity`, validated UTF-8 within the length, and exactly-once destruction.
4. UTF-8 string literals are `&'static str`. Embedded U+0000 is legal; its byte is counted in length, and it does **not** terminate the value.
5. `byte_length` counts bytes, not Unicode scalar values or grapheme clusters. Unicode scalar iteration is a separate operation. Direct `str[i]` integer indexing is disallowed in safe Core; byte access is explicit through a byte view, and substring creation validates UTF-8 codepoint boundaries.
6. Equality compares UTF-8 bytes. No implicit Unicode normalization, locale-dependent comparison, or grapheme segmentation occurs. Unicode-equivalent but differently normalized strings may compare unequal.
7. `String` mutation requires exclusive access. Any safe operation that creates or changes a string MUST preserve UTF-8 validity or return an error without producing an invalid `String`. **Its raw byte buffer must not be publicly mutable in safe code**; fields must be private/opaque or exposed only through checked operations. The current public `buffer` field would otherwise defeat the type invariant.
8. A slice of a `String` borrows its allocation. Growing/reallocating/mutating that allocation while a conflicting slice is live is prohibited by the borrow checker.
9. `CStr`/`CString` (names provisional) represent C interoperability. Conversion from `&str` to a C-compatible string rejects embedded NUL; a C string may need copying and a terminal NUL. Converting untrusted `(ptr,len)` to `&str` requires bounds and UTF-8 validation.
10. `strlen(&str)`, casts from an unvalidated raw pointer to `&str`, and byte-index slicing that can split a code point are invalid under this contract.

## 6. Other Core typing and evaluation [PROPOSED]

1. `Unit` means successful completion without a result value. An omitted function return type means `Unit` (existing syntax expectation). A non-`Unit` block-bodied function MUST return a suitable value on every reachable normal path.
2. Conditions for `if` and `while` MUST be `Bool`; implicit integer-to-boolean conversions are forbidden.
3. An `if` expression with no `else` has type `Unit`. With both branches producing values, both must have exactly the same type initially; implicit numeric promotions and branch-type joins are DEFERRED. The branch selected at runtime produces the result.
4. Array type `[T; N]` has fixed length `N`, and all elements must have type `T`. An empty array requires an expected element type. Array-to-pointer decay does not happen implicitly.
5. Arguments and operators are evaluated in source order, left to right. `&&` and `||` short-circuit; side-effectful right operands MUST NOT be evaluated when short-circuited.
6. Names resolve lexically, with local bindings before imported names. Ambiguous imports and ambiguous overload candidates are compile errors. An overload is determined by explicit labels and argument types; return type alone is not sufficient to overload. Ordinary arguments match without implicit borrow or lossy numeric coercion. An integer literal may take the sole viable candidate's expected integer type if in range; when multiple candidates are viable, prefer a unique exact default-literal-type match (`Int64`) and otherwise report ambiguity. Never choose by hash-map iteration order.
7. Unannotated integer literals initially default to `Int64`, floats to `Double` where no expected type exists. A contextual literal may be typed as the expected scalar if representable. No silent lossy coercion between named scalar types.
8. Signed and unsigned integers must use their corresponding comparisons, division, and remainder semantics. Integer overflow, division-by-zero, and float-to-integer out-of-range behavior require an explicit normative decision before stabilization. **Recommended** for fixed-width `+`, `-`, `*`: modulo 2^width; division by zero must trap or raise a defined failure; signed-min divided by -1 must be specified.
9. Default float semantics SHOULD follow IEEE-754 ordered comparisons, with `NaN == x` false and `NaN != x` true. Compiler lowering must not substitute an ordered-not-equal operation that makes `NaN != x` false.
10. Invalid input programs and unsupported constructs must emit structured compiler diagnostics, never `panic!`/`todo!()` or LLVM undefined behavior. This is a conformance condition separate from language semantics.

## 7. Deferred language features

- Generics, `protocol`, constrained extensions and dynamic dispatch: not Core until type checking is sound; current parser/AST presence is not a support guarantee.
- Closures that capture local references, async, threads, destructuring/partial moves, pinning, self-referential objects, trait-object ABI, custom unsized types.
- `null`, `as?`, `?.`, postfix `!`, `:?`: cannot be called stable without a typed `Optional`/failure contract. A bare `&T` or `&mut T` is non-null.
- `deinit` custom syntax and precise panic/unwind behavior; compiler must eventually implement an explicit exactly-once destructor contract.
- Advanced numeric coercion, overload ranking, and automatic borrow/deref chains.

## 8. Adversarial acceptance vectors

Illustrations are proposed and will not necessarily parse/compile today. **Accept** means should be accepted after implementing the contract; **Reject** means must be rejected safely with a compiler diagnostic.

| ID | Scenario | Expected | What must be established |
|---|---|---|---|
| O01 | `val v = Vector(); v.push(1)` where push needs `&mut self` | Reject | `val` cannot give mutable borrow |
| O02 | `var v = Vector(); v.push(1)` | Accept | mutable receiver borrow |
| O03 | `var x = 1; val r = &x; val m = &mut x; use(r)` | Reject | exclusive and shared borrows overlap |
| O04 | `fun leaked(): &str { val s = String::from("hi"); return s.as_str() }` | Reject | local borrow cannot escape |
| O05 | `fun consume(v: String) {} ; consume(s); consume(s)` | Reject | use after move |
| O06 | `val a = s.clone(); mutate(a); read(s)` | Accept | deep clone creates distinct owner |
| O07 | `return String::from("hello").as_str()` | Reject | temporary lifetime cannot escape |
| O08 | `var s = String::from("x"); val view = s.as_str(); s.push("y"); use(view)` | Reject | no realloc/mutation while borrowed |
| O09 | `val s = "A\0B"; assert(s.byte_length == 3)` | Accept | embedded NUL counted, not terminator |
| S10 | `"あ".byte_length == 3` | Accept | byte length not codepoint count |
| S11 | create `&str` over byte interval `[1,2)` within UTF-8 encoding of `"あ"` | Reject/error | UTF-8 boundary validated |
| S12 | `CString::from_str("A\0B")` | Defined error | interior NUL cannot become C string |
| S13 | safe cast `*UInt8` from malloc directly to `&str` without length/validation | Reject | raw pointer cannot assert valid UTF-8 borrow |
| F14 | `pipe(&fds as *Int32)` on `[Int32;2]` | Reject / migrate | no implicit array-reference decay |
| F15 | unsafe `pipe(fds.as_mut_ptr())` with valid `var fds` and C-compatible signature | Accept with unsafe proof | FFI receives contiguous writable memory; borrow/lifetime controlled |
| F16 | call C `strlen` with `&str` directly | Reject | `&str` is not C ABI |
| F17 | dereference null raw pointer in safe code | Reject | raw deref requires unsafe |
| V18 | copying owning `FileDescriptor` then closing both | Reject | unique owner, exactly-once release |
| E19 | `false && side_effect()` | Accept, no side effect | short circuit |
| E20 | `true || side_effect()` | Accept, no side effect | short circuit |
| T21 | `if true { 1 } else { 2.0 }` used as a value | Reject | mismatched branch types |
| T22 | `if 1 { ... }` | Reject | conditions require `Bool` |
| N23 | `UInt64` value >= 2^63 compared with small `UInt64` | Correct unsigned result | unsigned comparison |
| N24 | `NaN != NaN` | true | unordered-not-equal float semantics |
| D25 | early return after constructing an owning buffer | Exactly one destruction | RAII all normal exits |
| D26 | move an owning struct and exit both lexical scopes | Exactly one destruction | no double free |
| S27 | obtain writable raw byte access to `String.buffer` in safe code; write `0xff` | Reject | private UTF-8 representation invariant |
| O28 | call mutable method through `val` field of an owned struct | Reject | field `val` is not mutably borrowed |
| O29 | mutable reborrow ends at last use, then original owner is mutated | Accept | non-lexical lifetime soundness |
| F30 | FFI retains a pointer after local buffer is destroyed | Reject in safe wrapper / requires unsafe responsibility | ownership escape via FFI |

## 9. Confirmed code-level conflicts (static reading only)

- `libraries/std/src/collection/vector.wiz`: `push(&self)` mutates fields; `clone()` allocates `self.length` bytes but reports original `self.capacity`; no visible owning destructor.
- `libraries/std/src/string.wiz`: uses `strlen` on `&str`, assumes C string; `asStr()` constructs a borrow by raw cast; arbitrary byte-range `subString` does not verify UTF-8 boundaries.
- `libraries/libc/src/string.wiz`: `strlen(_ item: &str)` incorrectly exposes a C ABI raw-pointer function with a future fat-reference type.
- `libraries/std/src/io.wiz`: reads return heap-allocated `*UInt8` without explicit owner in signature.
- `wiz/wiz_syntax_parser/src/parser/wiz/type_.rs`: only `*T` and `&T` decorators; no `&mut` or lifetime grammar.
- `wiz/wiz_lexar/src/lib.rs`: lifetime tokens are lexed, but that is not semantic checking.
- `wiz/wizc/src/high_level_ir/type_checker.rs`: `if` Bool condition check is reversed; checking arrays compares whole-array type to each element type; block return exhaustiveness and safe mutation checks are missing.
- `wiz/wizc/src/high_level_ir/type_resolver/name_environment.rs`: overload selection depends on an unordered candidate set, without sufficient ranking / ambiguity diagnostics.
- `wiz/wizc/src/llvm_ir/codegen.rs`: `if` value PHI type is hardcoded to `i64`; binary operands are evaluated before deciding whether operator is `&&`/`||`; integers use signed div/rem/comparisons in common paths; float `!=` is lowered using ordered-not-equal.
- `wiz/wizc/src/high_level_ir.rs`: parsing a `deinit` ultimately reaches `todo!()`.

## 10. Decisions still required before freezing Core

1. **E1**: `val` fields: prohibit mutable borrow (recommended here), or permit mutating an owned field through `&mut self` while forbidding replacement?
2. **E2**: destruction interface: compiler-intrinsic `Drop` protocol, `deinit` syntax, or another hook; and exact field order.
3. **E3**: `*T` migration: reject in new Core, alias to `*mut T` temporarily, or retain as a distinct unchecked raw-pointer type.
4. **E4**: integer overflow and explicit cast failure semantics, including signed minimum divided by -1, narrowing, NaN and infinity.
5. **E5**: method receiver auto-borrow rules; no implicit argument borrowing is currently proposed.
6. **E6**: invalid byte-range slicing failure API, until `Result`/`Optional` semantics are stabilized.
7. **E7**: panics, traps and resource cleanup (abort vs unwind), and whether future `Drop` runs during unwinding.
8. **E8**: `String` and `str` API names, including `byte_length`, `as_str`, `bytes`, and checked slices.

## 11. Minimal ordered implementation slices (no repository mutation undertaken)

1. Add language-spec acceptance harness: parse/type-check/codegen/runtime positive-negative tests with stable diagnostic IDs. Ensure unsupported syntax errors do not panic.
2. Implement `&T`/`&mut T` type categories and place mutability, then enforce exclusive vs shared borrows and move tracking for simple locals and function calls.
3. Implement `String`/`&str` pair with `(ptr,len)` semantics and UTF-8 validation; split libc FFI bindings into explicit C pointer signatures; migrate core printing and formatting.
4. Add drop glue and resource wrappers, first for `String`/`UInt8Vector`/`FileDescriptor`; ensure transfers, clones, and early returns free exactly once.
5. Add lifetime elision and explicit relationships across returns and stored references; reject closures/advanced cases until designed.
6. Repair existing type/codegen defects (if, arrays, overload ambiguity, signed/unsigned operations, short circuit and Phi types) to meet Core contract.
7. Only after these steps, stabilize generic types, Protocol and Optional/Result; do not conflate parser acceptance with feature support.