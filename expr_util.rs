#[test]
fn we_can_prove_another_nested_slice_exec_with_no_rows() {
    let data = owned_table([
        bigint("a", [101, 105, 105, 105, 105]),
        bigint("b", [1, 2, 3, 4, 7]),
        int128("c", [1, 3, 3, 4, 5]),
        varchar("d", ["1", "2", "3", "4", "5"]),
        scalar("e", [1, 2, 3, 4, 5]),
    ]);
    println!("Input data: {data:?}");
    let t = "sxt.t".parse().unwrap();
    let mut accessor = OwnedTableTestAccessor::<InnerProductProof>::new_empty_with_setup(());
    accessor.add_table(t, data, 0);
    let const_expr = const_int128(105);
println!("Constructed const_int128: {const_expr:?}");

let aliased_const_expr = aliased_plan(const_expr, "const");
println!("Aliased const expression: {aliased_const_expr:?}");
    let expr = slice_exec(
        slice_exec(
            filter(
                vec![
                    col_expr_plan(t, "b", &accessor),
                    col_expr_plan(t, "c", &accessor),
                    col_expr_plan(t, "d", &accessor),
                    col_expr_plan(t, "e", &accessor),
                    aliased_plan(const_int128(105), "const"),
                    aliased_plan(
                        equal(column(t, "b", &accessor), column(t, "c", &accessor)),
                        "bool",
                    ),
                ],
                tab(t),
                equal(column(t, "a", &accessor), const_int128(105)),
            ),
            6,
            Some(3),
        ),
        3,
        None,
    );
    println!("Query expression: {expr:?}"); 
    let res = VerifiableQueryResult::new(&expr, &accessor, &());
    exercise_verification(&res, &expr, &accessor, t);
    let res_table = res.verify(&expr, &accessor, &()).unwrap().table;
    println!("Verified table result: {res_table:?}");
    let expected = owned_table([
        bigint("b", [0; 0]),
        int128("c", [0; 0]),
        varchar("d", [""; 0]),
        scalar("e", [0; 0]),
        int128("const", [0; 0]),
        boolean("bool", [true; 0]),
    ]);
    println!("Expected table: {expected:?}");
    assert_eq!(res_table, expected);
}








IMP:::
EXPR_UTI::::
use proof_of_sql_parser::intermediate_ast::Literal;
use sqlparser::ast::{
    BinaryOperator, Expr, Function, FunctionArg, FunctionArgExpr, Ident, ObjectName, UnaryOperator,
};

/// Compute the sum of an expression
#[must_use]
pub fn sum(expr: Expr) -> Expr {
    Expr::Function(Function {
        name: ObjectName(vec![Ident::new("SUM")]),
        args: vec![FunctionArg::Unnamed(FunctionArgExpr::Expr(*Box::new(expr)))],
        filter: None,
        null_treatment: None,
        over: None,
        distinct: false,
        special: false,
        order_by: vec![],
    })
}

/// Get column from name
///
/// # Panics
///
/// This function will panic if the name cannot be parsed into a valid column expression as valid [Identifier]s.
#[must_use]
pub fn col(name: &str) -> Expr {
    Expr::Identifier(name.into())
}

/// Compute the maximum of an expression
#[must_use]
pub fn max(expr: Expr) -> Expr {
    Expr::Function(Function {
        name: ObjectName(vec![Ident::new("MAX")]),
        args: vec![FunctionArg::Unnamed(FunctionArgExpr::Expr(*Box::new(expr)))],
        filter: None,
        null_treatment: None,
        over: None,
        distinct: false,
        special: false,
        order_by: vec![],
    })
}

/// Construct a new `Expr` A + B
#[must_use]
pub fn add(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        op: BinaryOperator::Plus,
        left: Box::new(left),
        right: Box::new(right),
    }
}

/// Construct a new `Expr` A - B
#[must_use]
pub fn sub(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        op: BinaryOperator::Minus,
        left: Box::new(left),
        right: Box::new(right),
    }
}

/// Get literal from value
pub fn lit<L>(literal: L) -> Expr
where
    L: Into<Literal>,
{
    Expr::from(literal.into())
}

/// Count the amount of non-null entries of an expression
#[must_use]
pub fn count(expr: Expr) -> Expr {
    Expr::Function(Function {
        name: ObjectName(vec![Ident::new("COUNT")]),
        args: vec![FunctionArg::Unnamed(FunctionArgExpr::Expr(*Box::new(expr)))],
        filter: None,
        null_treatment: None,
        over: None,
        distinct: false,
        special: false,
        order_by: vec![],
    })
}

/// Construct a new `Expr` representing A * B
#[must_use]
pub fn mul(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: BinaryOperator::Multiply,
        right: Box::new(right),
    }
}

/// Compute the minimum of an expression
#[must_use]
pub fn min(expr: Expr) -> Expr {
    Expr::Function(Function {
        name: ObjectName(vec![Ident::new("MIN")]),
        args: vec![FunctionArg::Unnamed(FunctionArgExpr::Expr(*Box::new(expr)))],
        filter: None,
        null_treatment: None,
        over: None,
        distinct: false,
        special: false,
        order_by: vec![],
    })
}

