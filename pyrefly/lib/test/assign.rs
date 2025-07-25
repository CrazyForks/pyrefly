/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::test::util::TestEnv;
use crate::testcase;

testcase!(
    test_subscript_unpack_assign,
    r#"
from typing import assert_type

x: list[int] = [0, 1, 2]
x[0], x[1] = 3, 4
x[0], x[1] = 3, "foo"  # E: Cannot set item in `list[int]`\n  No matching overload found for function `list.__setitem__`
"#,
);

testcase!(
    test_subscript_assign,
    r#"
from typing import assert_type

x = []
x[0] = 1
assert_type(x, list[int])

y = [1, 2, 3]
y[0] = 1
assert_type(y, list[int])

z = [1, 2, 3]
z[0] = "oops"  # E: No matching overload found

a: int = 1
a[0] = 1  # E: `int` has no attribute `__setitem__`

def f(x: int) -> None:
    x[0] = 1  # E: `int` has no attribute `__setitem__`
"#,
);

testcase!(
    test_error_assign,
    r#"
x: str = 1  # E: `Literal[1]` is not assignable to `str`
y = x
"#,
);

testcase!(
    test_reassign_myself,
    r#"
from typing import assert_type
x = [1, 2, 3]
for x in x:
    pass
assert_type(x, int)
"#,
);

testcase!(
    test_class_var_assign_from_instance,
    r#"
from typing import ClassVar

class C:
    x: ClassVar[int] = 1
c = C()
c.x = 2  # E: Cannot set field `x`
"#,
);

testcase!(
    test_class_var_assign_from_class,
    r#"
from typing import ClassVar

class C:
    x: ClassVar[int] = 1
C.x = 2
"#,
);

testcase!(
    test_assign_twice_empty,
    r#"
from typing import assert_type, Any
def b() -> bool:
    return True

if b():
    x = []
else:
    x = [3]
y = x
assert_type(y, list[int] | list[Any])
"#,
);

testcase!(
    test_assign_widen,
    r#"
from typing import Literal, LiteralString, Any
a: Literal['test'] = "test"
b: LiteralString = "test"
c: str = "test"
d: Any = "test"
"#,
);

testcase!(
    test_assign_widen_list,
    r#"
from typing import Literal, LiteralString, Any
a: list[Literal['test']] = ["test"]
b: list[LiteralString] = ["test"]
c: list[str] = ["test"]
d: list[Any] = ["test"]
"#,
);

testcase!(
    test_assign_at_types,
    r#"
a: int = 3
a = "test"  # E: `Literal['test']` is not assignable to variable `a` with type `int`
"#,
);

testcase!(
    test_optional_assign,
    r#"
from typing import Optional
x: Optional[int] = 42
y: Optional[str] = 43  # E: `Literal[43]` is not assignable to `str | None`
    "#,
);

testcase!(
    test_assign_ellipse,
    TestEnv::one_with_path("foo", "foo.pyi", "x: int = ..."),
    r#"
from typing import assert_type
from types import EllipsisType
from foo import x
assert_type(x, int)
y: int = ...  # E: `Ellipsis` is not assignable to `int`
z: EllipsisType = ...
"#,
);

testcase!(
    test_assign_unpack,
    r#"
from typing import assert_type, Literal
a, b = (1, "test")
assert_type(a, Literal[1])
assert_type(b, Literal["test"])
    "#,
);

testcase!(
    test_assign_unpack_unpack,
    r#"
from typing import assert_type, Literal
(a, b), c, d = ((1, "test"), 2, 3)
assert_type(a, Literal[1])
assert_type(b, Literal["test"])
assert_type(c, Literal[2])
assert_type(d, Literal[3])
    "#,
);

testcase!(
    test_assign_unpack_ambiguous,
    r#"
from typing import assert_type
def f(x: list[str]):
    a, b = x
    assert_type(a, str)
    assert_type(b, str)
    "#,
);

testcase!(
    test_assign_multiple,
    r#"
from typing import assert_type, Literal
a = b = 1
assert_type(a, Literal[1])
assert_type(b, Literal[1])
    "#,
);

testcase!(
    test_assign_list,
    r#"
from typing import assert_type, Literal
[a, b] = (1, "test")
assert_type(a, Literal[1])
assert_type(b, Literal["test"])
    "#,
);

testcase!(
    test_unpack_too_many,
    r#"
(a, b, c, d) = (1, 2)  # E: Cannot unpack tuple[Literal[1], Literal[2]] (of size 2) into 4 values
    "#,
);

testcase!(
    test_unpack_not_enough,
    r#"
(a,) = (1, 2)  # E: Cannot unpack tuple[Literal[1], Literal[2]] (of size 2) into 1 value
() = (1, 2)  # E: Cannot unpack tuple[Literal[1], Literal[2]] (of size 2) into 0 values
    "#,
);

