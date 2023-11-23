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

- (Operator) - implemented all select query feature for example (LIKE, IN, JOIN, GROUP, HAVING, ...)

- (SubQuery) - Joker support Exist operator, you can write complete subQuery inside it



# Example 

```
  Select::
      cols(vec!["id, age, fullname"])
      .distinct()
      .from("customer")
      .inner_join("merchant").on("customer.id", "customer_id")
      .left_join("product").on("customer.id", "customer_id")
      .where_by("age", Op::between(10, 25))
      .and("fullname", Op::like("full%"))
      .and("fullname", Op::not_in(vec!["danyal", "danyalmh", "danyalai"]))
      .group_by(vec!["merchant_id"])
      .having(&Func::count("id"), Op::eq(2025000))            
      .order_by("fullname")
      .order_by_desc("age")
      .limit(10)
      .offset(5)
      .build();
        

```


# Benchmark 
- almost all complex queries run under 5us

# Crates
joker_query = "1.0.0"
