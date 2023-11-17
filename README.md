![JokerQuery](https://github.com/Rustixir/joker/blob/main/logo.png)

<div align="center">
  <!-- Downloads -->
  <a href="https://crates.io/crates/joker_query">
    <img src="https://img.shields.io/crates/d/joker_query.svg?style=flat-square"
      alt="Download" />
  </a>
</div>


# JokerQuery

The Joker is a cute sql query builder
with Joker can implement most complex queries with sugar syntax and high performance

# Features

- (Operator) - implemented all select query feature for example (LIKE, IN, JOIN, GROUP, HAVING, ...)

- (SubQuery) - Joker support Exist operator, you can write complete subQuery inside it

- (Prepare Statement) - Joker also support Prepare Statement


# Example 

```
Select::
     cols(vec!["id, age, fullname"])
    .distinct()
    .from("customer")
    .inner_join("merchant").on("customer.id", "customer_id")
    .left_join("product").on("customer.id", "customer_id")
    .where_by("age", Op::Between(10.into(), 25.into()))
    .and("fullname", Op::Like("full%"))
    .and("fullname", Op::NotIn(vec!["danyal", "danyalmh", "danyalai"].into()))
    .group_by(vec!["merchant_id"])
    .having(&func_count_id, Op::None)            
    .order_by("fullname")
    .order_by_desc("age")
    .limit(10)
    .offset(5)
    .build();
```

** Prepare Statement: 

```
let query_ref = 
            Select::
                cols(vec!["id, age, fullname"])
                .distinct()
                .from("customer")
                .inner_join("merchant").on("customer.id", "customer_id")
                .left_join("product").on("customer.id", "customer_id")
                .where_by("age", Op::Between(Value::Param, Value::Param))
                .and("fullname", Op::Like(Value::param_str()))
                .and("fullname", Op::NotIn(vec![Value::Param, Value::Param, Value::Param].into()))
                .group_by(vec!["merchant_id"])
                .having(&func_count_id, Op::None)            
                .order_by("fullname")
                .order_by_desc("age")
                .limit(10)
                .offset(5)
                .prepare();

let query = Select::stmt(&query_ref).unwrap();
let res = query.bind(vec![
            10.into(), 
            25.into(), 
            "full%".into(),
            "danyal".into(),
            "danyalmh".into(),
            "danyalai".into()
         ]).unwrap();

```

# Benchmark 
- almost all complex queries run under 5us

# Crates
joker_query = "0.1.1"