testcase!(
    test_splat_back,
    r#"
from typing import assert_type, Literal
(a, b, *c) = (1, 2, 3, "test")
assert_type(a, Literal[1])
assert_type(b, Literal[2])
assert_type(c, list[Literal["test", 3]])
    "#,
);

testcase!(
    test_splat_front,
    r#"
from typing import assert_type, Literal
(*a, b, c) = (1, 2, 3, "test")
assert_type(a, list[Literal[1, 2]])
assert_type(b, Literal[3])
assert_type(c, Literal["test"])
    "#,
);

testcase!(
    test_splat_middle,
    r#"
from typing import assert_type, Literal
(a, *b, c) = (1, True, 2, "test")
assert_type(a, Literal[1])
assert_type(b, list[Literal[True, 2]])
assert_type(c, Literal["test"])
    "#,
);

testcase!(
    test_splat_unpack,
    r#"
from typing import assert_type, Literal
(a, *(b,)) = (1, 2)
assert_type(a, Literal[1])
assert_type(b, Literal[2])
    "#,
);

testcase!(
    test_splat_nothing,
    r#"
from typing import assert_type, Never
(*a,) = ()
assert_type(a, list[Never])
    "#,
);

testcase!(
    test_never,
    r#"
from typing import Any, Never, NoReturn
def foo(x: Never) -> Any:
    y: NoReturn = x
    z: int = x
    return x
def bar(x: Never) -> NoReturn:
    return x
    "#,
);

testcase!(
    test_splat_ambiguous,
    r#"
from typing import assert_type
def f(x: list[str]):
    a, *b = x
    assert_type(a, str)
    assert_type(b, list[str])
    "#,
);

testcase!(
    test_splat_error,
    r#"
a, *b = (1,)  # OK
a, *b = ()  # E: Cannot unpack tuple[()] (of size 0) into 1+ values
    "#,
);

testcase!(
    test_multiple_annotations,
    r#"
from typing import Literal
def f(cond: bool):
    x: int = 0
    if cond:
        x: int = 1  # OK
    y: int = 0
    if cond:
        y: str = "oops"  # E: `y` cannot be annotated with `str`, it is already defined with type `int`
    z: int = 0
    if cond:
        z: Literal[1] = 1  # E: `z` cannot be annotated with `Literal[1]`, it is already defined with type `int`
    "#,
);

testcase!(
    test_multiple_annotations_without_merge,
    r#"
x: int = 0
x: str = ""  # E: `x` cannot be annotated with `str`, it is already defined with type `int`
    "#,
);

testcase!(
    test_hoist_ann,
    r#"
x = 0 # E: `Literal[0]` is not assignable to variable `x` with type `str`
x: str = ""
    "#,
);

testcase!(
    test_aug_assign_illegal_targets,
    r#"
x = 42
(x,) += (42,)  # E: Parse error: Invalid augmented assignment target
[x] += [42]  # E: Parse error: Invalid augmented assignment target
*x += 42  # E: Parse error: Invalid augmented assignment target
    "#,
);

testcase!(
    test_annot_flow_assign,
    r#"
from typing import Literal
x: int = 0
lit0: Literal[0] = x  # E: `int` is not assignable to `Literal[0]`
x = 1
lit1: Literal[1] = x
x = "oops"  # E: `Literal['oops']` is not assignable to variable `x` with type `int`
lit2: Literal["oops"] = x  # E: `int` is not assignable to `Literal['oops']`
    "#,
);

testcase!(
    test_type_alias_simple,
    r#"
from typing import assert_type
type X = int
def f(x: X):
    assert_type(x, int)
    "#,
);

testcase!(
    test_type_alias_generic,
    r#"
from typing import assert_type
type X[T] = list[T]
def f(x: X[int]):
    assert_type(x, list[int])
    "#,
);

testcase!(
    test_assign_final,
    r#"
from typing import Final
x: Final   # E: Expected a type argument for `Final`
y: Final[int]  # OK
z: Final = 1  # OK
    "#,
);

testcase!(
    test_assign_final_attr,
    r#"
from typing import Final
class A:
    x: Final = 1
a = A()
a.x = 1  # E: Cannot set field `x`
del a.x  # E: Cannot delete field `x`
class B:
    x: Final[int] = 1
b = B()
b.x += 1  # E: Cannot set field `x`
    "#,
);

testcase!(
    test_aug_assign_integer,
    r#"
def f(x: int):
    x += 1
    "#,
);

testcase!(
    test_aug_assign_simple,
    r#"
x: list[int] = []
x += [1]
x += ["foo"]  # E: Augmented assignment produces a value of type `list[int | str]`, which is not assignable to `list[int]`
"#,
);

