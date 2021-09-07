initSidebarItems({"enum":[["Either","The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases."]],"macro":[["try_left","Macro for unwrapping the left side of an `Either`, which fails early with the opposite side. Can only be used in functions that return `Either` because of the early return of `Right` that it provides."],["try_right","Dual to `try_left!`, see its documentation for more information."]],"mod":[["serde_untagged","Untagged serialization/deserialization support for Either<L, R>."],["serde_untagged_optional","Untagged serialization/deserialization support for Option<Either<L, R>>."]]});