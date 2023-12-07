![JokerQuery](https://github.com/Rustixir/joker/blob/main/logo.png)

<div align="center">
  <!-- Downloads -->
  <a href="https://crates.io/crates/joker_query">
    <img src="https://img.shields.io/crates/d/joker_query.svg?style=flat-square"
      alt="Download" />
  </a>
</div>


# JokerQuery

The JokerQuery is a cute sql query builder
with JokerQuery can implement most complex queries with sugar syntax

# Features

− (Operator) - fully implemented ( Select, Insert, Update, Delete ) query operations

− (SubQuery) - can use subquery for operators ( IN, EXISTs, <, >, <= >=, Any )
    also can use sub-query for data source for example `select * from (...)`





# Example 


## NewVersion from 1.0.0

## Select 
``` 
    Select::
        cols(vec!["id", "age", "fullname"])
        .distinct()
        .from("customer")
        .inner_join("merchant").on("customer.id", "customer_id")
        .left_join("product").on("customer.id", "customer_id")
        .where_by("age", op::between(10, 25))
        .and("fullname", op::like("full%"))
        .or("fullname", op::not_in(vec!["danyal", "danyalmh", "danyalai"]))
        .group_by(vec!["merchant_id"])
        .having(&Func::count("id"), op::eq(2025))            
        .order_by("fullname")
        .order_by_desc("age")
        .limit(10)
        .offset(5)
        .build();
```


## Examples
- See the complete examples [here](https://github.com/Rustixir/joker_query/tree/main/example).




# Benchmark 
- almost all complex queries run under 5us


# Crates
joker_query = "1.0.0"