testcase!(
    test_aug_assign_unannotated,
    r#"
from typing import assert_type
x = [1]
x += ["foo"]
assert_type(x, list[int | str])
"#,
);

testcase!(
    test_aug_assign_final,
    r#"
from typing import Final
x: Final = [""]
x += [""]  # E: Cannot assign to var x because it is marked final
x[0] += ""
"#,
);

testcase!(
    test_aug_assign_fallback,
    r#"
class A:
    def __iadd__(self, other):
        return self
    def __add__(self, other):
        return self
class B:
    def __add__(self, other):
        return self
class C:
    def __radd__(self, other):
        return self
class D: pass

a = A()
b = B()
c = C()
d = D()

a += 1
b += 1
d += c
    "#,
);

testcase!(
    test_aug_assign_function,
    r#"
def foo(y: list[int]) -> None:
    y += [1]
    y += ["foo"]  # E: Augmented assignment produces a value of type `list[int | str]`, which is not assignable to `list[int]`
    z: list[int] = []
    z += [1]
    z += ["foo"]  # E: Augmented assignment produces a value of type `list[int | str]`, which is not assignable to `list[int]`
"#,
);

testcase!(
    test_aug_assign_attr,
    r#"
class C:
    foo: list[int]

    def __init__(self) -> None:
        self.foo = []

c: C = C()
c.foo += [1]
c.foo += ["foo"]  # E: `list[int | str]` is not assignable to attribute `foo` with type `list[int]`
"#,
);

testcase!(
    test_aug_assign_attr_self,
    r#"
class C:
    foo: list[int]

    def __init__(self) -> None:
        self.foo = []
        self.foo += [1]
        self.foo += ["foo"]  # E: `list[int | str]` is not assignable to attribute `foo` with type `list[int]`
"#,
);

testcase!(
    test_aug_assign_subscript,
    r#"
x: list[list[int]] = []
x += [[1]]
x[0] += [1]
x += [1]  # E: Augmented assignment produces a value of type `list[int | list[int]]`, which is not assignable to `list[list[int]]`
"#,
);

testcase!(
    test_assign_special_subtype,
    r#"
from types import NoneType, EllipsisType

def foo(a: tuple[int, ...], b: NoneType, c: EllipsisType) -> None:
    a2: tuple = a
    b = None
    b2: object = b
    b3: None = b
    b4: int | None = b
    c = ...
    c2: object = c
"#,
);

testcase!(
    test_subscript_assign_any_check_rhs,
    r#"
from typing import Any
def expect_str(x: str): ...
def test(x: Any):
    x[0] += expect_str(0) # E: Argument `Literal[0]` is not assignable to parameter `x` with type `str`
"#,
);

testcase!(
    test_aug_assign_any_check_rhs,
    r#"
from typing import Any
def expect_str(x: str): ...
def test(x: Any):
    x += expect_str(0) # E: Argument `Literal[0]` is not assignable to parameter `x` with type `str`
"#,
);

testcase!(
    test_aug_assign_error_not_class_check_rhs,
    r#"
from typing import Any
def expect_str(x: str) -> Any: ...
def test(x: Any):
    x += expect_str(0) # E: Argument `Literal[0]` is not assignable to parameter `x` with type `str`
"#,
);

testcase!(
    test_aug_assign_error_not_callable_check_rhs,
    r#"
from typing import Any
def expect_str(x: str) -> Any: ...
class C:
    __iadd__: None = None
def test(x: C):
    x += expect_str(0) # E: Argument `Literal[0]` is not assignable to parameter `x` with type `str`
"#,
);

testcase!(
    test_walrus_simple,
    r#"
from typing import assert_type, Literal
(x := True)
assert_type(x, Literal[True])
    "#,
);

testcase!(
    test_walrus_use_value,
    r#"
from typing import assert_type
class A: pass
class B(A): pass

y1 = (x1 := B())
assert_type(y1, B)

y2: A = (x2 := B())

y3: list[A] = (x3 := [B()]) # E: `list[B]` is not assignable to `list[A]`

y4: B = (x4 := A())  # E: `A` is not assignable to `B`
    "#,
);

testcase!(
    test_walrus_annotated_target,
    r#"
from typing import assert_type
class A: pass
class B(A): pass

x1: A
(x1 := B())

x2: list[A]
(x2 := [B()])

x3: B
(x3 := A())  # E: `A` is not assignable to variable `x3` with type `B`
    "#,
);

testcase!(
    test_use_before_write,
    r#"
y  # E: `y` is uninitialized
y = 42
    "#,
);