/// Construct a new `Expr` for NOT P
#[must_use]
pub fn not(expr: Expr) -> Expr {
    Expr::UnaryOp {
        op: UnaryOperator::Not,
        expr: Box::new(expr),
    }
}

/// Construct a new `Expr` for A >= B
#[must_use]
pub fn ge(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: BinaryOperator::GtEq,
        right: Box::new(right),
    }
}

/// Construct a new `Expr` for A == B
#[must_use]
pub fn equal(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: BinaryOperator::Eq,
        right: Box::new(right),
    }
}

/// Construct a new `Expr` for P OR Q
#[must_use]
pub fn or(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: BinaryOperator::Or,
        right: Box::new(right),
    }
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// /// An expression with an alias e.g. `a + 1 AS b`
// pub struct TestAliasedResultExpr {
//     /// The expression e.g. `a + 1`, `COUNT(*)`, etc.
//     pub expr: Box<Expr>,
//     /// The alias e.g. `count` in `COUNT(*) AS count`
//     pub alias: Ident,
// }

// impl TestAliasedResultExpr {
//     /// Create a new `AliasedResultExpr`
//     #[must_use]
//     pub fn new(expr: Expr, alias: Ident) -> Self {
//         Self {
//             Box::new(expr),
//                         alias,
//         }
//     }

// }

// /// An expression with an alias, i.e., EXPR AS ALIAS
// ///
// /// # Panics
// ///
// /// This function will panic if the `alias` cannot be parsed as a valid [Identifier].
// #[must_use]
// pub fn aliased_expr(expr: Expr, alias: &str) -> AliasedResultExpr {
//     AliasedResultExpr {
//         expr: Box::new(expr),
//         alias: Ident::new(alias),
//     }
// }




aggregateOperator:::

To refactor the given structure `aggregation_exprs: Vec<(AggregationOperator, Expr, Ident)>` into a structure that fully utilizes `Expr`, you need to incorporate the functionality of `AggregationOperator` directly into the `Expr` type. Here's how you can approach this:

---

### Current Structure
```rust
aggregation_exprs: Vec<(AggregationOperator, Expr, Ident)>,
```

- **`AggregationOperator`**: Enum representing the type of aggregation.
- **`Expr`**: Represents the SQL expression being aggregated.
- **`Ident`**: Identifier for the aggregated column.

---

### Refactored Structure
The goal is to replace this structure with:
```rust
aggregation_exprs: Vec<Expr>,
```

---

### How `Expr` Replaces `AggregationOperator`:
In SQL, aggregation functions like `SUM`, `MAX`, `MIN`, etc., are represented as functions. The `Expr::Function` variant can capture this. For example:
```rust
Expr::Function(Function {
    name: ObjectName(vec![Ident::new("SUM")]),
    args: vec![FunctionArg::Unnamed(Expr::Identifier("column_name".into()))],
    filter: None,
    null_treatment: None,
    over: None,
    distinct: false,
    special: false,
    order_by: vec![],
})
```

Here:
- `ObjectName` represents the name of the function (e.g., `SUM`, `MAX`).
- `args` contains the column or expression being aggregated.
- Other fields allow flexibility for advanced SQL features.

---

### Changes to `AggregationOperator`
You no longer need `AggregationOperator` because the function name (e.g., `SUM`, `MAX`) is embedded in the `Expr`.

You can safely **remove** the `AggregationOperator` enum.

---

### Refactored Code Example
Original:
```rust
aggregation_exprs.push((
    AggregationOperator::Sum,
    Expr::Identifier(Ident::new("column_name")),
    Ident::new("result_alias"),
));
```

Refactored:
```rust
aggregation_exprs.push(Expr::Function(Function {
    name: ObjectName(vec![Ident::new("SUM")]), // Replace AggregationOperator::Sum with "SUM"
    args: vec![FunctionArg::Unnamed(Expr::Identifier(Ident::new("column_name")))],
    filter: None,
    null_treatment: None,
    over: None,
    distinct: false,
    special: false,
    order_by: vec![],
}));
```

---

### Updating the Code
1. **Replace `AggregationOperator` Usage**:
   Wherever you use `AggregationOperator`, replace it with corresponding `Expr::Function` objects.

2. **Update Iterations**:
   If you iterate over `aggregation_exprs` to access `AggregationOperator` and `Expr`, adjust to process only `Expr`.

   Example:
   ```rust
   for expr in aggregation_exprs.iter() {
       if let Expr::Function(func) = expr {
           let function_name = func.name.to_string(); // e.g., "SUM"
           let column = &func.args[0];                // Aggregated column
       }
   }
   ```

3. **Use Identifiers from `Expr`**:
   Extract aliases or identifiers directly from the `Expr::Function` structure where necessary.

---

### Updated `aggregation_exprs` Structure
```rust
aggregation_exprs: Vec<Expr>,
```

---

### Benefits
- **Simplified Design**: Avoids parallel representations of aggregation functions.
- **Improved Flexibility**: Fully leverages `Expr`'s capabilities, making it easier to work with complex SQL constructs.
- **Eliminates Redundancy**: Removes `AggregationOperator`, which becomes unnecessary.

Let me know if you need help with specific parts of the refactor!




Srtill left where_by (expression)??

TableExpression?
SetExpression