testcase!(
    test_read_before_write,
    r#"
x = y  # E: `y` is uninitialized
y = 42
    "#,
);

testcase!(
    test_invalid_walrus_target,
    r#"
class C:
    d: int
def foo(c: C):
    (c.d := 1)  # E: Parse error: Assignment expression target must be an identifier
    "#,
);

testcase!(
    test_walrus_inside_comprehension_if,
    r#"
from typing import assert_type
list1 = [1, 2, 3]
list2 = [elt for x in list1 if (elt := x)]
assert_type(list2, list[int])
    "#,
);

testcase!(
    test_side_effecting_comprehension_targets,
    r#"
class C:
    d: int
    def __setitem__(self, idx: int, value: int) -> None: ...
    def __init__(self):
        self.d = 0
def foo(c: C):
    # Perhaps surprisingly, this is actually legal (and is a side-effect at runtime)
    [1 for c.d in [1, 2, 3]]
    [1 for c[0] in [1, 2, 3]]
    # We expect type errors if the side-effect is a type error.
    [1 for c.d in ["1", "2", "3"]]  # E: `str` is not assignable to attribute `d` with type `int`
    [1 for c[0] in ["1", "2", "3"]]  # E: Argument `str` is not assignable to parameter `value` with type `int` in function `C.__setitem__`
    "#,
);

testcase!(
    test_assign_unpacked_with_existing_annotations,
    r#"
x: int
y: str
z: tuple[bool, ...]
x, *z, y = True, 1, 2, "test" # E: list[Literal[1, 2]]` is not assignable to `tuple[bool, ...]
    "#,
);

testcase!(
    test_assign_invalid,
    r#"
from typing import assert_type, Literal

a1+a2 = 7 # E: Could not find name `a1` # E: Could not find name `a2` # E: Parse error: Invalid assignment target
f() = 12 # E: Could not find name `f` # E: Parse error: Invalid assignment target
(42,) += (42,) # E: Parse error: Invalid augmented assignment target

type (b) = int # E: Could not find name `b` # E: Parse error: Invalid assignment target

# Type annotations on list/tuple are invalid, so ignore them
c1, c2: (bool, str) = 1, 2 # E: Parse error: Only single target (not tuple) can be annotated
assert_type(c1, Literal[1])
[d1, d2]: list[int] = ["test", "more"] # E: Parse error: Only single target (not list) can be annotated
assert_type(d1, str)

*e = ["test"] # E: starred assignment target must be in a list or tuple
assert_type(e, list[str])
"#,
);

testcase!(
    test_bad_annotated_assign,
    r#"
(x.x, y): tuple[int, str]  # E: Only single target (not tuple) can be annotated # E: Could not find name `x`
"#,
);

testcase!(
    test_assign_annotated_subscript,
    r#"
xs: list[int] = [1, 2, 3]
xs[0]: int = 3 # E: Subscripts should not be annotated
xs[1]: str = 3 # E: Subscripts should not be annotated
xs[2]: str = "test" # E: Subscripts should not be annotated # E: Cannot set item in
"#,
);

testcase!(
    test_assign_annotated_starred,
    r#"
*e: int = (42,)  # E: Parse error: Invalid annotated assignment target
"#,
);

testcase!(
    test_assign_subscript_wrong_type,
    r#"
xs: list[int] = [1, 2, 3]
xs[1] = "test" # E: Cannot set item in
"#,
);

// Tests how we display implicit Any types.
testcase!(
    test_reveal_error,
    r#"
from typing import reveal_type
x = oops # E: Could not find name `oops`
reveal_type(x) # E: revealed type: Unknown
    "#,
);

testcase!(
    test_reveal_type_assign,
    r#"
from typing import reveal_type

def f(x):
    x = 3
    x = "None"
    reveal_type(x) # E: revealed type: Literal['None']

def f(x):
    x = 3
    x = "None"
    "#,
);

testcase!(
    test_ann_assign_valid1,
    r#"
class A:
    _x: int
    def __init__(self, x:int):
        self._x: int = x
    "#,
);

testcase!(
    test_ann_assign_invalid,
    r#"
class A:
    _x: bool 
    def __init__(self, x:int):
        self._x: int = x # E: `int` is not assignable to attribute `_x` with type `bool`
    "#,
);

testcase!(
    test_ann_assign_valid2,
    r#"
int2 = int
class A:
    _x: int2
    def __init__(self, x:int):
        self._x: int = x 
    "#,
);

testcase!(
    test_ann_assign_twice,
    r#"
class A:
    _x: bool 
    def __init__(self, x:int):
        self._x: int = x # E: `int` is not assignable to attribute `_x` with type `bool`
        self._x: bool = x # E: `int` is not assignable to attribute `_x` with type `bool`
    "#,
);
